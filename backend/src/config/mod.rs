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
pub struct Emailer{
    pub from_address: String,
    pub to_address: String,
    pub app_password: String,
}

impl Emailer {
    fn from_env() -> Self {
        Emailer {
            from_address: env::var("EMAILER_FROM_ADDRESS").expect("EMAILER_FROM_ADDRESS must be set"),
            to_address: env::var("EMAILER_TO_ADDRESS").expect("EMAILER_TO_ADDRESS must be set"),
            app_password: env::var("EMAILER_APP_PASSWORD").expect("EMAILER_APP_PASSWORD must be set"),
        }
    }
}

#[derive(Debug)]
pub struct CronJob{
    pub interval: u64,
    pub enabled: bool,   
}

impl CronJob {
    fn from_env() -> Self {
        CronJob {
            interval: env::var("CRONJOB_INTERVAL_SECS")
                .expect("CRON_JOB_INTERVAL must be set")
                .parse()
                .expect("CRON_JOB_INTERVAL must be a number"),
            enabled: env::var("CRONJOB_ENABLED").expect("CRONJOB_ENABLED must be set") == "true",
        }
    }   
}

#[derive(Debug)]
pub struct Iagon{
    pub cli_path: String,
    pub get_info: String,
    pub get_node_status: String,
}

impl Iagon {
    fn from_env() -> Self {
        Iagon {
            cli_path: env::var("IAGON_CLI_PATH").expect("IAGON_CLI_PATH must be set"),
            get_info: env::var("GET_INFO").unwrap_or("false".to_string()),
            get_node_status: env::var("GET_NODE_STATUS").unwrap_or("false".to_string())
        }
    }
}

#[derive(Debug)]
pub struct App{
    pub secret: String,
    pub auth_enabled: bool,
    pub http_secure: bool,
}

impl App {
    fn from_env() -> Self {
        App {
            secret: env::var("APP_SECRET").expect("APP_SECRET must be set"),
            auth_enabled: env::var("APP_AUTH_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true)
                ||
                env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT must be set") != "dev",
            http_secure: env::var("APP_HTTP_SECURE")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>().unwrap_or(true)
                || 
                env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT must be set") != "dev",
        }
    }
}

#[derive(Debug)]
pub struct Logging {
    pub rust_log: String,
    pub rust_log_file: String,
}

impl Logging {
    fn from_env() -> Self {
        Logging {
            rust_log: env::var("RUST_LOG").expect("RUST_LOG env var must be set"),
            rust_log_file: env::var("RUST_LOG_FILE").expect("RUST_LOG_FILE env var must be set"),
        }   
    }
}

#[derive(Debug)]
pub struct Settings {
    pub postgres: Postgres,
    pub iagon: Iagon,
    pub cronjob: CronJob,
    pub app: App,
    pub logging: Logging,   
    pub emailer: Emailer, 
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            postgres: Postgres::from_env(),
            iagon: Iagon::from_env(),
            cronjob: CronJob::from_env(),
            app: App::from_env(),
            logging: Logging::from_env(),
            emailer: Emailer::from_env(),
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