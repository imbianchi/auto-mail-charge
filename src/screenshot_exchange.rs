use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

pub fn get_screenshot_usd_brl() -> Result<(), Box<dyn std::error::Error>> {
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
