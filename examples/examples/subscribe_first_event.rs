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
use tidext::{tidechain, ClientBuilder, TidefiKeyring};

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
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Alice.to_seed(), None)?;
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    //.set_url("ws://127.0.0.1:9944")
    .build()
    .await?;

  let mut blocks_sub = client.runtime().blocks().subscribe_all().await?;
  while let Some(block) = blocks_sub.next().await {
    let block = block?;

    let block_number = block.header().number;
    let block_hash = block.hash();
    // Ask for the events for this block.
    let events = block.events().await?;

    debug!("Block #{block_number}:");
    debug!("Hash: {block_hash}");

    // Iterate through the events using metadata to dynamically decode and skip them,
    // and return the first event found which decodes to the provided Ev type.
    let update_block_event =
      events.find_first::<tidechain::security::events::UpdateCurrentBlock>()?;

    if let Some(event) = update_block_event {
      debug!("Update block event found {:?}", event.0)
    }
  }

  Ok(())
}
