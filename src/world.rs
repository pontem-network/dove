use crate::config::MoveDialect;

#[derive(Debug)]
pub struct Config {
    pub dialect: MoveDialect,
}

#[derive(Debug)]
pub struct WorldState {
    pub config: Config,
}

impl WorldState {
    pub fn new(config: Config) -> Self {
        WorldState { config }
    }

    pub fn update_configuration(&mut self, config: Config) {
        self.config = config;
    }
}
