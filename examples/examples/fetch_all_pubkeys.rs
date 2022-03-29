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
  // init signer
  let signer = TidefiKeyring::try_from_seed(AccountKeyring::Alice.to_seed(), None)
    .await?
    .pair_signer();
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
