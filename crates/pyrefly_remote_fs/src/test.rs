/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#[cfg(test)]
mod basic_tests {
    use std::collections::HashMap;

    use pyrefly_util::vfs::{replaceFS, VFS_PROVIDERS};

    #[test]
    fn test_remote_fs_registration() {
        // Проверяем что remote зарегистрирован через linkme
        let provider_names: Vec<&str> = VFS_PROVIDERS.iter().map(|p| p.name()).collect();
        assert!(provider_names.contains(&"remote"));
        assert!(provider_names.contains(&"default"));

        // Должно быть как минимум 2 провайдера
        assert!(provider_names.len() >= 2);
    }

    #[test]
    fn test_replace_fs_with_empty_args() {
        let args = HashMap::new();
        let result = replaceFS("remote", &args);
        assert!(result.is_err(), "Remote FS should fail without required args");
    }

    #[test]
    fn test_vfs_provider_missing_port() {
        let mut args = HashMap::new();
        args.insert("token".to_string(), "valid_token".to_string());

        let result = replaceFS("remote", &args);
        assert!(result.is_err(), "VfsProvider should fail without port");
    }

    #[test]
    fn test_vfs_provider_missing_token() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "8089".to_string());

        let result = replaceFS("remote", &args);
        assert!(result.is_err(), "VfsProvider should fail without token");
    }

    #[test]
    fn test_vfs_provider_invalid_port() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "invalid_port".to_string());
        args.insert("token".to_string(), "valid_token".to_string());

        let result = replaceFS("remote", &args);
        assert!(result.is_err(), "VfsProvider should fail with invalid port");
    }
}