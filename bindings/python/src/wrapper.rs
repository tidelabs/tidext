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

use pyo3::{exceptions::PyRuntimeError, prelude::*};
use tidefi_primitives::{
  CurrencyId as PrimitiveCurrencyId, CurrencyMetadata as PrimitiveCurrencyMetadata, Hash,
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

pub type CurrencyId = Option<u32>; // None means Tdfy

pub fn currency_id_from(id: PrimitiveCurrencyId) -> CurrencyId {
  match id {
    PrimitiveCurrencyId::Tdfy => None,
    PrimitiveCurrencyId::Wrapped(id) => Some(id),
  }
}

pub fn currency_id_into(id: CurrencyId) -> PrimitiveCurrencyId {
  match id {
    None => PrimitiveCurrencyId::Tdfy,
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

pub fn to_hash(hex: String) -> PyResult<Hash> {
  let hex = if let Some(hex) = hex.strip_prefix("0x") {
    hex
  } else {
    hex.as_str()
  };
  let b = hex::decode(hex)
    .map_err(|e| PyRuntimeError::new_err(format!("invalid hash, must be a hex string: {e}")))?;

  let hash: [u8; 32] = b
    .try_into()
    .map_err(|_| PyRuntimeError::new_err("hash must represent 32 bytes"))?;

  Ok(sp_core::H256(hash))
}

pub fn hash_to_string(hash: Hash) -> String {
  hex::encode(hash.0)
}
