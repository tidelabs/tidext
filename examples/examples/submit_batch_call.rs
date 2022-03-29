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
  let signer = TidefiKeyring::try_from_seed(AccountKeyring::Charlie.to_seed(), None)
    .await?
    .pair_signer();
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
        currency_id_from: CurrencyId::Tide,
        amount_from: 1_000_000_000_000,
        currency_id_to: CurrencyId::Wrapped(assets::USDT),
        amount_to: 1_000_000_000,
        swap_type: SwapType::Limit,
        slippage_tolerance: None,
      }),
      TidechainCall::Tidefi(TidefiCall::swap {
        currency_id_from: CurrencyId::Tide,
        amount_from: 2_000_000_000_000,
        currency_id_to: CurrencyId::Wrapped(assets::USDT),
        amount_to: 2_000_000_000,
        swap_type: SwapType::Market,
        // 0.1%
        slippage_tolerance: Some(Permill::from_rational(1_u32, 1000_u32)),
      }),
      TidechainCall::TidefiStaking(TidefiStakingCall::stake {
        currency_id: CurrencyId::Tide,
        amount: 100_000_000_000_000,
        duration: 150,
      }),
    ])
    .await?;

  Ok(())
}
