
pub struct Config {
    pub db_url: String,
    pub log_json: bool,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            db_url: std::env::var("DATABASE_URL")?,
            log_json: std::env::var("LOG_JSON").map(|mut v| v == "1").unwrap_or(false),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
        })
    }
}