use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
}

pub fn load_env() -> Config {
    envy::from_env().unwrap()
}
