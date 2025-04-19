use lsp_types::{
    ClientCapabilities, GotoDefinitionParams, WorkDoneProgressParams, WorkspaceClientCapabilities,
    request::GotoDefinition,
};
use std::process::Stdio;
use std::thread;
use tokio::process::Command;
use tokio::runtime::Runtime;

// Import the client and transport types from their correct modules.
use async_lsp_client::LspServer;

use std::path::PathBuf;

use lsp_servers::{lsp_servers::relevant_lsp_servers, settings::Settings};

use crate::capabilities;

pub async fn start_lsp_servers(dir: &str) -> Vec<LspServer> {
    let server_names = relevant_lsp_servers(
        PathBuf::from(dir),
        &Settings {
            installed_only: true,
        },
    );

    let mut servers = vec![];

    for server_name in server_names {
        servers.push(start_lsp_server(server_name).await);
    }

    servers
}

pub async fn start_lsp_server(server: &str) -> LspServer {
    let (server, rx) = LspServer::new(server, []);

    // initialize request
    let initialize_result = server
        .initialize(lsp_types::InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: capabilities::client::capabilites,
            trace: None,
            workspace_folders: None,
            client_info: None,
            locale: None,
        })
        .await;

    // todo - figure out how if I need to parse this mess and if so how to do it
    println!("{:?}", initialize_result.unwrap());

    // initialized notification
    server.initialized().await;

    return server;
}
