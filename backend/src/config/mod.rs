use std::env;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Postgres{
    pub connection_string: String,
}

impl Postgres {
    fn from_env() -> Self {
        Postgres {
            connection_string: env::var("METRICS_POSTGRES_CONNECTION_STRING")
                .expect("METRICS_POSTGRES_CONNECTION_STRING must be set")
        }
    }
}


#[derive(Debug)]
pub struct Settings {
    pub postgres: Postgres
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            postgres: Postgres::from_env()
        }
    }
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init_settings() {
    SETTINGS.get_or_init(Settings::new);
}

pub fn get_settings() -> &'static Settings {
    SETTINGS.get().expect("Settings not initialized. Call init_settings() first.")
}