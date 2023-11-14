use dotenv::dotenv;

use lettre::message::header::ContentType;
use lettre::message::{header, Attachment, Body, Mailboxes, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use std::fs;
use std::process::Command;

use serde_json::Value;

fn get_usd_exchange_rate() -> Result<f64, Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .arg(std::env::var("URL_API_USD").unwrap())
        .output()?;

    if output.status.success() {
        let response: Value = serde_json::from_slice(&output.stdout)?;
        let usd_exchange_rate = response["rates"]["BRL"]
            .as_f64()
            .ok_or("BRL rate not found")?;

        let charge_value: String =
            std::env::var("CHARGE_VALUE").expect("CHARGE_VALUE must be set.");

        Ok(usd_exchange_rate * charge_value.parse::<f64>().unwrap())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.into())
    }
}

fn get_screenshot_usd_brl() -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let url_google = std::env::var("URL_USD_GOOGLE").unwrap();

    tab.navigate_to(&url_google)?;
    tab.wait_until_navigated()?;

    let screenshot_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    std::fs::write("screenshot.png", &screenshot_data)?;

    Ok(())
}

fn send_email(usd_converted: f64) -> Result<(), Box<dyn std::error::Error>> {
    let body = "Hello, this is a test email from Rust!";

    let sender = std::env::var("EMAIL_SENDER").unwrap();
    let pswd = std::env::var("EMAIL_PSWD").unwrap();
    let smtp = std::env::var("SMTP").unwrap();

    let to_addresses = std::env::var("EMAIL_RECIPIENTS").unwrap();
    let to_addresses: Mailboxes = to_addresses.parse().unwrap();
    let to_header: header::To = to_addresses.into();

    let subject = std::env::var("EMAIL_SUBJECT").unwrap();

    let image = fs::read("screenshot.png")?;
    let image_body = Body::new(image);

    let email = Message::builder()
        .mailbox(to_header)
        .from(sender.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::mixed()
                .multipart(
                    MultiPart::alternative()
                        .singlepart(SinglePart::plain(String::from("Hello, world! :)")))
                        .multipart(
                            MultiPart::related()
                                .singlepart(SinglePart::html(String::from(
                                    "<p><b>Hello</b>, <i>world</i>! <img src=cid:123></p>",
                                )))
                                .singlepart(
                                    Attachment::new_inline(String::from("123"))
                                        .body(image_body, "image/png".parse().unwrap()),
                                ),
                        ),
                )
                .singlepart(Attachment::new(String::from("example.rs")).body(
                    String::from("fn main() { println!(\"Hello, World!\") }"),
                    "text/plain".parse().unwrap(),
                )),
        )?;

    let creds = Credentials::new(sender.to_owned(), pswd.to_owned());

    let mailer = SmtpTransport::relay(&smtp)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email Sent!"),
        Err(e) => println!("Error sending email: {:?}", e),
    }

    Ok(())
}

fn main() {
    dotenv().ok();

    let usd_converted = get_usd_exchange_rate().expect("Error trying to convert USD");

    get_screenshot_usd_brl().expect("Error when getting screenshot.");

    send_email(usd_converted).expect("Error sending the email");
}
