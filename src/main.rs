use lambda_runtime::{error::HandlerError, lambda, Context};
use lettre::smtp::authentication::Credentials;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};
use serde::{Deserialize, Serialize};

use std::env::var;

#[derive(Deserialize)]
pub struct EmailEvent {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize)]
struct EmailResponse {
    message: String,
}

fn lambda_handler(payload: EmailEvent, _context: Context) -> Result<EmailResponse, HandlerError> {
    let from_email_address = var("USERNAME").expect("Issue grabbing email");
    let to_email_address = var("DESTINATION").expect("Issue with destination");
    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(from_email_address.clone()).unwrap()),
            vec![EmailAddress::new(to_email_address).unwrap()],
        )
        .expect("Issue creating envelope"),
        format!("Contact Me: {}", payload.name.clone()),
        format!("Message sent from {} \n {}", payload.email, payload.message).into_bytes(),
    );

    let mut transport = SmtpClient::new_simple(
        var("MAIL_CLIENT")
            .expect("Issue unwrapping mail client")
            .as_str(),
    )
    .expect("Error creating client")
    .credentials(Credentials::new(
        from_email_address,
        var("PASSWORD").unwrap_or_else(|_| "password".to_string()),
    ))
    .transport();

    match transport.send(email) {
        Ok(_) => {
            transport.close();
            Ok(EmailResponse {
                message: "Thanks for your email, I'll endeavor to get back to you within 48 hours."
                    .to_string(),
            })
        }
        Err(_) => {
            transport.close();
            Err("unable to send email".into())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}
