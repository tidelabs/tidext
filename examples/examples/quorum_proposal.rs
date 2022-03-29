use sp_keyring::AccountKeyring;
use tidext::{
  primitives::{assets, ComplianceLevel, CurrencyId, Mint, ProposalType},
  ClientBuilder, QuorumExt, TidefiKeyring,
};

#[path = "../src/lib.rs"]
mod helpers;

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
    // set main signer (need to use stronghold)
    .set_signer(signer)
    .build()
    .await?;

  client
    .submit_proposal(ProposalType::Mint(Mint {
      account_id: AccountKeyring::Alice.to_account_id(),
      currency_id: CurrencyId::Wrapped(assets::USDT),
      mint_amount: 10010,
      transaction_id: Vec::new(),
      compliance_level: ComplianceLevel::Green,
    }))
    .await?;

  Ok(())
}
