/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::types::{VfsRequest, VfsResponse};
use anyhow::{bail, Result};
use log::info;
use std::io::Read;
use std::time::Duration;
use ureq::serde_json;

#[derive(Debug, Clone)]
pub struct RemoteClient {
    base_url: String,
    token: String,
}

impl RemoteClient {
    pub fn new(port: u16, token: String) -> Result<Self> {
        let base_url = format!("http://localhost:{}", port);
        info!("Creating RemoteClient for {}", base_url);

        Ok(RemoteClient {
            base_url,
            token,
        })
    }

    pub fn send_vfs_request(&self, request: VfsRequest) -> Result<VfsResponse> {
        info!("Sending VFS request: {:?}", request);
        let request_data = serde_json::to_vec(&request)?;

        let response = match ureq::post(&format!("{}/lsp/vfs", self.base_url))
            .set("Content-Type", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
            .timeout(Duration::from_secs(3))
            .send_bytes(&request_data) {
            Ok(resp) => resp,
            Err(ureq::Error::Status(401, _)) => {
                bail!("Authentication failed");
            }
            Err(e) => return Err(e.into()),
        };

        info!("VFS request completed with status {}", response.status());
        let mut buffer = Vec::new();
        response.into_reader().read_to_end(&mut buffer)?;

        let vfs_response: VfsResponse = serde_json::from_slice(&buffer)?;
        info!("VFS response: {:?}", vfs_response);
        Ok(vfs_response)
    }
}