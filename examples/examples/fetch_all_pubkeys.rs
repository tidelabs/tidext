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

use sp_keyring::AccountKeyring;
use tidext::{primitives::assets, ClientBuilder, TidefiKeyring};

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
  let signer =
    TidefiKeyring::try_from_seed(client_path, AccountKeyring::Alice.to_seed(), None).await?;
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    .set_url("ws://127.0.0.1:9944")
    .build()
    .await?;

  let runtime = client.runtime();

  let pubkeys = runtime
    .storage()
    .quorum()
    .public_keys(&assets::USDT, None)
    .await?;

  let members_count = runtime.storage().quorum().counter_for_members(None).await?;
  if pubkeys.len() < members_count as usize {
    error!(
      "Not enough pubkeys available, {} member(s), and {} pubkeys available.",
      members_count,
      pubkeys.len(),
    )
  }

  for (account_id, pubkey) in pubkeys {
    debug!(
      "USDT pubkey for {}: {}",
      account_id,
      String::from_utf8_lossy(&pubkey)
    );
  }

  Ok(())
}
