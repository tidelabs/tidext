use crate::test_node_process;

#[async_std::test]
async fn rpc_subscribe_finalized_blocks() {
  let node_process = test_node_process().await;
  let client = node_process.client();
  let mut blocks = client
    .runtime()
    .client
    .rpc()
    .subscribe_finalized_blocks()
    .await
    .unwrap();
  blocks.next().await.unwrap().unwrap();
}
