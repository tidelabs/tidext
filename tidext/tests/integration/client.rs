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

use crate::test_node_process;

#[async_std::test]
async fn rpc_subscribe_finalized_blocks() {
  let node_process = test_node_process().await;
  let client = node_process.client();
  let mut blocks = client
    .runtime()
    .rpc()
    .subscribe_finalized_block_headers()
    .await
    .unwrap();
  blocks.next().await.unwrap().unwrap();
}
