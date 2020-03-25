use anyhow::Result;

use move_language_server::server;

pub fn main() -> Result<()> {
    env_logger::init();
    log::info!("starting language server");

    server::run_server()?;

    log::info!("shutting down server");
    Ok(())
}
