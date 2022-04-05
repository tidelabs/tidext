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

use crate::{test_context, AccountKeyring};
use frame_support::assert_ok;
use tidext::{primitives::assets::Asset, Error, Signer, TidefiKeyring};

#[async_std::test]
async fn tx_basic_transfer() -> Result<(), Error> {
  // 10 tides
  let transfer_amount = Asset::Tide.saturating_mul(10);

  let alice = TidefiKeyring::try_from_seed(AccountKeyring::Alice.to_seed(), None).await?;
  let charlie = TidefiKeyring::try_from_seed(AccountKeyring::Charlie.to_seed(), None).await?;

  let cxt = test_context().await;
  let mut client = cxt.node_proc.client().clone();

  client.set_signer(charlie.clone());
  assert_eq!(client.signer.account_id(), charlie.account_id());

  let alice_pre = client
    .balance(alice.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  let charlie_pre = client
    .balance(charlie.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  assert_ok!(
    client
      .transfer_and_wait_for_finalized_success(
        alice.account_id().clone(),
        Asset::Tide.currency_id(),
        transfer_amount,
      )
      .await
  );

  let alice_post = client
    .balance(alice.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  let charlie_post = client
    .balance(charlie.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  assert!(charlie_pre - transfer_amount >= charlie_post);
  // initial deposit is 1 TIDE
  assert_eq!(alice_pre + transfer_amount - 1_000_000_000_000, alice_post);
  Ok(())
}
