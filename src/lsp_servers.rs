use lsp_servers::{lsp_servers::relevant_lsp_servers, settings::Settings};

pub fn start_lsp_servers(dir: &str) {
    let servers = relevant_lsp_servers(
        PathBuf::from(dir),
        &Settings {
            installed_only: true,
        },
    );

    for server in servers {
        start_lsp_server(server);
    }
}

pub fn start_lsp_server(server: &str) {
    println!("{}", server);
}
