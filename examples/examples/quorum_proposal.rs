use sp_keyring::AccountKeyring;
use tidefi_primitives::{ComplianceLevel, Mint, ProposalType};
use tidext::{assets, ClientBuilder, CurrencyId, QuorumExt};

#[path = "../src/lib.rs"]
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // init logger
  helpers::init_logger()?;
  // init signer
  let signer = helpers::init_signer(AccountKeyring::Charlie.to_seed()).await;
  // init client
  let client = ClientBuilder::new()
    // set main signer (need to use stronghold)
    .set_signer(signer)
    //.set_url("ws://dedevtidesubstrate-a.semantic-network.tech:9944")
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

  //client.confirm_withdrawal(Hash::zero()).await?;

  Ok(())
}
