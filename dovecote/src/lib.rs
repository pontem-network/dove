#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

use crate::rpc::Rpc;
use anyhow::Error;
use std::sync::Arc;

pub mod rpc;
pub mod bg;

#[derive(Debug, Clone)]
pub struct State {
    pub rpc: Arc<Rpc>,
}

impl State {
    pub fn new() -> Result<State, Error> {
        Ok(State {
            rpc: Arc::new(Rpc::new()?),
        })
    }
}
