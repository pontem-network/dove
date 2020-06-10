use anyhow::Result;

use move_language_server::server;

build_info::build_info!(fn crate_build_info);

pub fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting language server");

    let build_info = crate_build_info();
    log::info!("Compiled at: {}", build_info.timestamp);
    log::info!(
        "Git: {:#?}",
        build_info.version_control.as_ref().unwrap().git().unwrap()
    );

    server::run_server()?;

    log::info!("Shutting down server");
    Ok(())
}
