// Copyright 2021-2022 Semantic Network Ltd.
// This file is part of tidext.

// tidext is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tidechain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidext.  If not, see <http://www.gnu.org/licenses/>.

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
