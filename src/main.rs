use lambda_runtime::{error::HandlerError, lambda, Context};
use lettre::smtp::authentication::Credentials;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, SmtpTransport, Transport};
use log::info;
use serde::{Deserialize, Serialize};

use std::env::var;

#[derive(Deserialize, Clone, Debug)]
struct EmailEvent {
    body: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Email {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Clone)]
struct EmailResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

impl EmailResponse {
    fn new(status_code: u16, body: String) -> Self {
        Self {
            is_base64_encoded: false,
            status_code,
            body: serde_json::to_string(&body).unwrap(),
        }
    }
}

fn lambda_handler(event: EmailEvent, _context: Context) -> Result<EmailResponse, HandlerError> {
    info!("{:?}", event);
    if let Some(payload) = event.body {
        let email: Email = serde_json::from_str(&payload)?;

        let from_email_address = var("USERNAME").expect("Issue grabbing email");

        let email = create_email(email, from_email_address.clone());
        let mut transport = setup_transport(from_email_address);

        return match transport.send(email) {
            Ok(_) => {
                transport.close();
                Ok(EmailResponse::new(
                    200,
                    "Thanks for your message, I'll endeavor to respond within 48 hours."
                        .to_string(),
                ))
            }
            Err(_) => {
                transport.close();
                Err("unable to send email".into())
            }
        };
    }
    Err("unable to send email".into())
}

fn create_email(email: Email, from_email_address: String) -> SendableEmail {
    let to_email_address = var("DESTINATION").expect("Issue with destination");

    SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(from_email_address).unwrap()),
            vec![EmailAddress::new(to_email_address).unwrap()],
        )
        .expect("Issue creating envelope"),
        format!("Contact Me: {}", email.name.clone()),
        format!("Message sent from {} \n\n {}", email.email, email.message).into_bytes(),
    )
}

fn setup_transport(from_email_address: String) -> SmtpTransport {
    SmtpClient::new_simple(
        var("MAIL_CLIENT")
            .expect("Issue unwrapping mail client")
            .as_str(),
    )
    .expect("Error creating client")
    .credentials(Credentials::new(
        from_email_address,
        var("PASSWORD").expect("Issue finding password"),
    ))
    .transport()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}
