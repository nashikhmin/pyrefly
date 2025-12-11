/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs::ReadDir;
use std::path::Path;

use anyhow::{bail, Result};
use pyrefly_util::vfs::Vfs;

use crate::client::RemoteClient;
use crate::types::{VfsRequest, VfsResponse, VfsResponseData};

#[derive(Debug, Clone)]
pub struct RemoteFs {
    client: RemoteClient,
}

impl RemoteFs {
    pub fn new(port: u16, token: String) -> Result<Self> {
        let client = RemoteClient::new(port, token)?;

        Ok(RemoteFs { client })
    }
}

impl Vfs for RemoteFs {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        let request = VfsRequest::ReadToString {
            path: path.to_string_lossy().to_string(),
        };

        match self.client.send_vfs_request(request)? {
            VfsResponse::Success { data: VfsResponseData::String { value: content } } => Ok(content),
            VfsResponse::Error { message } => bail!("Failed to read file: {}", message),
            _ => bail!("Unexpected response type for read_to_string"),
        }
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        let request = VfsRequest::Read {
            path: path.to_string_lossy().to_string(),
        };

        match self.client.send_vfs_request(request)? {
            VfsResponse::Success { data: VfsResponseData::Bytes { value: content } } => Ok(content),
            VfsResponse::Error { message } => bail!("Failed to read file: {}", message),
            _ => bail!("Unexpected response type for read"),
        }
    }

    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        let request = VfsRequest::Write {
            path: path.to_string_lossy().to_string(),
            contents: contents.to_vec(),
        };

        match self.client.send_vfs_request(request)? {
            VfsResponse::Success { data: VfsResponseData::Unit } => Ok(()),
            VfsResponse::Error { message } => bail!("Failed to write file: {}", message),
            _ => bail!("Unexpected response type for write"),
        }
    }

    fn read_dir(&self, _path: &Path) -> Result<ReadDir> {
        bail!("read_dir not supported in RemoteFs - std::fs::ReadDir cannot be created remotely")
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        let request = VfsRequest::CreateDirAll {
            path: path.to_string_lossy().to_string(),
        };

        match self.client.send_vfs_request(request)? {
            VfsResponse::Success { data: VfsResponseData::Unit } => Ok(()),
            VfsResponse::Error { message } => bail!("Failed to create directory: {}", message),
            _ => bail!("Unexpected response type for create_dir_all"),
        }
    }
}