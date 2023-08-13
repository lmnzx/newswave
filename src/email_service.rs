use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub async fn send_email(to: String, subject: String, body: String) {
    let from_email = ""; // will come from config file

    // ! DO NOT COMMIT THIS KEY
    let smtp_key = "";

    // ! DO NOT COMMIT THIS HOST
    let host = "";

    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .reply_to(from_email.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    let creds = Credentials::new("".to_string(), smtp_key.to_string());

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
            .unwrap()
            .credentials(creds)
            .build();

    match mailer.send(email).await {
        Ok(_) => tracing::info!("Email sent successfully"),
        Err(e) => tracing::error!("Error sending email: {:?}", e),
    }
}
