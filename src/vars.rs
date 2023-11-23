extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Vars {
    pix_number: String,
    charge_value: String,
    email_recipients: String,
    email_subject: String,
    email_signature: String,
    email_pswd: String,
    smtp: String,
    email_sender: String,
}

impl Vars {
    pub fn default() -> Result<Self, std::io::Error> {
        dotenv().ok();

        Ok(Self {
            pix_number: env::var("PIX_NUMBER").expect("Variable not found"),
            charge_value: env::var("CHARGE_VALUE").expect("Variable not found"),
            email_recipients: env::var("EMAIL_RECIPIENTS").expect("Variable not found"),
            email_subject: env::var("EMAIL_SUBJECT").expect("Variable not found"),
            email_signature: env::var("EMAIL_SIGNATURE").expect("Variable not found"),
            email_pswd: env::var("EMAIL_PSWD").expect("Variable not found"),
            smtp: env::var("SMTP").expect("Variable not found"),
            email_sender: env::var("EMAIL_SENDER").expect("Variable not found"),
        })
    }

    pub fn get_var(&self, var_name: &str) -> &str {
        match var_name {
            "PIX_NUMBER" => &self.pix_number,
            "CHARGE_VALUE" => &self.charge_value,
            "EMAIL_RECIPIENTS" => &self.email_recipients,
            "EMAIL_SUBJECT" => &self.email_subject,
            "EMAIL_SIGNATURE" => &self.email_signature,
            "EMAIL_PSWD" => &self.email_pswd,
            "SMTP" => &self.smtp,
            "EMAIL_SENDER" => &self.email_sender,
            _ => "Error"
        }
    }
}
