use lsp_types::WorkspaceClientCapabilities;

pub const capabilities: Option<WorkspaceClientCapabilities> = Some(WorkspaceClientCapabilities {
    apply_edit: None,
    workspace_edit: None,
    did_change_configuration: None,
    did_change_watched_files: None,
    symbol: None,
    execute_command: None,
    workspace_folders: None,
    configuration: None,
    semantic_tokens: None,
    code_lens: None,
    file_operations: None,
    inline_value: None,
    inlay_hint: None,
    diagnostic: None,
});
