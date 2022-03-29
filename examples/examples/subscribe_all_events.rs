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
  // init signer
  let signer = TidefiKeyring::try_from_seed(AccountKeyring::Alice.to_seed(), None)
    .await?
    .pair_signer();
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    //.set_url("ws://127.0.0.1:9944")
    .build()
    .await?;

  let runtime = client.runtime();
  let mut event_sub = runtime.events().subscribe().await?;
  while let Some(events) = event_sub.next().await {
    let events = events?;
    let block_hash = events.block_hash();

    // We can iterate, statically decoding all events if we want:
    debug!("All events in block {:?}:", block_hash);
    debug!("Static event details:");
    for event in events.iter() {
      let event = event?;
      debug!("{:?}", event);
    }

    // Or we can dynamically decode events:
    debug!("Dynamic event details: {:?}", block_hash);
    for event in events.iter_raw() {
      let event = event?;
      let is_asset_transfer = event
        .as_event::<tidechain::tidefi::events::Transfer>()?
        .is_some();
      let pallet = event.pallet;
      let variant = event.variant;
      debug!(
        " {}::{} (is asset transfer? {})",
        pallet, variant, is_asset_transfer
      );
    }

    // Or we can dynamically find the first transfer event, ignoring any others:
    let transfer_event = events.find_first::<tidechain::tidefi::events::Transfer>()?;

    if let Some(ev) = transfer_event {
      debug!("Asset transfer success: value: {:?}", ev.amount);
    } else {
      debug!("No balance transfer event found in this block");
    }
  }

  Ok(())
}
