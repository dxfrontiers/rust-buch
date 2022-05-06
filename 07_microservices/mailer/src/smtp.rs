use crate::config::MailerConfig;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub fn prepare_transport(config: &MailerConfig) -> SmtpTransport {
    let creds =
        Credentials::new(config.smtp_user.clone(), config.smtp_password.clone());

        return SmtpTransport::relay(config.smtp_host.as_str())
            .expect(format!("Could not connect to smtp host: {}", config.smtp_host).as_str())
            .credentials(creds)
            .build();
}