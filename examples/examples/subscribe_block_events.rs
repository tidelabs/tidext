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
use tidext::{primitives::CurrencyId, tidechain, ClientBuilder, TidefiKeyring};
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

  let client_path = b"client".to_vec();
  // init signer
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Charlie.to_seed(), None)?;
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    //.set_url("ws://127.0.0.1:9944")
    .build()
    .await?;

  let runtime = client.runtime();

  // Subscribe to (in this case, all) blocks.
  // Note: You probably want to use `runtime.blocks().subscribe_finalized()` most of the time.
  let mut block_sub = runtime.blocks().subscribe_all().await?;

  // While this subscription is active, balance transfers are made somewhere:
  tokio::task::spawn({
    let client = client.clone();
    async move {
      let mut transfer_amount = 1_000_000_000_000;

      // Make small balance transfers from Charlie to Bob in a loop:
      loop {
        client
          .transfer_and_wait_for_in_block_success(
            AccountKeyring::Bob.to_account_id(),
            CurrencyId::Tdfy,
            transfer_amount,
          )
          .await
          .expect("Unable to make transfer");

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        transfer_amount += 100_000_000;
      }
    }
  });

  // Get each block as it arrives.
  while let Some(block) = block_sub.next().await {
    let block = block?;

    // Ask for the events for this block.
    let events = block.events().await?;

    let block_hash = block.hash();

    // We can dynamically decode events:
    debug!("  Dynamic event details: {block_hash:?}:");
    for event in events.iter() {
      let event = event?;
      let is_balance_transfer = event
        .as_event::<tidechain::tidefi::events::Transfer>()?
        .is_some();
      let pallet = event.pallet_name();
      let variant = event.variant_name();
      println!("    {pallet}::{variant} (is balance transfer? {is_balance_transfer})");
    }

    // Or we can find the first transfer event, ignoring any others:
    let transfer_event = events.find_first::<tidechain::tidefi::events::Transfer>()?;

    if let Some(ev) = transfer_event {
      debug!("  - Balance transfer success: value: {:?}", ev.amount);
    } else {
      debug!("  - No balance transfer event found in this block");
    }

    // Or we can find all events to the provided `Ev` type
    let transfer_events = events
      .find::<tidechain::tidefi::events::Transfer>()
      .collect::<Vec<_>>();

    for event in transfer_events {
      let event = event?;
      debug!("  - New transfer from {}", event.from_account_id);
    }

    let treasury_deposit_events = events
      .find::<tidechain::treasury::events::Deposit>()
      .collect::<Vec<_>>();

    for event in treasury_deposit_events {
      let event = event?;
      debug!("  - New treasury deposit of {} TDFY", event.value);
    }

    let balances_deposit_events = events
      .find::<tidechain::balances::events::Deposit>()
      .collect::<Vec<_>>();

    for event in balances_deposit_events {
      let event = event?;
      debug!(
        "  - New balance deposit of {} TDFY for {}",
        event.amount, event.who
      );
    }
  }

  Ok(())
}
