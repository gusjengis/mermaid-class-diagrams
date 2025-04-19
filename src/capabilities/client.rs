use lsp_types::ClientCapabilities;

use super::workspace;

pub const capabilites: ClientCapabilities = ClientCapabilities {
    workspace: workspace::capabilities,
    text_document: None,
    window: None,
    general: None,
    experimental: None,
};
