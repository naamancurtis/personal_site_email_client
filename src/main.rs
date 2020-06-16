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
    let env_origin = var("ORIGIN").expect("Issue finding origin");

    match request.headers().get("origin") {
        Some(origin) => {
            let split_origin: Vec<&str> = origin.to_str().unwrap().split("//").collect();
            // Website is always served over https, if the origin isn't, then deny the request
            if split_origin[0] != "https:" {
                return Ok(generate_response(403));
            }

            // Check the origin is what is expected
            if split_origin[1] != env_origin && split_origin[1] != format!("www.{}", env_origin) {
                return Ok(generate_response(403));
            }
        }
        _ => return Ok(generate_response(403)),
    }

    if let Body::Text(body) = request.body() {
        let email = serde_json::from_str(&body).expect("failed to parse body");

        let from_email_address = var("USERNAME").expect("Issue grabbing email");

        let email = create_email(email, from_email_address.clone());
        let mut transport = setup_transport(from_email_address);

        return match transport.send(email) {
            Ok(_) => {
                transport.close();

                Ok(generate_response(200))
            }
            Err(_) => {
                transport.close();
                Ok(generate_response(500))
            }
        };
    }

    error!(
        "Expected request body of type Body::Text(_) instead got {:?}",
        request.body()
    );
    Ok(generate_response(400))
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

/// Only interested in the status code, no endpoints
/// actually return any data
fn generate_response(status_code: u16) -> Response<()> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .status(status_code)
        .body(())
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}
