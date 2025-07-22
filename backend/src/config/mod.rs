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
pub struct CronJob{
    pub interval: u64,    
}

impl CronJob {
    fn from_env() -> Self {
        CronJob {
            interval: env::var("CRONJOB_INTERVAL_SECS")
                .expect("CRON_JOB_INTERVAL must be set")
                .parse()
                .expect("CRON_JOB_INTERVAL must be a number")
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
pub struct Settings {
    pub postgres: Postgres,
    pub iagon: Iagon,
    pub cronjob: CronJob,   
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            postgres: Postgres::from_env(),
            iagon: Iagon::from_env(),
            cronjob: CronJob::from_env(),       
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