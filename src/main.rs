use lambda_http::{lambda, Body, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};
use lettre::smtp::authentication::Credentials;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, SmtpTransport, Transport};
use log::{error, info};
use serde::Deserialize;

use std::env::var;

#[derive(Deserialize, Clone, Debug)]
struct Email {
    name: String,
    email: String,
    message: String,
}

fn handler(request: Request, _ctx: Context) -> Result<impl IntoResponse, HandlerError> {
    info!("Request: {:?}", request);

    if let Body::Text(body) = request.body() {
        let email = serde_json::from_str(&body).expect("failed to parse body");

        let from_email_address = var("USERNAME").expect("Issue grabbing email");

        let email = create_email(email, from_email_address.clone());
        let mut transport = setup_transport(from_email_address);

        return match transport.send(email) {
            Ok(_) => {
                transport.close();

                Ok(Response::builder()
                    .status(200)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(
                        serde_json::to_string(
                            "Thanks for your message, I'll endeavor to respond within 48 hours.",
                        )
                        .unwrap(),
                    )
                    .unwrap())
            }
            Err(_) => {
                transport.close();
                Err("unable to send email".into())
            }
        };
    }

    error!(
        "Expected request body of type Body::Text(_) instead got {:?}",
        request.body()
    );
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
        format!(
            "\nA request to contact: {} \n\n Email address: {} \n\n Message Start: \n\n{}",
            email.name, email.email, email.message
        )
        .into_bytes(),
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
    lambda!(handler);
    Ok(())
}
