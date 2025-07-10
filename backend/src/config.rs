use dotenvy::dotenv;

pub struct Config {
    //pub db_url: String,
    pub log_json: bool,
    pub port: u16,
}

impl Config {
    #[allow(clippy::manual_inspect)]
    pub fn from_env() -> Result<Self, std::env::VarError> {
        // Initialize dotenv within the method
        dotenv().ok();

        Ok(Self {
            //db_url: std::env::var("DATABASE_URL")?,
            log_json: std::env::var("LOG_JSON").map(|v| v == "1").unwrap_or(false),
            port: std::env::var("PORT")
                .map_err(|e| {
                    eprintln!("Port not set in the config file.");
                    e
                })
                .and_then(|port| {
                    port.parse::<u16>().map_err(|_| {
                        eprintln!("failed to parse the port as valid number: ");
                        std::env::VarError::NotPresent
                    })
                })
                .unwrap_or_else(|_| {
                    eprintln!("Defaulting to 3000");
                    3000
                }),
        })
    }
}
