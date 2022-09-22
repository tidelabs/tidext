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

use crate::{test_context, test_context_tidechain};
use tidext::{Error, TidefiRuntime};

#[macro_export]
macro_rules! with_tidext_runtime {
	{
		$self:ident,
		$client:ident,
		{
			$( $code:tt )*
		}
	} => {
		match $self.runtime_type() {
			TidefiRuntime::Tidechain($client) => { $( $code )* },
			TidefiRuntime::Lagoon($client) => { $( $code )* },
		}
	}
}

fn type_of<T>(_: &T) -> String {
  std::any::type_name::<T>().into()
}

#[cfg(feature = "tidechain-native")]
#[async_std::test]
async fn dynamic_tidechain_runtime() -> Result<(), Error> {
  let cxt = test_context_tidechain().await;
  let client = cxt.node_proc.client().clone();

  assert!(matches!(
    client.runtime_type(),
    TidefiRuntime::Tidechain { .. }
  ));

  Ok(())
}

#[cfg(feature = "lagoon-native")]
#[async_std::test]
async fn dynamic_lagoon_runtime() -> Result<(), Error> {
  let cxt = test_context().await;
  let client = cxt.node_proc.client().clone();

  assert!(matches!(
    client.runtime_type(),
    TidefiRuntime::Lagoon { .. }
  ));

  Ok(())
}

#[cfg(feature = "lagoon-native")]
#[async_std::test]
async fn dynamic_lagoon_macro_runtime() -> Result<(), Error> {
  let cxt = test_context().await;
  let client = cxt.node_proc.client().clone();
  with_tidext_runtime! {
    client,
    current_runtime,
    {
      assert_eq!(
        type_of(&current_runtime.storage()),
        "&tidext::subxt_impl::lagoon::StorageApi",
      );
      assert_eq!(
        type_of(&current_runtime.constants()),
        "&tidext::subxt_impl::lagoon::ConstantsApi",
      );
      assert_eq!(
        type_of(&current_runtime.tx()),
        "&tidext::subxt_impl::lagoon::TransactionApi",
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
  with_tidext_runtime! {
    client,
    current_runtime,
    {
      assert_eq!(
        type_of(&current_runtime.storage()),
        "&tidext::subxt_impl::tidechain::StorageApi",
      );
      assert_eq!(
        type_of(&current_runtime.constants()),
        "&tidext::subxt_impl::tidechain::ConstantsApi",
      );
      assert_eq!(
        type_of(&current_runtime.tx()),
        "&tidext::subxt_impl::tidechain::TransactionApi",
      );
      Ok(())
    }
  }
}
