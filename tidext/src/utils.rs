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

use serde::{Deserialize, Serialize};

/// Make RPC call to Tidechain
///
/// # Example
///
/// ```no_run
/// use tidext::{make_rpc_call, primitives::CurrencyId};
///
/// let account_balance = make_rpc_call!(
///   client_instance,
///   "tidefi_getAccountBalance",
///   CurrencyBalance<BalanceInfo>,
///   account_id,
///   CurrencyId::Tifi
/// )?;
/// ```
#[macro_export]
macro_rules! make_rpc_call {
  ($self:ident, $call:literal, $res:ty, $($params: expr),*) => {{
    $self
      .runtime()
      .client
      .rpc()
      .client
      .request::<$res>(&format!("{}", $call), rpc_params![$($params),*])
      .await
  }};
}

/// Query Tidechain storage
///
/// # Example
///
/// ```no_run
/// use tidext::{query_storage, primitives::CurrencyId};
///
/// let tifi_staking_pool = query_storage!(
///   client_instance,
///   tidefi_staking,
///   staking_pool,
///   &CurrencyId::Tifi
/// )?;
/// ```
#[macro_export]
macro_rules! query_storage {
  ($self:ident, $pallet:ident, $item:ident, $($params: expr),*) => {{
    $self.runtime().storage().$pallet().$item($($params),*, None).await
  }};
  ($self:ident, $pallet:ident, $item:ident) => {{
   $self.runtime().storage().$pallet().$item(None).await
 }};
}

/// Get latest Tidechain block
#[macro_export]
macro_rules! latest_block {
  ($self:ident) => {{
    $self
      .runtime()
      .client
      .rpc()
      .block_hash(None)
      .await?
      .unwrap_or_default()
  }};
}

/// Health status of the node the client is currently connected
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeHealth {
  /// Number of connected peers
  pub peers: usize,
  /// Is the node syncing
  pub is_syncing: bool,
  /// Should this node have any peers
  ///
  /// Might be false for local chains or when running without discovery.
  pub should_have_peers: bool,
}
