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
///   CurrencyId::Tide
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
/// let tide_staking_pool = query_storage!(
///   client_instance,
///   tidefi_staking,
///   staking_pool,
///   &CurrencyId::Tide
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

/// Get latest tidechain block
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

/// Health status returned by the RPC
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
