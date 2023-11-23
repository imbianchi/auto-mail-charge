pub mod email;
pub mod screenshot_exchange;
pub mod usd_exchange;
pub mod vars;

fn main() {
    let usd_rate = usd_exchange::get_usd_exchange_rate().expect("Error getting USD rate.");

    screenshot_exchange::get_screenshot_usd_brl().expect("Error getting screenshot.");

    email::send_email(usd_rate).expect("Error sending the email");
}
