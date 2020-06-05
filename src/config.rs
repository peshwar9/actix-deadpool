// Standard lib
// External crates - Primary
use config::ConfigError;
// External crates - Utilities
use serde::Deserialize;
// Other internal modules
// Const and type declarations
// Struct declarations

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub db: deadpool_postgres::Config,
}

// Functions

impl Config {
    pub fn env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
