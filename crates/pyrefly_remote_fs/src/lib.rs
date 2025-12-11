/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use linkme::distributed_slice;
use pyrefly_util::vfs::{ArcVfs, VfsProvider, VFS_PROVIDERS};

mod remote;
mod client;
pub mod types;

mod test;

pub use remote::RemoteFs;

struct RemoteFsProvider;

impl VfsProvider for RemoteFsProvider {
    fn name(&self) -> &'static str {
        "remote"
    }

    fn create(&self, args: &HashMap<String, String>) -> Result<ArcVfs> {
        let port = args.get("port")
            .ok_or_else(|| anyhow::anyhow!("Missing 'port' argument for remote VFS"))?
            .parse::<u16>()?;

        let token = args.get("token")
            .ok_or_else(|| anyhow::anyhow!("Missing 'token' argument for remote VFS"))?
            .clone();

        let remote_fs = RemoteFs::new(port, token)?;
        Ok(Arc::new(remote_fs))
    }
}

#[distributed_slice(VFS_PROVIDERS)]
static REMOTE_FS_PROVIDER: &dyn VfsProvider = &RemoteFsProvider;