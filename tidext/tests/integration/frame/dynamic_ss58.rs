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

use crate::{test_context, test_context_tidechain, AccountKeyring};
use tidext::{Error, TidefiKeyring};

#[cfg(feature = "tidechain-native")]
#[async_std::test]
async fn ss58_tidechain() -> Result<(), Error> {
  let client_path = b"client_path".to_vec();
  let cxt = test_context_tidechain().await;
  let alice =
    TidefiKeyring::try_from_seed(client_path.clone(), AccountKeyring::Alice.to_seed(), None)?;
  let mut client = cxt.node_proc.client().clone();
  client.set_signer(Some(alice));

  assert_eq!(client.ss58_prefix(), 7007);
  assert_eq!(
    client.account_id_ss58().unwrap(),
    "fhEVxe5tBqLSDCpztuuScgqkpnuS1KecNwsB1ZL4U6xkho6cS".to_string()
  );

  Ok(())
}

#[cfg(feature = "lagoon-native")]
#[async_std::test]
async fn ss58_lagoon() -> Result<(), Error> {
  let client_path = b"client_path".to_vec();
  let cxt = test_context().await;
  let alice =
    TidefiKeyring::try_from_seed(client_path.clone(), AccountKeyring::Alice.to_seed(), None)?;
  let mut client = cxt.node_proc.client().clone();
  client.set_signer(Some(alice));

  assert_eq!(client.ss58_prefix(), 42);
  assert_eq!(
    client.account_id_ss58().unwrap(),
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()
  );

  Ok(())
}
