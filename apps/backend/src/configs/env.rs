use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
}

pub fn load_config() -> Config {
    envy::from_env().unwrap()
}
