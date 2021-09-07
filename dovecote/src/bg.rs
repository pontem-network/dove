use crate::State;
use async_std::task;
use std::time::Duration;
use actix_web::web;
use anyhow::Error;

const CLEAN_RATE: Duration = Duration::from_secs(60 * 5);

pub async fn clean_project(state: State) {
    loop {
        task::sleep(CLEAN_RATE).await;
        let state = state.clone();
        web::block::<_, _, Error>(move || {
            state.rpc.projects.clean_up();
            Ok(())
        }).await.expect("Unreachable.");
    }
}