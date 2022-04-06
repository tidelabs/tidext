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

use tidext::{
  primitives::{assets, CurrencyId, SwapType},
  ClientBuilder, Permill, TidechainCall, TidefiCall, TidefiKeyring, TidefiStakingCall,
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
  // init signer
  let signer = TidefiKeyring::try_from_seed(AccountKeyring::Charlie.to_seed(), None).await?;
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    //.set_url("ws://127.0.0.1:9944")
    .build()
    .await?;

  debug!("submitting batch calls...");
  client
    .submit_batch(vec![
      TidechainCall::Tidefi(TidefiCall::swap {
        currency_id_from: CurrencyId::Tifi,
        amount_from: 1_000_000_000_000,
        currency_id_to: CurrencyId::Wrapped(assets::USDT),
        amount_to: 1_000_000_000,
        swap_type: SwapType::Limit,
        slippage_tolerance: None,
      }),
      TidechainCall::Tidefi(TidefiCall::swap {
        currency_id_from: CurrencyId::Tifi,
        amount_from: 2_000_000_000_000,
        currency_id_to: CurrencyId::Wrapped(assets::USDT),
        amount_to: 2_000_000_000,
        swap_type: SwapType::Market,
        // 0.1%
        slippage_tolerance: Some(Permill::from_rational(1_u32, 1000_u32)),
      }),
      TidechainCall::TidefiStaking(TidefiStakingCall::stake {
        currency_id: CurrencyId::Tifi,
        amount: 100_000_000_000_000,
        duration: 150,
      }),
    ])
    .await?;

  Ok(())
}
