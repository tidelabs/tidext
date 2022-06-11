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

use crate::{Error, Result};
use parity_scale_codec::Decode;
use scale_value::Value;
use sp_core::crypto::{AccountId32, Ss58Codec};
use std::{boxed::Box, collections::HashMap};
use tidefi_primitives::CurrencyId as TidefiCurrencyId;

/// Provides custom decoding for predefined environment types.
#[derive(Default)]
pub struct EnvTypesTranscoder {
  decoders: HashMap<u32, Box<dyn CustomTypeDecoder>>,
}

impl EnvTypesTranscoder {
  /// Construct an `EnvTypesTranscoder` from the given type registry.
  pub fn new(decoders: HashMap<u32, Box<dyn CustomTypeDecoder>>) -> Self {
    Self { decoders }
  }

  /// If the given type lookup id is for an environment type with custom
  /// decoding, decodes the given input with the custom decoder and returns
  /// `Some(value)`. Otherwise returns `None`.
  ///
  /// # Errors
  ///
  /// - If the custom decoding fails.
  pub fn try_decode(&self, type_id: u32, input: &mut &[u8]) -> Result<Option<Value>> {
    match self.decoders.get(&type_id) {
      Some(decoder) => {
        log::debug!("Decoding type {:?} with custom decoder", type_id);
        let decoded = decoder.decode_value(input)?;
        Ok(Some(decoded))
      }
      None => {
        log::debug!("No custom decoder found for type {:?}", type_id);
        Ok(None)
      }
    }
  }
}

/// Implement this trait to define custom decoding for a type in a `scale-info` type registry.
pub trait CustomTypeDecoder {
  fn decode_value(&self, input: &mut &[u8]) -> Result<Value>;
}

/// Custom encoding/decoding for the Substrate `AccountId` type.
///
/// Enables an `AccountId` to be input/ouput as an SS58 Encoded literal e.g.
/// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[derive(Clone)]
pub struct AccountId;

impl CustomTypeDecoder for AccountId {
  fn decode_value(&self, input: &mut &[u8]) -> Result<Value> {
    let account_id = AccountId32::decode(input)?;
    Ok(Value::string(account_id.to_ss58check()))
  }
}

/// Custom decoding for the `Hash` or `[u8; 32]` type so that it is displayed as a hex encoded
/// string.
pub struct Hash;

impl CustomTypeDecoder for Hash {
  fn decode_value(&self, input: &mut &[u8]) -> Result<Value> {
    let hash = sp_core::H256::decode(input)?.to_string();
    Ok(Value::string(hash))
  }
}

/// Custom decoding for the `CurrencyId` type so that it is displayed as a string.
pub struct CurrencyId;

impl CustomTypeDecoder for CurrencyId {
  fn decode_value(&self, input: &mut &[u8]) -> Result<Value> {
    let currency = TidefiCurrencyId::decode(input)?;
    let asset: tidefi_primitives::assets::Asset = currency
      .try_into()
      .map_err(|e: &str| Error::DecodeError(e.to_string()))?;

    Ok(Value::string(asset.symbol()))
  }
}
