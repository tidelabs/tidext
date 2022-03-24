use std::{thread::sleep, time::Duration};
use tidefi_primitives::{assets::Asset, ProposalType};
use tidext::{ClientBuilder, QuorumCall, QuorumExt, TidechainCall};
// load sr25519 test account
use sp_keyring::AccountKeyring;
use strum::IntoEnumIterator;

// logging
#[macro_use]
extern crate log;

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

  // we should probably set keys on each start (it delete all the old-ones)
  if client.is_quorum_member(&client.get_account_id()).await? {
    let all_assets_pubkey = Asset::iter()
      .map(|asset| (asset.id(), "pubkey".as_bytes().to_vec()))
      .collect();

    trace!("submitting pubkeys {:?}", all_assets_pubkey);
    client.submit_public_keys(all_assets_pubkey).await?;
    trace!("public keys submitted and updated successfully");
    debug!("quorum member ready!");
  }

  // spawn listener
  let client_isolated = client.clone();
  tokio::spawn(async move {
    loop {
      let proposals = client_isolated.get_proposals().await.unwrap();

      let mut all_calls: Vec<TidechainCall> = Vec::new();
      for (proposal_id, _initial_block, proposal_type) in proposals {
        match proposal_type {
          ProposalType::Mint(_mint_details) => {
            all_calls.push(TidechainCall::Quorum(QuorumCall::reject_proposal {
              proposal: proposal_id,
            }))
          }
          ProposalType::Withdrawal(_withdrawal_details) => {
            all_calls.push(TidechainCall::Quorum(QuorumCall::acknowledge_proposal {
              proposal: proposal_id,
            }))
          }
          ProposalType::UpdateConfiguration(_members, _threshold) => {
            all_calls.push(TidechainCall::Quorum(QuorumCall::acknowledge_proposal {
              proposal: proposal_id,
            }))
          }
        }
      }

      // submitting proposal in batch
      if !all_calls.is_empty() {
        debug!("submitting batch call of {} items", all_calls.len());
        client_isolated.submit_batch(all_calls).await.unwrap();
      }

      // wait minimum 1 block
      sleep(Duration::from_secs(12));
    }
  });

  let eve_balance = client
    .balances(&AccountKeyring::Eve.to_account_id())
    .await?;

  debug!("Eve balance: {:?}", eve_balance);
  // you could also use the loop {} directly to block the process...
  sleep(Duration::from_secs(600));

  Ok(())
}

//
