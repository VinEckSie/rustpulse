// src/config.rs
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenvy::dotenv().ok(); // loads .env or .env.test
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            port: std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
        })
    }
}
