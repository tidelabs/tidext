pub use crate::test_utils::{AccountKeyring, TestNodeProcess};
use crate::TidechainRuntimeApi;

/// substrate node should be installed on the `$PATH`
const TIDECHAIN_NODE_PATH: &str = "tidechain";

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

pub async fn test_node_process() -> TestNodeProcess {
  test_node_process_with(AccountKeyring::Alice).await
}

pub struct TestContext {
  pub node_proc: TestNodeProcess,
  pub api: TidechainRuntimeApi,
}

pub async fn test_context() -> TestContext {
  env_logger::try_init().ok();
  let node_proc = test_node_process().await;
  let api = node_proc.client().clone().runtime();
  TestContext { node_proc, api }
}
