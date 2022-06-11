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

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("hex error: {0}")]
  Hex(#[from] hex::FromHexError),
  #[error("scale error: {0}")]
  Scale(#[from] parity_scale_codec::Error),
  #[error("scale error: {0}")]
  ScaleValueDecode(#[from] scale_value::scale::DecodeError),
  #[error("decode error: {0}")]
  DecodeError(String),
  #[error("Failed to decode: expected more data")]
  EarlyEof(&'static str),
  #[error("Failed to decode extrinsics: {0} bytes of the input were not consumed")]
  ExcessBytes(usize),
  #[error("Failed to decode unsupported extrinsic version '{0}'")]
  CannotDecodeExtrinsicVersion(u8),
  #[error("Cannot find call corresponding to extrinsic with pallet index {0} and call index {1}")]
  CannotFindCall(u8, u8),
  #[error("Failed to decode extrinsic: cannot find type ID {0}")]
  CannotFindType(u32),
  #[error("metadata version {0} is not supported")]
  UnsupportedVersion(u32),
  #[error("unexpected type; expecting a Variant type, but got {got}")]
  ExpectedVariantType { got: String },
  #[error("could not find type with ID {0}")]
  TypeNotFound(u32),
}
