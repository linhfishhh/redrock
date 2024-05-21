use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub host: String,
    pub pg: deadpool_postgres::Config
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0:8080".into(),
            pg: deadpool_postgres::Config::default()
        }
    }
}

impl Settings {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .build()?
            .try_deserialize()
    }
}