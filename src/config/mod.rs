use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub redis_url: String,
    pub email_config: EmailConfig,
}
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_expiration: env::var("")
                .unwrap_or_else(|_| "31536000".to_string())
                .parse()
                .unwrap_or(31536000),
            redis_url: env::var("REDIS_URL")?,
            email_config: EmailConfig {
                smtp_host: env::var("SMTP_HOST")?,
                smtp_port: env::var("SMTP_PORT")
                    .unwrap_or_else(|_| "1212".to_string())
                    .parse()
                    .unwrap_or(1212),
                username: env::var("USERNAME")?,
                password: env::var("PASSWORD")?,
                from_email: env::var("FROM_EMAIL")?,
                from_name: env::var("FROM_NAME")?,
            },
        })
    }
}
