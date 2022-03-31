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

pub use crate::test_utils::{AccountKeyring, TestNodeProcess};

/// substrate node should be installed on the `$PATH`
const TIDECHAIN_NODE_PATH: &str = "tidechain";

/// Tidext node process with specific signer
pub async fn test_node_process_with(key: AccountKeyring) -> TestNodeProcess {
  let path = std::env::var("TIDECHAIN_NODE_PATH").unwrap_or_else(|_| {
    if which::which(TIDECHAIN_NODE_PATH).is_err() {
      panic!("A Tidechain binary should be installed on your path for integration tests.")
    }
    TIDECHAIN_NODE_PATH.to_string()
  });

  let proc = TestNodeProcess::build(path.as_str())
    .with_authority(key)
    .scan_for_open_ports()
    .spawn()
    .await;
  proc.unwrap()
}

/// Launch a node process with `Alice` as signer.
pub async fn test_node_process() -> TestNodeProcess {
  test_node_process_with(AccountKeyring::Alice).await
}

/// Tidext test context.
pub struct TestContext {
  pub node_proc: TestNodeProcess,
}

/// Initialize a default test context.
pub async fn test_context() -> TestContext {
  env_logger::try_init().ok();
  let node_proc = test_node_process().await;
  TestContext { node_proc }
}
