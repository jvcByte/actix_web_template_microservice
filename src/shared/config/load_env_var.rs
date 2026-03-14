use std::env;
use std::sync::OnceLock;

/// Authentication configuration — loaded once at startup via `init()`.
#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub secret: String,
    pub access_exp_minutes: i64,
    pub refresh_exp_days: i64,
}

static AUTH_CONFIG: OnceLock<AuthConfig> = OnceLock::new();

impl AuthConfig {
    /// Call once at application startup (in `main`). Panics if required vars are missing.
    pub fn init() {
        let secret = env::var("JWT_SECRET").expect(".env: JWT_SECRET must be set");

        let access_exp_minutes = match env::var("JWT_EXP_MINUTES") {
            Ok(v) => v.parse::<i64>().unwrap_or_else(|_| {
                eprintln!("WARNING: Invalid JWT_EXP_MINUTES, defaulting to 15");
                15
            }),
            Err(_) => 15,
        };

        let refresh_exp_days = match env::var("REFRESH_TOKEN_EXP_DAYS") {
            Ok(v) => v.parse::<i64>().unwrap_or_else(|_| {
                eprintln!("WARNING: Invalid REFRESH_TOKEN_EXP_DAYS, defaulting to 30");
                30
            }),
            Err(_) => 30,
        };

        AUTH_CONFIG
            .set(AuthConfig {
                secret,
                access_exp_minutes,
                refresh_exp_days,
            })
            .expect("AuthConfig already initialized");
    }

    /// Get the global config. Panics if `init()` was not called first.
    pub fn get() -> &'static AuthConfig {
        AUTH_CONFIG
            .get()
            .expect("AuthConfig not initialized — call AuthConfig::init() at startup")
    }
}
