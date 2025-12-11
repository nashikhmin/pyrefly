/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use pyrefly_remote_fs::RemoteFs;
use pyrefly_util::vfs::{replaceFS, Vfs};
use std::collections::HashMap;
use std::path::Path;
pub mod mock_server;

use self::mock_server::start_mock_server;

#[tokio::test]
async fn test_full_vfs_workflow() {
    let port = 9000;
    let (state, _handle) = start_mock_server(port).await;

    // Test creating RemoteFs with valid token
    let remote_fs = tokio::task::spawn_blocking(move || {
        RemoteFs::new(port, "valid_token".to_string())
    }).await.unwrap();

    assert!(remote_fs.is_ok(), "Creating RemoteFs should succeed with valid token");
    let remote_fs = remote_fs.unwrap();

    // Test read existing file
    let content = tokio::task::spawn_blocking({
        let remote_fs = remote_fs.clone();
        move || {
            remote_fs.read_to_string(Path::new("/test.txt"))
        }
    }).await.unwrap();

    assert!(content.is_ok(), "Reading existing file should succeed");
    assert_eq!(content.unwrap(), "test content");

    // Test read binary file
    let binary_content = tokio::task::spawn_blocking({
        let remote_fs = remote_fs.clone();
        move || {
            remote_fs.read(Path::new("/binary.bin"))
        }
    }).await.unwrap();

    assert!(binary_content.is_ok(), "Reading binary file should succeed");
    assert_eq!(binary_content.unwrap(), vec![0x00, 0x01, 0x02, 0xFF]);

    // Test write new file
    let write_result = tokio::task::spawn_blocking({
        let remote_fs = remote_fs.clone();
        move || {
            remote_fs.write(Path::new("/new_file.txt"), b"new content")
        }
    }).await.unwrap();

    assert!(write_result.is_ok(), "Writing file should succeed");

    // Verify file was written
    let files = state.files.lock().unwrap();
    assert_eq!(files.get("/new_file.txt").unwrap(), b"new content");
    drop(files);

    // Test create directory
    let mkdir_result = tokio::task::spawn_blocking({
        let remote_fs = remote_fs.clone();
        move || {
            remote_fs.create_dir_all(Path::new("/new/nested/dir"))
        }
    }).await.unwrap();

    assert!(mkdir_result.is_ok(), "Creating directory should succeed");

    // Verify directory was created
    let directories = state.directories.lock().unwrap();
    assert!(directories.contains(&"/new/nested/dir".to_string()));

    // Test read_dir not supported
    let readdir_result = tokio::task::spawn_blocking({
        let remote_fs = remote_fs.clone();
        move || {
            remote_fs.read_dir(Path::new("/test_dir"))
        }
    }).await.unwrap();

    assert!(readdir_result.is_err(), "read_dir should not be supported");
    assert!(readdir_result.unwrap_err().to_string().contains("not supported"));
}

#[tokio::test]
async fn test_authentication_failure() {
    let port = 9001;
    let (_state, _handle) = start_mock_server(port).await;

    let result = tokio::task::spawn_blocking(move || {
        RemoteFs::new(port, "invalid_token".to_string())
    }).await.unwrap();

    // The RemoteFs will be created successfully, but the first request will fail
    assert!(result.is_ok(), "Creating RemoteFs should succeed even with invalid token");
    let remote_fs = result.unwrap();

    // Try to make a request with the invalid token
    let read_result = tokio::task::spawn_blocking(move || {
        remote_fs.read_to_string(Path::new("/test.txt"))
    }).await.unwrap();

    assert!(read_result.is_err(), "Request should fail with invalid token");
    assert!(read_result.unwrap_err().to_string().contains("Authentication failed"));
}

#[tokio::test]
async fn test_file_not_found() {
    let port = 9002;
    let (_state, _handle) = start_mock_server(port).await;

    let remote_fs = tokio::task::spawn_blocking(move || {
        RemoteFs::new(port, "valid_token".to_string())
    }).await.unwrap().unwrap();

    let result = tokio::task::spawn_blocking(move || {
        remote_fs.read_to_string(Path::new("/nonexistent.txt"))
    }).await.unwrap();

    assert!(result.is_err(), "Reading nonexistent file should fail");
    assert!(result.unwrap_err().to_string().contains("File not found"));
}

#[tokio::test]
async fn test_vfs_provider() {
    let port = 9003;
    let (_state, _handle) = start_mock_server(port).await;

    let result = tokio::task::spawn_blocking(move || {
        let mut args = HashMap::new();
        args.insert("port".to_string(), port.to_string());
        args.insert("token".to_string(), "valid_token".to_string());

        replaceFS("remote", &args)
    }).await.unwrap();

    assert!(result.is_ok(), "VfsProvider should create RemoteFs successfully");

    let vfs = result.unwrap();
    let content = tokio::task::spawn_blocking(move || {
        vfs.read_to_string(Path::new("/test.txt"))
    }).await.unwrap();

    assert!(content.is_ok(), "VFS operations should work through provider");
    assert_eq!(content.unwrap(), "test content");
}

#[tokio::test]
async fn test_pyproject_toml_deserialization() {
    let port = 9004;
    let (_state, _handle) = start_mock_server(port).await;

    let remote_fs = tokio::task::spawn_blocking(move || {
        RemoteFs::new(port, "valid_token".to_string())
    }).await.unwrap().unwrap();

    // Test reading pyproject.toml with specific content that matches the original issue
    let content = tokio::task::spawn_blocking({
        let remote_fs = remote_fs.clone();
        move || {
            remote_fs.read_to_string(Path::new("/pyproject.toml"))
        }
    }).await.unwrap();

    assert!(content.is_ok(), "Reading pyproject.toml should succeed");

    let expected_content = r#"[project]
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

    assert_eq!(content.unwrap(), expected_content);
}