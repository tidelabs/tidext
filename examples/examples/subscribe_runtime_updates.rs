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

use std::time::Duration;
use tidext::{primitives::CurrencyId, ClientBuilder, TidefiKeyring};

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
  let signer =
    TidefiKeyring::try_from_seed(client_path, AccountKeyring::Charlie.to_seed(), None).await?;

  // init client
  let client = ClientBuilder::new().set_signer(signer).build().await?;

  // Start a new tokio task to watch the runtime updates while
  // utilizing the API for other use cases.
  let update_client = client.runtime().client.updates();
  tokio::spawn(async move {
    let result = update_client.perform_runtime_updates().await;
    if let Err(err) = result {
      error!("Runtime update error={:?}", err);
    }
  });

  // Make small balance transfers from Charlie to Bob:
  for _ in 0..10 {
    client
      .transfer(
        AccountKeyring::Bob.to_account_id().into(),
        CurrencyId::Tdfy,
        1_000_000_000_000,
      )
      .await?;

    tokio::time::sleep(Duration::from_secs(30)).await;
  }

  Ok(())
}
