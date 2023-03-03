// Copyright 2021-2023 Semantic Network Ltd.
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

use crate::{test_context, test_context_tidechain};
use tidext::{with_runtime, Error};

fn type_of<T>(_: &T) -> String {
  std::any::type_name::<T>().into()
}

#[cfg(feature = "tidechain-native")]
#[async_std::test]
async fn dynamic_tidechain_runtime() -> Result<(), Error> {
  let cxt = test_context_tidechain().await;
  let client = cxt.node_proc.client().clone();

  assert!(client.runtime_type().id().contains("Tidechain"));
  Ok(())
}

#[cfg(feature = "lagoon-native")]
#[async_std::test]
async fn dynamic_lagoon_runtime() -> Result<(), Error> {
  let cxt = test_context().await;
  let client = cxt.node_proc.client().clone();

  assert!(client.runtime_type().id().contains("Lagoon"));
  Ok(())
}

#[cfg(feature = "lagoon-native")]
#[async_std::test]
async fn dynamic_lagoon_macro_runtime() -> Result<(), Error> {
  let cxt = test_context().await;
  let client = cxt.node_proc.client().clone();
  with_runtime! {
    client,
    current_runtime,
    {
      let spec_version = current_runtime.spec_version();
      assert_eq!(
        type_of(&current_runtime.storage()),
        format!("&tidext::subxt_impl::runtimes::lagoon{}::StorageApi", spec_version),
      );
      assert_eq!(
        type_of(&current_runtime.constants()),
        format!("&tidext::subxt_impl::runtimes::lagoon{}::ConstantsApi", spec_version),
      );
      assert_eq!(
        type_of(&current_runtime.tx()),
        format!("&tidext::subxt_impl::runtimes::lagoon{}::TransactionApi", spec_version),
      );
      Ok(())
    }
  }
}

#[cfg(feature = "tidechain-native")]
#[async_std::test]
async fn dynamic_tidechain_macro_runtime() -> Result<(), Error> {
  let cxt = test_context_tidechain().await;
  let client = cxt.node_proc.client().clone();
  with_runtime! {
    client,
    current_runtime,
    {
      let spec_version = current_runtime.spec_version();
      assert_eq!(
        type_of(&current_runtime.storage()),
        format!("&tidext::subxt_impl::runtimes::tidechain{}::StorageApi", spec_version),
      );
      assert_eq!(
        type_of(&current_runtime.constants()),
        format!("&tidext::subxt_impl::runtimes::tidechain{}::ConstantsApi", spec_version),
      );
      assert_eq!(
        type_of(&current_runtime.tx()),
        format!("&tidext::subxt_impl::runtimes::tidechain{}::TransactionApi", spec_version),
      );
      Ok(())
    }
  }
}
