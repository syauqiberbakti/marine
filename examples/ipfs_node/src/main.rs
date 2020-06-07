/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod node;
mod errors;
mod config;
mod imports;

use node::IpfsNode;

use std::path::PathBuf;

const IPFS_MODULES_DIR: &str = "/Users/mike/dev/work/fluence/wasm/fce/bin/wasm_modules";

const IPFS_MODULES_CONFIG_PATH: &str =
    "/Users/mike/dev/work/fluence/wasm/fce/examples/ipfs_node/Config.toml";

const IPFS_RPC: &str = "/Users/mike/dev/work/fluence/wasm/fce/bin/wasm_ipfs_rpc_wit.wasm";

fn main() {
    let ipfs_rpc = std::fs::read(IPFS_RPC).unwrap();

    let mut ipfs_node = IpfsNode::new(
        PathBuf::from(IPFS_MODULES_DIR),
        PathBuf::from(IPFS_MODULES_CONFIG_PATH),
    )
    .unwrap();

    let result = ipfs_node.rpc_call(&ipfs_rpc).unwrap();

    println!("execution result {:?}", result);
}