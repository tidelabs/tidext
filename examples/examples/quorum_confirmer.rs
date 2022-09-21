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

use std::{thread::sleep, time::Duration};
use tidefi_primitives::{assets::Asset, ProposalType};
use tidext::{ClientBuilder, QuorumCall, QuorumExt, TidechainCall, TidefiKeyring};
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

  let client_path = b"client".to_vec();

  // init signer
  let signer = TidefiKeyring::try_from_seed(client_path, AccountKeyring::Charlie.to_seed(), None)?;
  // init client
  let client = ClientBuilder::new()
    // set main signer (need to use stronghold)
    .set_signer(signer)
    //.set_url("ws://127.0.0.1:9944")
    .build()
    .await?;

  // we should probably set keys on each start (it delete all the old-ones)
  if client
    .is_quorum_member(client.account_id().unwrap())
    .await?
  {
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
            all_calls.push(TidechainCall::Quorum(QuorumCall::RejectProposal {
              proposal: proposal_id,
            }))
          }
          ProposalType::Withdrawal(_withdrawal_details) => {
            all_calls.push(TidechainCall::Quorum(QuorumCall::AcknowledgeProposal {
              proposal: proposal_id,
            }))
          }
          ProposalType::UpdateConfiguration(_members, _threshold) => {
            all_calls.push(TidechainCall::Quorum(QuorumCall::AcknowledgeProposal {
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
