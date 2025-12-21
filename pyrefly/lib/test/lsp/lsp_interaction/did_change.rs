/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use lsp_server::Message;
use lsp_types::notification::DidChangeTextDocument;
use lsp_types::notification::Notification;
use lsp_types::TextDocumentIdentifier;
use lsp_types::Url;
use serde_json::json;

use crate::test::lsp::lsp_interaction::object_model::InitializeSettings;
use crate::test::lsp::lsp_interaction::object_model::LspInteraction;
use crate::test::lsp::lsp_interaction::util::get_test_files_root;

#[test]
fn test_text_document_did_change() {
    let root = get_test_files_root();
    let mut interaction = LspInteraction::new();
    interaction.set_root(root.path().to_path_buf());
    interaction
        .initialize(InitializeSettings::default())
        .unwrap();

    interaction.client.did_open("text_document.py");

    let filepath = root.path().join("text_document.py");
    interaction
        .client
        .send_notification::<DidChangeTextDocument>(json!({
            "textDocument": {
                "uri": Url::from_file_path(&filepath).unwrap().to_string(),
                "languageId": "python",
                "version": 2
            },
            "contentChanges": [{
                "range": {
                    "start": {"line": 6, "character": 0},
                    "end": {"line": 7, "character": 0}
                },
                "text": format!("{}\n", "rint(\"another change\")")
            }],
        }));

    interaction
        .client
        .send_notification::<DidChangeTextDocument>(json!({
            "textDocument": {
                "uri": Url::from_file_path(&filepath).unwrap().to_string(),
                "languageId": "python",
                "version": 3
            },
            "contentChanges": [{
                "range": {
                    "start": {"line": 6, "character": 0},
                    "end": {"line": 6, "character": 0}
                },
                "text": "p"
            }],
        }));

    interaction
        .client
        .diagnostic("text_document.py")
        .expect_response(json!({"items": [], "kind": "full"}))
        .unwrap();

    interaction.shutdown().unwrap();
}

#[test]
fn test_text_document_did_change_unicode() {
    let root = get_test_files_root();
    let mut interaction = LspInteraction::new();
    interaction.set_root(root.path().to_path_buf());
    interaction
        .initialize(InitializeSettings::default())
        .unwrap();

    interaction.client.did_open("utf.py");

    let utf_filepath = root.path().join("utf.py");
    interaction
        .client
        .send_notification::<DidChangeTextDocument>(json!({
            "textDocument": {
                "uri": Url::from_file_path(&utf_filepath).unwrap().to_string(),
                "languageId": "python",
                "version": 2
            },
            "contentChanges": [{
                "range": {
                    "start": { "line": 7, "character": 8 },
                    "end": { "line": 8, "character": 2 }
                },
                "rangeLength": 3,
                "text": ""
            }],
        }));

    interaction
        .client
        .send_notification::<DidChangeTextDocument>(json!({
            "textDocument": {
                "uri": Url::from_file_path(&utf_filepath).unwrap().to_string(),
                "languageId": "python",
                "version": 3
            },
            "contentChanges": [{
                "range": {
                    "start": { "line": 7, "character": 8 },
                    "end": { "line": 7, "character": 8 }
                },
                "rangeLength": 0,
                "text": format!("\n{}", "print(\"")
            }],
        }));

    interaction
        .client
        .diagnostic("utf.py")
        .expect_response(json!({"items": [], "kind": "full"}))
        .unwrap();

    interaction.shutdown().unwrap();
}

#[test]
fn test_cache_invalidation_unicode_filename() {
    use crate::lsp::wasm::cache_invalidated::CacheInvalidated;

    let root = get_test_files_root();
    let mut interaction = LspInteraction::new();
    interaction.set_root(root.path().to_path_buf());
    interaction
        .initialize(InitializeSettings::default())
        .unwrap();

    // Open the Unicode filename file
    interaction.client.did_open("вва.py");

    let unicode_filepath = root.path().join("вва.py");
    let expected_uri = Url::from_file_path(&unicode_filepath).unwrap();

    // Send a change to trigger cache invalidation
    interaction
        .client
        .send_notification::<DidChangeTextDocument>(json!({
            "textDocument": {
                "uri": expected_uri.to_string(),
                "languageId": "python",
                "version": 2
            },
            "contentChanges": [{
                "range": {
                    "start": {"line": 0, "character": 0},
                    "end": {"line": 0, "character": 0}
                },
                "text": "# Modified\n"
            }],
        }));

    // Expect cache invalidation notification with properly decoded Unicode URI
    interaction
        .client
        .expect_message("Cache invalidation notification with Unicode filename", |msg| {
            if let Message::Notification(notification) = msg {
                if notification.method == CacheInvalidated::METHOD {
                    // Parse the notification params as CacheInvalidatedParams
                    if let Ok(params) = serde_json::from_value::<crate::lsp::wasm::cache_invalidated::CacheInvalidatedParams>(notification.params) {
                        if params.invalidated_files.len() == 1 {
                            let text_doc = &params.invalidated_files[0];
                            // Convert back to file path to check Unicode characters are preserved
                            if let Ok(path) = text_doc.uri.to_file_path() {
                                let filename = path.file_name().unwrap().to_string_lossy();
                                if filename == "вва.py" {
                                    return Some(());
                                }
                            }
                        }
                    }
                }
            }
            None
        })
        .unwrap();

    interaction.shutdown().unwrap();
}


#[test]
fn test_cache_invalidated_params_serialization() {
    use crate::lsp::wasm::cache_invalidated::CacheInvalidatedParams;

    // Test Unicode URI serialization/deserialization
    let unicode_path = std::path::Path::new("/Users/test/вва.py");
    let unicode_uri = Url::from_file_path(unicode_path).unwrap();

    let text_doc_id = TextDocumentIdentifier { uri: unicode_uri.clone() };
    let params = CacheInvalidatedParams {
        invalidated_files: vec![text_doc_id.clone()],
    };

    // Serialize to JSON (as LSP protocol would)
    let json = serde_json::to_string(&params).unwrap();

    // Deserialize back
    let deserialized: CacheInvalidatedParams = serde_json::from_str(&json).unwrap();

    // Verify TextDocumentIdentifier is preserved correctly
    assert_eq!(deserialized.invalidated_files.len(), 1);
    assert_eq!(deserialized.invalidated_files[0].uri, unicode_uri);

    // Verify we can get the correct filename back
    let path = deserialized.invalidated_files[0].uri.to_file_path().unwrap();
    let filename = path.file_name().unwrap().to_string_lossy();
    assert_eq!(filename, "вва.py");
}
