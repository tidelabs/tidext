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

mod u8_map;
mod version_14;

use crate::{Error, ScaleInfoTypeId, Type, TypeId};
use frame_metadata::RuntimeMetadata;
use scale_info::{form::PortableForm, PortableRegistry};
use u8_map::U8Map;

// Some type aliases used below. `scale-info` is re-exported at the root,
// so to avoid confusion we only publicly export all scale-info types from that
// one place.
type TypeDefVariant = scale_info::TypeDefVariant<PortableForm>;
type SignedExtensionMetadata = frame_metadata::SignedExtensionMetadata<PortableForm>;

/// This is a representation of the SCALE encoded metadata obtained from a substrate
/// node.
#[derive(Debug)]
pub struct Metadata {
  /// Details about the extrinsic format.
  extrinsic: MetadataExtrinsic,
  /// Hash pallet calls by index, since when decoding, we'll have the pallet/call
  /// `u8`'s available to us to look them up by.
  pallet_calls_by_index: U8Map<MetadataPalletCalls>,

  /// Type information lives inside this.
  types: PortableRegistry,
}

impl Metadata {
  /// Convert the substrate runtime metadata into our Metadata.
  pub fn from_runtime_metadata(metadata: RuntimeMetadata) -> Result<Self, Error> {
    match metadata {
      RuntimeMetadata::V14(meta_v14) => {
        log::trace!("V14 metadata found.");
        version_14::decode(meta_v14)
      }
      unsupported_meta => Err(Error::UnsupportedVersion(unsupported_meta.version())),
    }
  }

  /// Return details about the type of extrinsic supported by this metadata.
  pub fn extrinsic(&self) -> &MetadataExtrinsic {
    &self.extrinsic
  }

  /// Given a [`crate::TypeId`], return the corresponding type from the type registry, if possible.
  pub fn resolve<Id: Into<TypeId>>(&self, id: Id) -> Option<&Type> {
    self.types.resolve(id.into())
  }

  /// Given the `u8` variant index of a pallet and call, this returns the pallet name and the call Variant
  /// if found, or `None` if no such call exists at those indexes, or we don't have suitable call data.
  pub(crate) fn call_variant_by_enum_index(
    &self,
    pallet: u8,
    call: u8,
  ) -> Option<(&str, &scale_info::Variant<PortableForm>)> {
    self.pallet_calls_by_index.get(pallet).and_then(|p| {
      p.calls.as_ref().and_then(|calls| {
        let type_def_variant = self.get_variant(calls.calls_type_id)?;
        let index = *calls.call_variant_indexes.get(call)?;
        let variant = type_def_variant.variants().get(index)?;
        Some((&*p.name, variant))
      })
    })
  }

  /// A helper function to get hold of a Variant given a type ID, or None if it's not found.
  fn get_variant(&self, ty: ScaleInfoTypeId) -> Option<&TypeDefVariant> {
    self
      .types
      .resolve(ty.id())
      .and_then(|ty| match ty.type_def() {
        scale_info::TypeDef::Variant(variant) => Some(variant),
        _ => None,
      })
  }
}

#[derive(Debug)]
struct MetadataPalletCalls {
  /// The pallet name.
  name: String,
  /// Metadata may not contain call information. If it does,
  /// it'll be here.
  calls: Option<MetadataCalls>,
}

#[derive(Debug)]
struct MetadataCalls {
  /// This allows us to find the type information corresponding to
  /// the call in the [`PortableRegistry`]/
  calls_type_id: ScaleInfoTypeId,
  /// This allows us to map a u8 enum index to the correct call variant
  /// from the calls type, above. The variant contains information on the
  /// fields and such that the call has.
  call_variant_indexes: U8Map<usize>,
}

/// Information about the extrinsic format supported on the substrate node
/// that the metadata was obtained from.
#[derive(Debug, Clone)]
pub struct MetadataExtrinsic {
  version: u8,
  signed_extensions: Vec<SignedExtensionMetadata>,
}

impl MetadataExtrinsic {
  /// The version of the extrinsic format in use by the node.
  #[allow(unused)]
  pub fn version(&self) -> u8 {
    self.version
  }

  /// Part of the extrinsic signature area can be varied to include whatever information
  /// a node decides is important. This returns details about that part.
  pub(crate) fn signed_extensions(&self) -> &[SignedExtensionMetadata] {
    &self.signed_extensions
  }
}
