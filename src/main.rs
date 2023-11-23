pub mod email;
pub mod vars;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use std::process::Command;
use serde_json::Value;

fn get_usd_exchange_rate() -> Result<f64, Box<dyn std::error::Error>> {    
    let output = Command::new("curl")
        .arg("https://open.er-api.com/v6/latest/USD")
        .output()?;

    if output.status.success() {
        let response: Value = serde_json::from_slice(&output.stdout)?;
        let usd_exchange_rate = response["rates"]["BRL"]
            .as_f64()
            .ok_or("BRL rate not found")?;

        Ok((usd_exchange_rate * 1000.0).round() / 1000.0)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.into())
    }
}

fn get_screenshot_usd_brl() -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let google_usd_today = "https://www.google.com/search?q=dolar+hoje&oq=dolar+hoje";

    tab.navigate_to(google_usd_today)?;
    tab.wait_until_navigated()?;

    let screenshot_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    std::fs::write("screenshot.png", &screenshot_data)?;

    Ok(())
}

fn main() {
    let usd_rate = get_usd_exchange_rate().expect("Error getting USD rate.");

    get_screenshot_usd_brl().expect("Error getting screenshot.");

    email::send_email(usd_rate).expect("Error sending the email");
}
