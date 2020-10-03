use anyhow::Result;

use lsp_server::Connection;
use move_language_server::server;

// build_info::build_info!(fn crate_build_info);

pub fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting language server");

    // let build_info = crate_build_info();
    // log::info!("Compiled at: {}", build_info.timestamp);
    // log::info!(
    //     "Git: {:#?}",
    //     build_info.version_control.as_ref().unwrap().git().unwrap()
    // );

    let (connection, io_threads) = Connection::stdio();
    log::info!("Transport is created, stdin and stdout are connected");

    server::run_server(&connection)?;

    io_threads.join()?;
    log::info!("Shutting down server");
    Ok(())
}
