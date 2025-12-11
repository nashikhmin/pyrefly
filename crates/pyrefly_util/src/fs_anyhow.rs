/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs::ReadDir;
use std::path::Path;

use crate::vfs::current_vfs;

pub fn read_to_string(path: &Path) -> anyhow::Result<String> {
    current_vfs().read_to_string(path)
}

pub fn read(path: &Path) -> anyhow::Result<Vec<u8>> {
    current_vfs().read(path)
}

pub fn write(path: &Path, contents: impl AsRef<[u8]>) -> anyhow::Result<()> {
    current_vfs().write(path, contents.as_ref())
}

pub fn read_dir(path: &Path) -> anyhow::Result<ReadDir> {
    current_vfs().read_dir(path)
}

pub fn create_dir_all(path: &Path) -> anyhow::Result<()> {
    current_vfs().create_dir_all(path)
}
