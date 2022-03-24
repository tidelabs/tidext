use pyo3::prelude::*;
use tidefi_primitives::{
  CurrencyId as PrimitiveCurrencyId, CurrencyMetadata as PrimitiveCurrencyMetadata,
  SwapType as PrimitiveSwapType,
};
use tidext::NodeHealth as PrimitiveNodeHealth;

#[pyclass]
pub struct NodeHealth {
  /// Number of connected peers
  #[pyo3(get)]
  pub peers: u32,
  /// Is the node syncing
  #[pyo3(get)]
  pub is_syncing: bool,
  /// Should this node have any peers
  ///
  /// Might be false for local chains or when running without discovery.
  #[pyo3(get)]
  pub should_have_peers: bool,
}

impl From<PrimitiveNodeHealth> for NodeHealth {
  fn from(node_health: PrimitiveNodeHealth) -> Self {
    Self {
      peers: node_health.peers as u32,
      is_syncing: node_health.is_syncing,
      should_have_peers: node_health.should_have_peers,
    }
  }
}

pub type CurrencyId = Option<u32>; // None means Tide

pub fn currency_id_from(id: PrimitiveCurrencyId) -> CurrencyId {
  match id {
    PrimitiveCurrencyId::Tide => None,
    PrimitiveCurrencyId::Wrapped(id) => Some(id),
  }
}

pub fn currency_id_into(id: CurrencyId) -> PrimitiveCurrencyId {
  match id {
    None => PrimitiveCurrencyId::Tide,
    Some(id) => PrimitiveCurrencyId::Wrapped(id),
  }
}

#[pyclass]
#[derive(Clone)]
pub struct CurrencyBalance {
  #[pyo3(get)]
  pub available: u128,
  #[pyo3(get)]
  pub reserved: u128,
}

#[pyclass]
#[derive(Clone)]
pub struct CurrencyMetadata {
  /// Currency name.
  #[pyo3(get)]
  pub name: String,
  /// Currency symbol.
  #[pyo3(get)]
  pub symbol: String,
  /// Number of decimals for the currency.
  #[pyo3(get)]
  pub decimals: u32,
  /// Currency is frozen on chain (can't transfer).
  #[pyo3(get)]
  pub is_frozen: bool,
}

impl From<PrimitiveCurrencyMetadata<Vec<u8>>> for CurrencyMetadata {
  fn from(metadata: PrimitiveCurrencyMetadata<Vec<u8>>) -> Self {
    Self {
      name: String::from_utf8_lossy(&metadata.name).into_owned(),
      symbol: String::from_utf8_lossy(&metadata.symbol).into_owned(),
      decimals: metadata.decimals.into(),
      is_frozen: metadata.is_frozen,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[pyclass]
pub struct SwapType {
  value: PrimitiveSwapType,
}

#[pymethods]
impl SwapType {
  fn is_market(&self) -> bool {
    matches!(self.value, PrimitiveSwapType::Market)
  }

  fn is_limit(&self) -> bool {
    matches!(self.value, PrimitiveSwapType::Limit)
  }
}

impl From<PrimitiveSwapType> for SwapType {
  fn from(swap_type: PrimitiveSwapType) -> Self {
    Self { value: swap_type }
  }
}

impl From<SwapType> for PrimitiveSwapType {
  fn from(swap_type: SwapType) -> Self {
    swap_type.value
  }
}
