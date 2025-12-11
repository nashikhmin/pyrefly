/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod test;

use std::collections::HashMap;
use std::fs::ReadDir;
use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;

use anyhow::bail;
use anyhow::Result;

use crate::default_vfs::DefaultFs;

/// Object-safe virtual filesystem interface.
pub trait Vfs: Send + Sync {
    /// Read file as UTF-8 string.
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Read file as bytes.
    fn read(&self, path: &Path) -> Result<Vec<u8>>;

    /// Write entire file (overwrites if exists).
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;

    /// Iterate over directory contents.
    fn read_dir(&self, path: &Path) -> Result<ReadDir>;

    /// Create directory and all missing parent directories.
    fn create_dir_all(&self, path: &Path) -> Result<()>;
}

/// Convenient alias for sharing VFS.
pub type ArcVfs = Arc<dyn Vfs>;

/// Provider for creating VFS implementations.
pub trait VfsProvider: Send + Sync {
    /// Provider name (unique).
    fn name(&self) -> &'static str;

    /// Create new VFS with given arguments.
    fn create(&self, args: &HashMap<String, String>) -> Result<ArcVfs>;
}

/// Slice for collecting all registered VFS providers via linkme.
#[linkme::distributed_slice]
pub static VFS_PROVIDERS: [&'static dyn VfsProvider] = [..];

/// Current global VFS (defaults to local).
static CURRENT_VFS: OnceLock<RwLock<ArcVfs>> = OnceLock::new();

fn vfs_lock() -> &'static RwLock<ArcVfs> {
    CURRENT_VFS.get_or_init(|| RwLock::new(Arc::new(DefaultFs)))
}

/// Get current global VFS.
pub fn current_vfs() -> ArcVfs {
    vfs_lock()
        .read()
        .expect("VFS RwLock poisoned (read)")
        .clone()
}

/// Replace current filesystem.
///
/// fsName: implementation name (registered providers via linkme).
/// args: arguments for provider implementation.
#[allow(non_snake_case)]
pub fn replaceFS(fsName: &str, args: &HashMap<String, String>) -> Result<ArcVfs> {
    // Look for registered providers (including default)
    for provider in &*VFS_PROVIDERS {
        if provider.name() == fsName {
            let new_fs = provider.create(args)?;
            *vfs_lock().write().expect("VFS RwLock poisoned (write)") = new_fs.clone();
            return Ok(new_fs);
        }
    }
    bail!("Unknown FS name: {}", fsName)
}
