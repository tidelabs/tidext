use tidext::{ClientBuilder, CurrencyId, SwapType};

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
  let signer = helpers::init_signer(AccountKeyring::Charlie.to_seed()).await;
  // init client
  let client = ClientBuilder::new()
    .set_signer(signer)
    //.set_url("ws://dedevtidesubstrate-a.semantic-network.tech:9944")
    .build()
    .await?;

  let stakes = client
    .stakes(&AccountKeyring::Charlie.to_account_id(), None)
    .await?;
  debug!("stakes: {:?}", stakes);
  let stakes = client.total_stake_for(CurrencyId::Wrapped(2)).await?;
  debug!("all btc stakes: {:?}", stakes);

  let system_health = client.system_health().await?;
  debug!("system health: {:?}", system_health);

  let stakes = client.total_stake_for(CurrencyId::Tide).await?;
  debug!("all tide stakes: {:?}", stakes);

  let stakes = client
    .stakes(
      &AccountKeyring::Ferdie.to_account_id(),
      Some(CurrencyId::Tide),
    )
    .await?;
  debug!("Ferdie tide stake: {:?}", stakes);

  let balance = client
    .balances(&AccountKeyring::Charlie.to_account_id())
    .await?;

  debug!("Charlie ALLS balance: {:?}", balance);

  let extrinsic = client
    .swap_extrinsic(
      CurrencyId::Tide,
      1_000_000_000_000,
      CurrencyId::Wrapped(2),
      100_000_000,
      SwapType::Limit,
      None,
    )
    .await?;

  debug!("Extrinsic: {:?}", extrinsic);

  let swap_fee = client.get_regular_swap_fee().await?;
  let mm_swap_fee = client.get_market_maker_swap_fee().await?;
  debug!("swap_fee: {}", swap_fee);
  debug!("mm_swap_fee: {}", mm_swap_fee);

  let balance = client
    .balance(&AccountKeyring::Charlie.to_account_id(), CurrencyId::Tide)
    .await?;

  debug!("Charlie TIDE balance: {:?}", balance);

  let balance = client
    .balances(&AccountKeyring::Charlie.to_account_id())
    .await?;
  debug!("Charlie ALLS balance: {:?}", balance);

  Ok(())
}
