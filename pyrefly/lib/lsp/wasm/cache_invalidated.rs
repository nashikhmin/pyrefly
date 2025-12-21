/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! LSP notification for cache invalidation events

use lsp_types::notification::Notification;
use lsp_types::TextDocumentIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CacheInvalidated {}

impl Notification for CacheInvalidated {
    type Params = CacheInvalidatedParams;
    const METHOD: &'static str = "types/cache-invalidated";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheInvalidatedParams {
    pub invalidated_files: Vec<TextDocumentIdentifier>,
}