/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::post,
    Router,
};
use tokio::net::TcpListener;

use pyrefly_remote_fs::types::{VfsRequest, VfsResponse, VfsResponseData};
use ureq::serde_json;

#[derive(Debug, Clone, Default)]
pub struct MockServerState {
    pub authenticated_tokens: Arc<Mutex<Vec<String>>>,
    pub files: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    pub directories: Arc<Mutex<Vec<String>>>,
}

impl MockServerState {
    pub fn new() -> Self {
        let state = Self::default();

        // Pre-populate with some test data
        {
            let mut files = state.files.lock().unwrap();
            files.insert("/test.txt".to_string(), b"test content".to_vec());
            files.insert("/binary.bin".to_string(), vec![0x00, 0x01, 0x02, 0xFF]);

            // Add pyproject.toml test data
            let pyproject_content = r#"[project]
name = "pythonproject9"
version = "0.1.0"
description = "Add your description here"
requires-python = ">=3.13"
dependencies = [
    "jupyterlab>=4.4.10",
]

[tool]

[tool.pyrefly]
project-includes = [
    "**/*.py*",
    "**/*.ipynb",
]
"#;
            files.insert("/pyproject.toml".to_string(), pyproject_content.as_bytes().to_vec());
        }

        {
            let mut dirs = state.directories.lock().unwrap();
            dirs.push("/test_dir".to_string());
        }

        state
    }

    pub fn add_valid_token(&self, token: String) {
        self.authenticated_tokens.lock().unwrap().push(token);
    }
}

pub async fn start_mock_server(port: u16) -> (MockServerState, tokio::task::JoinHandle<()>) {
    let state = MockServerState::new();
    state.add_valid_token("valid_token".to_string());

    let app = Router::new()
        .route("/lsp/vfs", post(handle_vfs))
        .with_state(state.clone());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Failed to bind to address");

    let handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("Server failed");
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (state, handle)
}

async fn handle_vfs(
    State(state): State<MockServerState>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response<axum::body::Body> {
    // Check authorization header
    let auth_header = headers.get("Authorization");
    let token = match auth_header {
        Some(header_value) => {
            let auth_str = header_value.to_str().unwrap_or("");
            if auth_str.starts_with("Bearer ") {
                auth_str.strip_prefix("Bearer ").unwrap_or("")
            } else {
                ""
            }
        }
        None => "",
    };

    // Validate token
    if !state.authenticated_tokens.lock().unwrap().contains(&token.to_string()) {
        let error_response = VfsResponse::Error {
            message: "Authentication failed".to_string(),
        };
        let response_data = serde_json::to_vec(&error_response).unwrap();
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from(response_data))
            .unwrap();
    }
    let vfs_request: VfsRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => {
            let error_response = VfsResponse::Error {
                message: "Invalid JSON".to_string(),
            };
            let response_data = serde_json::to_vec(&error_response).unwrap();
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(axum::body::Body::from(response_data))
                .unwrap();
        }
    };

    let response = match vfs_request {
        VfsRequest::ReadToString { path } => {
            if let Some(content) = state.files.lock().unwrap().get(&path) {
                match String::from_utf8(content.clone()) {
                    Ok(text) => VfsResponse::Success {
                        data: VfsResponseData::String { value: text },
                    },
                    Err(_) => VfsResponse::Error {
                        message: "File is not valid UTF-8".to_string(),
                    },
                }
            } else {
                VfsResponse::Error {
                    message: "File not found".to_string(),
                }
            }
        }
        VfsRequest::Read { path } => {
            if let Some(content) = state.files.lock().unwrap().get(&path) {
                VfsResponse::Success {
                    data: VfsResponseData::Bytes { value: content.clone() },
                }
            } else {
                VfsResponse::Error {
                    message: "File not found".to_string(),
                }
            }
        }
        VfsRequest::Write { path, contents } => {
            state.files.lock().unwrap().insert(path, contents);
            VfsResponse::Success {
                data: VfsResponseData::Unit,
            }
        }
        VfsRequest::CreateDirAll { path } => {
            state.directories.lock().unwrap().push(path);
            VfsResponse::Success {
                data: VfsResponseData::Unit,
            }
        }
        VfsRequest::ReadDir { path: _ } => {
            VfsResponse::Error {
                message: "ReadDir not implemented in mock server".to_string(),
            }
        }
    };

    let response_data = serde_json::to_vec(&response).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(response_data))
        .unwrap()
}