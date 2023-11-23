use crate::vars::Vars;

use chrono::prelude::*;

use lettre::message::{header, Attachment, Body, Mailboxes, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use std::fs;

pub fn send_email(usd_rate: f64) -> Result<(), Box<dyn std::error::Error>> {
    let vars = Vars::default().expect("ERROR");

    let charge_value = vars.get_var("CHARGE_VALUE").parse::<f64>().unwrap();

    let now: DateTime<Local> = Local::now();
    let last_month_u32 = now.month() - 1;
    let current_year_str = now.format("%Y").to_string();

    let dt = Utc.with_ymd_and_hms(now.year(), last_month_u32, now.day(), 0, 0, 0).unwrap();
    let last_month_str = dt.format_localized("%B", Locale::pt_BR).to_string().to_uppercase();

    let to_addresses: Mailboxes = vars.get_var("EMAIL_RECIPIENTS").parse().unwrap();
    let to_header: header::To = to_addresses.into();

    let subject = format!(
        "{}- {}/{}",
        &vars.get_var("EMAIL_SUBJECT"),
        &last_month_u32,
        &current_year_str
    );

    let total_value = &charge_value * &usd_rate;
    let total_value = (total_value * 100.0).round() / 100.0;

    let pix_number = vars.get_var("PIX_NUMBER");
    let email_sign = vars.get_var("EMAIL_SIGNATURE");

    let body = format!(
        "
            <div>
                Bom dia a todos,
                <br/>
                <br/>
                Valor a ser depositado referente ao mês de <b>{last_month_str} de {current_year_str}</b>.<br/>
                Servidor & Hospedagem: <b>${charge_value} x R${usd_rate} = R${total_value}</b>.<br/>
                <p>
                    <img style='width: 450px; height: 340px;' src=cid:monthly-charge />
                </p>
                <b>Favor efetuar pagamento no PIX: {pix_number}
                </b>
                <br/>
                <br/>
                Atenciosamente,<br/>
                {email_sign}
            </div>
        "
    );

    let image = fs::read("screenshot.png")?;
    let image_body = Body::new(image);

    let email_sender = vars.get_var("EMAIL_SENDER");

    let email = Message::builder()
        .mailbox(to_header)
        .from(email_sender.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::mixed().multipart(
                MultiPart::alternative().multipart(
                    MultiPart::related()
                        .singlepart(SinglePart::html(String::from(body)))
                        .singlepart(
                            Attachment::new_inline(String::from("monthly-charge"))
                                .body(image_body, "image/png".parse().unwrap()),
                        ),
                ),
            ),
        )?;

    let creds = Credentials::new(
        email_sender.to_owned(),
        vars.get_var("EMAIL_PSWD").to_owned(),
    );

    let mailer = SmtpTransport::relay(vars.get_var("SMTP"))
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {},
        Err(e) => println!("Error sending email: {:?}", e),
    }

    Ok(())
}
