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
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Charlie.to_seed(), None)?;

  // init client
  let client = ClientBuilder::new()
    .set_url("wss://rpc.lagoon.tidefi.io:443")
    .set_signer(signer)
    .build()
    .await?;

  let extrinsic = client
    .transfer_extrinsic(
      AccountKeyring::Charlie.to_account_id(),
      CurrencyId::Tdfy,
      1_000_000_000_000,
    )
    .await?;

  debug!("current extrinsic {:?}", extrinsic);

  let extrinsic_bytes = hex::decode(extrinsic.strip_prefix("0x").unwrap()).unwrap();
  let extrinsic_cursor = &mut &*extrinsic_bytes;

  let decoded_extrinsic = client.decode_extrinsic(extrinsic_cursor).await?;
  debug!("decoded_extrinsic {:?}", decoded_extrinsic);

  Ok(())
}
