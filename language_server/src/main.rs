use anyhow::Result;

use lsp_server::Connection;
use move_language_server::server;

pub fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting language server");

    let (connection, io_threads) = Connection::stdio();
    log::info!("Transport is created, stdin and stdout are connected");

    server::run_server(&connection)?;

    io_threads.join()?;
    log::info!("Shutting down server");
    Ok(())
}
