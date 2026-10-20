use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub bind_address: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let is_production = env::var("APP_ENV")
            .map(|value| value.eq_ignore_ascii_case("production"))
            .unwrap_or(false);

        // Load .env for local development only.
        if !is_production {
            let _ = dotenvy::dotenv();
        }

        let database_url = required("DATABASE_URL")?;
        let jwt_secret = required("JWT_SECRET")?;

        if jwt_secret.len() < 32 || jwt_secret.trim() != jwt_secret {
            return Err(
                "JWT_SECRET must be at least 32 bytes and have no surrounding whitespace"
                    .to_string(),
            );
        }

        let port = match env::var("PORT") {
            Ok(value) => {
                let parsed = value
                    .parse::<u16>()
                    .map_err(|_| "PORT must be an integer from 1 to 65535".to_string())?;

                if parsed == 0 {
                    return Err("PORT must be from 1 to 65535".to_string());
                }

                parsed
            }
            Err(env::VarError::NotPresent) => 3000,
            Err(_) => return Err("PORT contains invalid Unicode".to_string()),
        };

        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

        if bind_address.trim().is_empty() {
            return Err("BIND_ADDRESS must not be empty".to_string());
        }

        Ok(Self {
            database_url,
            jwt_secret,
            port,
            bind_address,
        })
    }
}

fn required(name: &str) -> Result<String, String> {
    let value = env::var(name).map_err(|_| format!("{name} must be set"))?;

    if value.trim().is_empty() {
        return Err(format!("{name} must not be empty"));
    }

    Ok(value)
}
