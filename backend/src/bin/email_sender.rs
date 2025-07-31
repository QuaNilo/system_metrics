use chrono::Utc;
use system_metrics::utils::emailer::{MAILER, EmailBody};
use clap::{Parser, Subcommand};
use system_metrics::config::Settings;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Handle SSH login event
    SshLogin {
        /// Username who connected via SSH
        user: String,
        /// IP address of the SSH connection
        ip: String,
        /// Date/time of login
        date: String,
    },
    /// Handle other types of events
    Other {
        /// The type of event
        #[arg(value_name = "type")]
        arg_type: String,
    },
}

fn main() {
    let config: Settings = Settings::new();
    let cli = Cli::parse();
    match cli.command{
        Commands::SshLogin { user, ip, date } => {
            println!("SSH login event: {} from {} on {}", user, ip, date);
            match MAILER.send_email(
                &config.emailer.to_address,
                &format!("SSH Login at {}", Utc::now().format("%Y-%m-%d %H:%M:%S")),
                &EmailBody::ssh_login(&user, &ip, &date)
            ) {
                Ok(_) => println!("Email sent successfully"),
                Err(e) => println!("Error sending email: {}", e)
            }
        },
        Commands::Other { arg_type } => {
            println!("Other event: {}", arg_type);
        }
    }
    
}