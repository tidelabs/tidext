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

//! Tidext official keyring.
use subxt::sp_core::sr25519;

#[cfg_attr(feature = "keyring-stronghold", path = "stronghold.rs")]
#[cfg_attr(feature = "keyring-stronghold_ex", path = "stronghold_ex.rs")]
#[cfg_attr(
  not(any(feature = "keyring-stronghold", feature = "keyring-stronghold_ex")),
  path = "dev.rs"
)]
mod keyring_impl;
pub use keyring_impl::*;

/// Tidechain key-pair representation.
pub type KeyPair = sr25519::Pair;
