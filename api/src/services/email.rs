use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::err::AppErr;

use crate::templates::{subject, template};

pub struct EmailService {
    server_email: String,
    auth_token: String,
    smtp_relay: String,
}

impl EmailService {
    pub fn from_env() -> Self {
        Self {
            server_email: std::env::var("SERVER_EMAIL").expect("SERVER_EMAIL NOT SET"),
            auth_token: std::env::var("SERVER_EMAIL_AUTH_TOKEN")
                .expect("SERVER_EMAIL_AUTH_TOKEN NOT SET"),
            smtp_relay: std::env::var("SMTP_TRANSPORT_RELAY")
                .expect("SMTP_TRANSPORT_RELAY NOT SET"),
        }
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), AppErr> {
        let email = Message::builder()
            .from(
                self.server_email
                    .parse()
                    .map_err(|_| AppErr::ParseEmailError)?,
            )
            .to(to.parse().map_err(|_| AppErr::ParseEmailError)?)
            .subject(subject.to_owned())
            .body(body.to_owned())
            .map_err(|_| AppErr::CreateEmailFailed)?;

        SmtpTransport::relay(&self.smtp_relay)
            .unwrap()
            .credentials(self.credentials())
            .build()
            .send(&email)
            .map(|_| ())
            .map_err(|err| AppErr::SendEmailFailed(err.to_string()))
    }

    fn credentials(&self) -> Credentials {
        Credentials::new(self.server_email.clone(), self.auth_token.clone())
    }

    pub fn send_vcode(&self, to: &str, code: &str) -> Result<(), AppErr> {
        self.send_email(to, &subject(), &template(code))
    }
}

#[test]
fn send_email_test() {
    dotenv::dotenv().unwrap();
    assert!(EmailService::from_env()
        .send_email(
            "example@qq.com",
            "Mid night",
            "have a good sleep"
        )
        .is_ok());
}
