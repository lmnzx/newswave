use std::env;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::config::Settings;

pub async fn send_email(to: String, subject: String, body: String) {
    let mode = env::var("MODE").unwrap_or_else(|_| "development".into());

    let s = Settings::get_config().expect("Failed to load configuration");

    let s = s.smtp;

    let from_email = s.from_email; // will come from config file

    // ! DO NOT COMMIT THIS KEY
    let smtp_key = s.key;

    // ! DO NOT COMMIT THIS HOST
    let host = s.host;

    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .reply_to(from_email.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    let creds = Credentials::new("".to_string(), smtp_key.to_string());

    let mailer: AsyncSmtpTransport<Tokio1Executor>;

    if mode == "development" {
        mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();
    } else {
        mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
            .unwrap()
            .credentials(creds)
            .build();
    }

    match mailer.send(email).await {
        Ok(_) => tracing::info!("Email sent successfully"),
        Err(e) => tracing::error!("Error sending email: {:?}", e),
    }
}
