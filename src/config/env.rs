use std::env;

#[derive(Debug, Clone)]
pub struct EnvData {
    pub database_url: String,
    pub secret_key: String,
    pub port: u16,
}

impl EnvData {
    pub fn new() -> Result<EnvData, String> {
        dotenvy::dotenv().ok();

        Ok(EnvData {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set".to_string())?,
            secret_key: env::var("SECRET_KEY").map_err(|_| "SECRET_KEY must be set".to_string())?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .map_err(|_| "PORT must be a valid number".to_string())?,
        })
    }
}
