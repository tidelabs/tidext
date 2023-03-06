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

use subxt::{Config, PolkadotConfig, SubstrateConfig as DefaultConfig};
use tidext_macro::runtimes;

#[runtimes(runtime_metadata_path = "../artifacts")]
pub mod runtimes {}

/// Custom [`Config`] implementation with Clone
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TidechainConfig;
impl Config for TidechainConfig {
  type Index = <DefaultConfig as Config>::Index;
  type Hash = <DefaultConfig as Config>::Hash;
  type Hasher = <DefaultConfig as Config>::Hasher;
  type AccountId = tidefi_primitives::AccountId;
  type Address = <DefaultConfig as Config>::Address;
  type Header = <DefaultConfig as Config>::Header;
  type Signature = <DefaultConfig as Config>::Signature;
  type ExtrinsicParams = <PolkadotConfig as Config>::ExtrinsicParams;
}

#[cfg(test)]
mod test {
  use super::runtimes::*;

  #[test]
  fn spec_name() {
    assert_eq!(
      Tidechain6040Runtime::default().spec_name(),
      "tidechain".to_string()
    );
    assert_eq!(
      Lagoon6040Runtime::default().spec_name(),
      "lagoon".to_string()
    );
  }

  #[test]
  fn spec_version() {
    assert_eq!(Tidechain6040Runtime::default().spec_version(), 6040);
    assert_eq!(Lagoon6030Runtime::default().spec_version(), 6030);
  }

  #[test]
  fn ids() {
    assert_eq!(Tidechain6040Runtime::default().id(), "Tidechain6040Runtime");
    assert_eq!(Lagoon6030Runtime::default().id(), "Lagoon6030Runtime");
  }

  #[test]
  fn runtimes() {
    let mut all_runtimes = TidefiRuntime::runtimes()
      .iter()
      .map(|r| r.id())
      .collect::<Vec<String>>();
    all_runtimes.sort();

    assert_eq!(
      all_runtimes,
      &[
        "Lagoon6030Runtime".to_string(),
        "Lagoon6040Runtime".to_string(),
        "Lagoon7000Runtime".to_string(),
        "Tidechain6030Runtime".to_string(),
        "Tidechain6040Runtime".to_string(),
        "Tidechain7000Runtime".to_string(),
      ]
    );
  }

  #[test]
  fn select_runtime() {
    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 6000).id(),
      "Tidechain6030Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 6000).id(),
      "Lagoon6030Runtime".to_string()
    );

    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 6030).id(),
      "Tidechain6030Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 6030).id(),
      "Lagoon6030Runtime".to_string()
    );

    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 6035).id(),
      "Tidechain6030Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 6035).id(),
      "Lagoon6030Runtime".to_string()
    );

    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 6040).id(),
      "Tidechain6040Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 6040).id(),
      "Lagoon6040Runtime".to_string()
    );

    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 6050).id(),
      "Tidechain6040Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 6050).id(),
      "Lagoon6040Runtime".to_string()
    );

    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 7000).id(),
      "Tidechain7000Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 7000).id(),
      "Lagoon7000Runtime".to_string()
    );

    assert_eq!(
      TidefiRuntime::select_runtime("tidechain", 8000).id(),
      "Tidechain7000Runtime".to_string()
    );
    assert_eq!(
      TidefiRuntime::select_runtime("lagoon", 8000).id(),
      "Lagoon7000Runtime".to_string()
    );
  }
}
