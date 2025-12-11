/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context as _, Result};

use crate::vfs::{ArcVfs, Vfs, VfsProvider};

/// Default local filesystem.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultFs;

impl Vfs for DefaultFs {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("When reading file `{}`", path.display()))
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        fs::read(path).with_context(|| format!("When reading file `{}`", path.display()))
    }

    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        fs::write(path, contents)
            .with_context(|| format!("When writing file `{}`", path.display()))
    }

    fn read_dir(&self, path: &Path) -> Result<ReadDir> {
        fs::read_dir(path)
            .with_context(|| format!("When reading directory `{}`", path.display()))
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .with_context(|| format!("When creating directory `{}`", path.display()))
    }
}

/// Provider for local filesystem.
struct DefaultFsProvider;

impl VfsProvider for DefaultFsProvider {
    fn name(&self) -> &'static str {
        "default"
    }

    fn create(&self, _args: &HashMap<String, String>) -> Result<ArcVfs> {
        Ok(Arc::new(DefaultFs))
    }
}

/// Register provider via linkme.
#[linkme::distributed_slice(crate::vfs::VFS_PROVIDERS)]
static DEFAULT_FS_PROVIDER: &dyn VfsProvider = &DefaultFsProvider;
