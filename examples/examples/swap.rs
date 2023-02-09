// Copyright 2021-2022 Semantic Network Ltd.
// This file is part of tidext.

// tidext is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// tidext is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidext.  If not, see <http://www.gnu.org/licenses/>.

use futures::StreamExt;
use std::str::FromStr;
use tidefi_primitives::{assets::Asset, AccountId, Hash, SwapType};
use tidext::{
  tidechain, AssetsCall, BalancesCall, ClientBuilder, Permill, TidechainCall, TidefiKeyring,
};

// load sr25519 test account
use sp_keyring::AccountKeyring;

#[path = "../src/lib.rs"]
mod helpers;

// logging
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // init logger
  helpers::init_logger()?;

  // mint tokens with Ferdie (sudo)
  let client_path = b"ferdie".to_vec();
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Ferdie.to_seed(), None)?;
  let client = ClientBuilder::new().set_signer(signer).build().await?;

  let previous_charlie_balance = client
    .balance(
      &AccountKeyring::Charlie.to_account_id(),
      Asset::Bitcoin.currency_id(),
    )
    .await?
    .available;

  // mint 10 BTC to Charlie
  with_tidext_runtime! {
    client,
    current_runtime,
    {
      client
      .runtime()
      .tx()
      .sign_and_submit_then_watch_default(
        &current_runtime
          .tx()
          .sudo()
          .sudo_as(
            // Assets pallet account ID
            AccountId::from_str("fhCBCt9jfV4TFbUWy71WxxQgDnwn2eLxkPZMJK2Xz4QRJrcbx").unwrap().into(),
            // mint call
            TidechainCall::Assets(AssetsCall::Mint {
              id: 2,
              beneficiary: AccountKeyring::Charlie.to_account_id(),
              amount: Asset::Bitcoin.saturating_mul(10)
            }).into()
          ), client.signer()?)
      .await?
    }
  }
  .wait_for_in_block()
  .await?;

  // mint 100k TDFY to Dave
  with_tidext_runtime! {
    client,
    current_runtime,
    {
      client
      .runtime()
      .tx()
      .sign_and_submit_then_watch_default(
        &current_runtime
          .tx()
          .sudo()
          .sudo(
            // set balance call
            TidechainCall::Balances(BalancesCall::SetBalance {
              who: AccountKeyring::Dave.to_account_id(),
              new_free: Asset::Tdfy.saturating_mul(200_000),
              new_reserved: 0,
            }).into()
          ), client.signer()?)
      .await?
    }
  }
  .wait_for_in_block()
  .await?;

  // init charlie signer (limit order)
  let client_path = b"charlie".to_vec();
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Charlie.to_seed(), None)?;
  // init client
  let client = ClientBuilder::new().set_signer(signer).build().await?;

  // validate Charlie balance
  assert_eq!(
    client
      .balance(client.account_id().unwrap(), Asset::Bitcoin.currency_id())
      .await?
      .available,
    previous_charlie_balance.saturating_add(Asset::Bitcoin.saturating_mul(10)),
  );

  // spawn swap
  let mut swap_id: Option<Hash> = None;
  let mut mm_swap_id: Option<Hash> = None;

  let client_isolated = client.clone();
  tokio::spawn(async move {
    let runtime = client.runtime();
    // Create our subscription and filter only the events we needs
    let mut all_events_to_subscribe = runtime
      .events()
      .subscribe()
      .await
      .unwrap()
      .filter_events::<(
        tidechain::tidefi::events::Swap,
        tidechain::oracle::events::SwapProcessed,
      )>();

    while let Some(ev) = all_events_to_subscribe.next().await {
      let event_details = ev.unwrap();

      let block_hash = event_details.block_hash;
      let event = event_details.event;
      debug!("Event at {:?}:", block_hash);

      if let (Some(swap), _) = &event {
        if swap.swap_type == SwapType::Limit {
          mm_swap_id = Some(swap.request_id);
        }
        if swap.swap_type == SwapType::Market {
          swap_id = Some(swap.request_id);
        }
        // try to match each swap together
        if mm_swap_id.is_some() && swap_id.is_some() {
          with_tidext_runtime! {
            client,
            current_runtime,
            {

              // grab the limit order from the chain
              let limit_order = client
                .runtime()
                .storage()
                .fetch(&current_runtime.storage().oracle().swaps(mm_swap_id.unwrap()), None)
                .await
                .unwrap()
                .unwrap();

              // grab the market order from the chain
              let market_order = client
                .runtime()
                .storage()
                .fetch(&current_runtime.storage().oracle().swaps(swap_id.unwrap()), None)
                .await
                .unwrap()
                .unwrap();

              // test the slippage validation
              assert_eq!(
                market_order.validate_slippage(
                  &limit_order,
                  market_order.amount_from,
                  market_order.amount_to.saturating_sub(1_000)
                ),
                Ok(())
              );

              println!("swap works as expected");
              std::process::exit(0);
            }
          };
        }
      }
      if let (_, Some(processed)) = &event {
        debug!("swap processed event: {:?}", processed);
      }
    }
  });

  // create limit order
  // 5 BTC -> 100 000 TDFY
  client_isolated
    .swap_and_wait_for_in_block_success(
      Asset::Bitcoin.currency_id(),
      Asset::Bitcoin.saturating_mul(5),
      Asset::Tdfy.currency_id(),
      Asset::Tdfy.saturating_mul(100_000),
      SwapType::Limit,
      // 2% slippage
      Some(Permill::from_rational(2_u128, 100_u128)),
    )
    .await?;

  // create market order
  // 100 000 TDFY -> 5 TDFY
  let dave_client_path = b"dave".to_vec();
  let dave_signer =
    TidefiKeyring::try_from_seed(dave_client_path, AccountKeyring::Dave.to_seed(), None)?;
  // init client
  let dave_client = ClientBuilder::new().set_signer(dave_signer).build().await?;

  dave_client
    .swap_and_wait_for_in_block_success(
      Asset::Tdfy.currency_id(),
      Asset::Tdfy.saturating_mul(60_000),
      Asset::Bitcoin.currency_id(),
      Asset::Bitcoin.saturating_mul(3),
      SwapType::Market,
      // 1% slippage
      Some(Permill::from_rational(1_u128, 100_u128)),
    )
    .await?;

  // wait 60 seconds before force-closing the process
  std::thread::sleep(std::time::Duration::from_secs(60));
  std::process::exit(1);
}
