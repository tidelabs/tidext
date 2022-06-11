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

use env_types::CustomTypeDecoder;
use parity_scale_codec::{Compact, Decode};
use scale_info::{form::PortableForm, IntoPortable, TypeInfo};
use scale_value::Value;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap, fmt::Debug};

use scale_info::Path;
use sp_runtime::{AccountId32, MultiAddress, MultiSignature};

pub type Result<T> = core::result::Result<T, error::Error>;
use crate::env_types::EnvTypesTranscoder;
use metadata::Metadata;

pub use error::Error;
mod env_types;
mod error;
mod metadata;
mod util;

type TypesByPath = HashMap<PathKey, u32>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PathKey(Vec<String>);

impl PathKey {
  pub fn from_type<T>() -> Self
  where
    T: TypeInfo,
  {
    let type_info = T::type_info();
    let path = type_info
      .path()
      .clone()
      .into_portable(&mut Default::default());
    PathKey::from(&path)
  }
}

impl From<&Path<PortableForm>> for PathKey {
  fn from(path: &Path<PortableForm>) -> Self {
    PathKey(path.segments().to_vec())
  }
}

/// The result of successfully decoding an extrinsic.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Extrinsic<'a> {
  /// Decoded call data and associated type information about the call.
  #[serde(borrow)]
  pub call_data: CallData<'a>,
  /// The signature and signed extensions (if any) associated with the extrinsic
  #[serde(borrow)]
  pub signature: Option<ExtrinsicSignature<'a>>,
}

impl<'a> Extrinsic<'a> {
  pub fn to_json(self) -> Result<serde_json::Value> {
    serde_json::to_value(self).map_err(Into::into)
  }

  pub fn into_owned(self) -> Extrinsic<'static> {
    Extrinsic {
      call_data: self.call_data.into_owned(),
      signature: self.signature.map(|s| s.into_owned()),
    }
  }
}

/// Decoded call data and associated type information.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallData<'a> {
  /// The name of the pallet
  #[serde(borrow)]
  pub pallet_name: Cow<'a, str>,
  /// The type information for this call (including the name
  /// of the call and information about each argument)
  pub ty: Cow<'a, scale_info::Variant<scale_info::form::PortableForm>>,
  /// The decoded argument data
  pub arguments: Vec<Value>,
}

impl<'a> CallData<'a> {
  pub fn into_owned(self) -> CallData<'static> {
    CallData {
      pallet_name: Cow::Owned(self.pallet_name.into_owned()),
      ty: Cow::Owned(self.ty.into_owned()),
      arguments: self.arguments,
    }
  }
}

/// The signature information embedded in an extrinsic.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExtrinsicSignature<'a> {
  /// Address the extrinsic is being sent from
  #[serde(with = "util::RemoteAddress")]
  pub address: MultiAddress<AccountId32, u32>,
  /// Signature to prove validity
  pub signature: MultiSignature,
  /// Signed extensions, which can vary by node. Here, we
  /// return the name and value of each.
  #[serde(borrow)]
  pub extensions: Vec<(Cow<'a, str>, Value)>,
}

impl<'a> ExtrinsicSignature<'a> {
  pub fn into_owned(self) -> ExtrinsicSignature<'static> {
    ExtrinsicSignature {
      address: self.address,
      signature: self.signature,
      extensions: self
        .extensions
        .into_iter()
        .map(|(k, v)| (Cow::Owned(k.into_owned()), v))
        .collect(),
    }
  }
}

/// The [`scale_info`] type ID as used throughout this library.
type ScaleInfoTypeId = scale_info::interner::UntrackedSymbol<std::any::TypeId>; // equivalent to: <scale_info::form::PortableForm as scale_info::form::Form>::Type;

/// A re-export of [`scale_info::Type`] as used throughout this library.
pub type Type = scale_info::Type<scale_info::form::PortableForm>;
pub type TypeId = u32;

pub struct DecoderBuilder<'a> {
  metadata: &'a frame_metadata::RuntimeMetadataV14,
  types_by_path: TypesByPath,
  decoders: HashMap<u32, Box<dyn CustomTypeDecoder>>,
}

impl<'a> DecoderBuilder<'a> {
  pub fn new(metadata: &'a frame_metadata::RuntimeMetadataV14) -> Self {
    let registry = &metadata.types;
    let types_by_path = registry
      .types()
      .iter()
      .map(|ty| (PathKey::from(ty.ty().path()), ty.id()))
      .collect::<TypesByPath>();
    Self {
      metadata,
      types_by_path,
      decoders: HashMap::new(),
    }
  }

  pub fn with_default_custom_type_decodes(self) -> Self {
    self
      .register_custom_type_decoder::<tidefi_primitives::AccountId, _>(env_types::AccountId)
      .register_custom_type_decoder::<tidefi_primitives::Hash, _>(env_types::Hash)
      .register_custom_type_decoder::<tidefi_primitives::CurrencyId, _>(env_types::CurrencyId)
  }

  pub fn register_custom_type_decoder<T, U>(self, encoder: U) -> Self
  where
    T: TypeInfo + 'static,
    U: CustomTypeDecoder + 'static,
  {
    let mut this = self;

    let path_key = PathKey::from_type::<T>();
    let type_id = this.types_by_path.get(&path_key);

    match type_id {
      Some(type_id) => {
        let existing = this.decoders.insert(*type_id, Box::new(encoder));
        log::debug!("Registered custom decoder for type `{:?}`", type_id);
        if existing.is_some() {
          panic!(
            "Attempted to register decoder with existing type id {:?}",
            type_id
          );
        }
      }
      None => {
        // if the type is not present in the registry, it just means it has not been used.
        log::info!("No matching type in registry for path {:?}.", path_key);
      }
    }
    this
  }

  pub fn build(self) -> Result<Decoder<'a>> {
    let env_types_transcoder = EnvTypesTranscoder::new(self.decoders);
    let internal_metadata = Metadata::from_runtime_metadata(frame_metadata::RuntimeMetadata::V14(
      self.metadata.to_owned(),
    ))?;

    Ok(Decoder::new(
      self.metadata,
      internal_metadata,
      env_types_transcoder,
    ))
  }
}

pub struct Decoder<'a> {
  runtime_metadata: &'a frame_metadata::RuntimeMetadataV14,
  internal_metadata: Metadata,
  env_types: EnvTypesTranscoder,
}

impl<'a> Decoder<'a> {
  pub fn new(
    metadata: &'a frame_metadata::RuntimeMetadataV14,
    internal_metadata: Metadata,
    env_types: EnvTypesTranscoder,
  ) -> Self {
    Self {
      runtime_metadata: metadata,
      internal_metadata,
      env_types,
    }
  }

  pub fn decode_extrinsic(&'a self, data: &mut &[u8]) -> Result<Extrinsic<'a>> {
    if data.is_empty() {
      return Err(Error::DecodeError(
        "unwrapped extrinsic byte length should be > 0".into(),
      ));
    }

    // Ignore the expected extrinsic length here at the moment, since we know it's 1
    let _len = <Compact<u32>>::decode(data)?;

    // V4 extrinsics (the format we can decode here) are laid out roughly as follows:
    //
    // first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
    //
    // signature, which is made up of (in order):
    // - sp_runtime::MultiAddress enum (sender)
    // - sp_runtime::MultiSignature enum
    // - For polkadot, these extensions (but can vary by chain, so we decode generically):
    //   - sp_runtime::generic::Era enum
    //   - compact encoded u32 (nonce; prior transaction count)
    //   - compact encoded u128 (tip paid to block producer/treasury)
    //
    // call, which is made up roughly of:
    // - u8 enum pallet index (for pallets variant)
    // - u8 call index (for inner variant)
    // - call args (types can be pulled from metadata for each arg we expect)
    //
    // So, we start by getting the version/signed from the first byte and go from there.
    let is_signed = data[0] & 0b1000_0000 != 0;
    let version = data[0] & 0b0111_1111;
    *data = &data[1..];

    // We only know how to decode V4 extrinsics at the moment
    if version != 4 {
      return Err(Error::CannotDecodeExtrinsicVersion(version));
    }

    // If the extrinsic is signed, decode the signature next.
    let signature = match is_signed {
      true => Some(self.decode_signature(data)?),
      false => None,
    };

    // Finally, decode the call data.
    let call_data = self.decode_call_data(data)?;

    Ok(Extrinsic {
      call_data,
      signature,
    })
  }

  fn decode_call_data(&'a self, data: &mut &[u8]) -> Result<CallData<'a>> {
    // Pluck out the u8's representing the pallet and call enum next.
    if data.len() < 2 {
      return Err(Error::EarlyEof(
        "expected at least 2 more bytes for the pallet/call index",
      ));
    }
    let pallet_index = u8::decode(data)?;
    let call_index = u8::decode(data)?;
    log::trace!("pallet index: {}, call index: {}", pallet_index, call_index);

    // Work out which call the extrinsic data represents and get type info for it:
    let (pallet_name, variant) = match self
      .internal_metadata
      .call_variant_by_enum_index(pallet_index, call_index)
    {
      Some(call) => call,
      None => return Err(Error::CannotFindCall(pallet_index, call_index)),
    };

    // Decode each of the argument values in the extrinsic:
    let arguments = variant
      .fields()
      .iter()
      .map(|field| self.decode(field.ty().id(), data))
      .collect::<Result<Vec<_>>>()?;

    Ok(CallData {
      pallet_name: Cow::Borrowed(pallet_name),
      ty: Cow::Borrowed(variant),
      arguments,
    })
  }

  fn decode_signature(&'a self, data: &mut &[u8]) -> Result<ExtrinsicSignature<'a>> {
    let address = <MultiAddress<AccountId32, u32>>::decode(data)?;
    let signature = MultiSignature::decode(data)?;
    let extensions = self.decode_signed_extensions(data)?;

    Ok(ExtrinsicSignature {
      address,
      signature,
      extensions,
    })
  }

  fn decode_signed_extensions(&'a self, data: &mut &[u8]) -> Result<Vec<(Cow<'a, str>, Value)>> {
    self
      .internal_metadata
      .extrinsic()
      .signed_extensions()
      .iter()
      .map(|ext| {
        let val = self.decode(ext.ty.id(), data)?;
        let name = Cow::Borrowed(&*ext.identifier);
        Ok((name, val))
      })
      .collect()
  }

  fn decode(&'a self, type_id: u32, input: &mut &[u8]) -> Result<Value> {
    let ty = self
      .runtime_metadata
      .types
      .resolve(type_id)
      .ok_or_else(|| {
        Error::DecodeError(format!("Failed to resolve type with id `{:?}`", type_id))
      })?;
    log::debug!(
      "Decoding input with type id `{:?}` and definition `{:?}`",
      type_id,
      ty
    );
    match self.env_types.try_decode(type_id, input) {
      // Value was decoded with custom decoder for type.
      Ok(Some(value)) => Ok(value),
      // No custom decoder registered so attempt default decoding.
      Ok(None) => scale_value::scale::decode_as_type(input, type_id, &self.runtime_metadata.types)
        .map(|val| val.remove_context())
        .map_err(Into::into),
      Err(e) => Err(e),
    }
  }
}
