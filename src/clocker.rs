use failure::Fallible;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
use std::fs;

pub fn test() -> Fallible<()> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    let jpeg_data = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;

    println!("Screenshots successfully created.");
    Ok(())
}
