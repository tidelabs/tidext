use tidext::{ClientBuilder, TidefiKeyring};

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
  let signer = TidefiKeyring::try_from_seed(AccountKeyring::Charlie.to_seed(), None)
    .await?
    .pair_signer();

  // init client
  let client = ClientBuilder::new().set_signer(signer).build().await?;

  let swap_fee = client.swap_fee().await?;

  debug!("current swap fee {:?}", swap_fee);

  Ok(())
}
