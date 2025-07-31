use lettre::{SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use lettre::Message;
use std::sync::Arc;
use chrono::Utc;
use once_cell::sync::Lazy;
use crate::config::get_settings;

pub struct Mailer {
    sender: String,
    mailer: SmtpTransport
}

pub struct EmailBody;

impl EmailBody {
    pub fn ssh_login(user: &str, ip: &str, date: &str) -> String {
        format!(
            "SSH Login detected!\nUser: {}\nIP Address: {}\nDate: {}\n",
            user, ip, date
        )
    }

    pub fn node_down() -> String {
        format!(
            "Node is down!\nDate: {}\n\nTo fix access server and run './iag-cli-linux start'",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        )
    }
}

impl Mailer {
    pub fn new(sender: &str, app_password: &str) -> Self {
        let creds = Credentials::new(sender.to_string(), app_password.to_string());

        let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        Self {
            sender: sender.to_string(),
            mailer,
        }
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let email = Message::builder()
            .from(self.sender.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(body.to_string())
            .unwrap();

        match self.mailer.send(&email) {
            Ok(_) => {
                println!("✅ Email sent");
                Ok(())
            },
            Err(e) => {
                eprintln!("❌ Failed: {:?}", e);
                Err(e.to_string())
            },
        }
    }
}
pub static MAILER: Lazy<Arc<Mailer>> = Lazy::new(|| {
    let config = get_settings();
    Arc::new(Mailer::new(&config.emailer.from_address, &config.emailer.app_password))
});