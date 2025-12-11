/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::vfs::{replaceFS, VFS_PROVIDERS};

    #[test]
    fn test_vfs_providers_registration() {
        // Проверяем что default зарегистрирован через linkme
        let provider_names: Vec<&str> = VFS_PROVIDERS.iter().map(|p| p.name()).collect();
        assert!(provider_names.contains(&"default"));
    }

    #[test]
    fn test_replace_fs_with_default() {
        let args = HashMap::new();
        let result = replaceFS("default", &args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_replace_fs_with_unknown() {
        let args = HashMap::new();
        let result = replaceFS("unknown", &args);
        assert!(result.is_err());
    }
}