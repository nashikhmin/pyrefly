/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitRequest {
    pub token: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum VfsRequest {
    ReadToString { path: String },
    Read { path: String },
    Write { path: String, contents: Vec<u8> },
    ReadDir { path: String },
    CreateDirAll { path: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VfsResponse {
    Success { data: VfsResponseData },
    Error { message: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VfsResponseData {
    String(String),
    Bytes(Vec<u8>),
    DirEntries(Vec<String>),
    Unit,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ureq::serde_json;

    #[test]
    fn test_init_request_serialization() {
        let request = InitRequest {
            token: "test_token".to_string(),
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let deserialized: InitRequest = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(request.token, deserialized.token);
    }


    #[test]
    fn test_vfs_request_read_to_string() {
        let request = VfsRequest::ReadToString {
            path: "/test.txt".to_string(),
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let deserialized: VfsRequest = serde_json::from_slice(&serialized).unwrap();

        match deserialized {
            VfsRequest::ReadToString { path } => {
                assert_eq!(path, "/test.txt");
            }
            _ => panic!("Expected ReadToString request"),
        }
    }

    #[test]
    fn test_vfs_request_write() {
        let request = VfsRequest::Write {
            path: "/test.txt".to_string(),
            contents: vec![1, 2, 3, 4],
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let deserialized: VfsRequest = serde_json::from_slice(&serialized).unwrap();

        match deserialized {
            VfsRequest::Write { path, contents } => {
                assert_eq!(path, "/test.txt");
                assert_eq!(contents, vec![1, 2, 3, 4]);
            }
            _ => panic!("Expected Write request"),
        }
    }

    #[test]
    fn test_vfs_response_string_data() {
        let response = VfsResponse::Success {
            data: VfsResponseData::String("test content".to_string()),
        };

        let serialized = serde_json::to_vec(&response).unwrap();
        let deserialized: VfsResponse = serde_json::from_slice(&serialized).unwrap();

        match deserialized {
            VfsResponse::Success { data: VfsResponseData::String(content) } => {
                assert_eq!(content, "test content");
            }
            _ => panic!("Expected Success response with String data"),
        }
    }

    #[test]
    fn test_vfs_response_bytes_data() {
        let response = VfsResponse::Success {
            data: VfsResponseData::Bytes(vec![0xFF, 0x00, 0x42]),
        };

        let serialized = serde_json::to_vec(&response).unwrap();
        let deserialized: VfsResponse = serde_json::from_slice(&serialized).unwrap();

        match deserialized {
            VfsResponse::Success { data: VfsResponseData::Bytes(bytes) } => {
                assert_eq!(bytes, vec![0xFF, 0x00, 0x42]);
            }
            _ => panic!("Expected Success response with Bytes data"),
        }
    }

    #[test]
    fn test_vfs_response_unit_data() {
        let response = VfsResponse::Success {
            data: VfsResponseData::Unit,
        };

        let serialized = serde_json::to_vec(&response).unwrap();
        let deserialized: VfsResponse = serde_json::from_slice(&serialized).unwrap();

        match deserialized {
            VfsResponse::Success { data: VfsResponseData::Unit } => {}
            _ => panic!("Expected Success response with Unit data"),
        }
    }

    #[test]
    fn test_vfs_response_error() {
        let response = VfsResponse::Error {
            message: "File not found".to_string(),
        };

        let serialized = serde_json::to_vec(&response).unwrap();
        let deserialized: VfsResponse = serde_json::from_slice(&serialized).unwrap();

        match deserialized {
            VfsResponse::Error { message } => {
                assert_eq!(message, "File not found");
            }
            _ => panic!("Expected Error response"),
        }
    }
}