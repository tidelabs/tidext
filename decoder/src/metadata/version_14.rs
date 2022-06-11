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

use super::{u8_map::U8Map, Metadata, MetadataCalls, MetadataExtrinsic, MetadataPalletCalls};
use crate::Error;
use frame_metadata::RuntimeMetadataV14;

/// Decode V14 metadata into our general Metadata struct
pub fn decode(meta: RuntimeMetadataV14) -> Result<Metadata, Error> {
  let registry = meta.types;
  let mut pallet_calls_by_index = U8Map::new();

  // Gather some details about the extrinsic itself:
  let extrinsic = MetadataExtrinsic {
    version: meta.extrinsic.version,
    signed_extensions: meta.extrinsic.signed_extensions,
  };

  // Gather information about the calls/storage in use:
  for pallet in meta.pallets {
    // capture the call information in this pallet:
    let calls = pallet
      .calls
      .map(|call_md| {
        // Get the type representing the variant of available calls:
        let calls_type_id = call_md.ty;
        let calls_type = registry
          .resolve(calls_type_id.id())
          .ok_or_else(|| Error::TypeNotFound(calls_type_id.id()))?;

        // Expect that type to be a variant:
        let calls_type_def = calls_type.type_def();
        let calls_variant = match calls_type_def {
          scale_info::TypeDef::Variant(variant) => variant,
          _ => {
            return Err(Error::ExpectedVariantType {
              got: format!("{:?}", calls_type_def),
            });
          }
        };

        // Store the mapping from u8 index to variant slice index for quicker decode lookup:
        let call_variant_indexes = calls_variant
          .variants()
          .iter()
          .enumerate()
          .map(|(idx, v)| (v.index(), idx))
          .collect();

        Ok(MetadataCalls {
          calls_type_id,
          call_variant_indexes,
        })
      })
      .transpose()?;
    pallet_calls_by_index.insert(
      pallet.index,
      MetadataPalletCalls {
        name: pallet.name,
        calls,
      },
    );
  }

  Ok(Metadata {
    pallet_calls_by_index,
    extrinsic,
    types: registry,
  })
}
