use crate::configs::env::{load_config, Config};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: load_config(),
        }
    }
}
