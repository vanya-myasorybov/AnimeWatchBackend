use crate::{
    config::EmailConfig,
    errors::{AppError, Result},
};
use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Message,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};

#[derive(Clone)]
pub struct EmailService {
    transport: AsyncSmtpTransport<lettre::Tokio1Executor>,
    from_mailbox: Mailbox,
}

impl EmailService {
    pub fn new(
        EmailConfig {
            from_email,
            from_name,
            username,
            password,
            smtp_host,
            smtp_port,
        }: EmailConfig,
    ) -> Result<Self> {
        let from_address = from_email
            .parse::<Address>()
            .map_err(|e| AppError::Internal(format!("Invalid from email: {}", e)))?;

        let from_mailbox = Mailbox::new(Some(from_name.to_string()), from_address);

        let creds = Credentials::new(username.to_string(), password.to_string());

        let transport = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&smtp_host)
            .map_err(|e| AppError::Internal(format!("Invalid SMTP host: {}", e)))?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(EmailService {
            transport,
            from_mailbox,
        })
    }

    pub async fn send_html(&self, to_email: &str, html_body: &str) -> Result<()> {
        let to_address = to_email
            .parse::<Address>()
            .map_err(|e| AppError::InvalidAddress(format!("Invalid recipient email: {}", e)))?;

        let email = Message::builder()
            .from(self.from_mailbox.clone())
            .to(Mailbox::new(None, to_address))
            .header(ContentType::TEXT_HTML)
            .body(html_body.to_string())
            .map_err(|e| AppError::BuildError(format!("Failed to build email: {}", e)))?;

        self.transport
            .send(email)
            .await
            .map_err(|e| AppError::SendError(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}
