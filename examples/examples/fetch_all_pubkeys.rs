// Copyright 2021-2023 Semantic Network Ltd.
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

use sp_keyring::AccountKeyring;
use tidext::{primitives::assets, with_runtime, ClientBuilder, TidefiKeyring};

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
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Alice.to_seed(), None)?;
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    .set_url("wss://rpc.staging.tidefi.io:443")
    .build()
    .await?;

  let pubkeys = with_runtime! {
    client,
    current_runtime,
    {
      client
      .runtime()
      .storage()
      .at(None)
      .await?
      .fetch(&current_runtime.storage().quorum().public_keys(assets::BTC))
      .await?
    }
  }
  .unwrap_or_default();

  let members_count = with_runtime! {
    client,
    current_runtime,
    {
      client
      .runtime()
      .storage()
      .at(None)
      .await?
      .fetch(&current_runtime.storage().quorum().counter_for_members())
      .await?
    }
  }
  .unwrap_or_default();

  if pubkeys.len() < members_count as usize {
    error!(
      "Not enough pubkeys available, {} member(s), and {} pubkeys available.",
      members_count,
      pubkeys.len(),
    )
  }

  for (account_id, pubkey) in pubkeys {
    debug!(
      "USDT pubkey for {}: 0x{}",
      account_id,
      hex::encode(&pubkey[..])
    );
  }

  Ok(())
}
