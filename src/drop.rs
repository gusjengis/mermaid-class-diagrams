use async_lsp_client::LspServer;
pub async fn drop(server: &mut LspServer) {
    // this is stalling indefinitely, wtf
    server.shutdown().await;
    server.exit().await;
}
