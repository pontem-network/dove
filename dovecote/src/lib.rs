use crate::rpc::Rpc;
use anyhow::Error;
use std::sync::Arc;

pub mod rpc;

#[derive(Debug, Clone)]
pub struct State {
    pub rpc: Arc<Rpc>,
}

impl State {
    pub fn new() -> Result<State, Error> {
        Ok(State {
            rpc: Arc::new(Rpc::new()?)
        })
    }
}

