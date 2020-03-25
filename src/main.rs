use anyhow::Result;
use lsp_server::Connection;
use lsp_types::ServerCapabilities;

mod main_loop;

pub fn main() -> Result<()> {
    flexi_logger::Logger::with_str("info").start().unwrap();
    log::info!("starting language server");

    let (connection, io_threads) = Connection::stdio();
    log::info!("Transport is created, stdin and stdout are connected");

    let server_capabilities = serde_json::to_value(&ServerCapabilities::default()).unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    log::info!("Initialization is finished");

    main_loop::main_loop(&connection, initialization_params)?;
    io_threads.join()?;

    log::info!("shutting down server");
    Ok(())
}
