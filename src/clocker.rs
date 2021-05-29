// use dotenv;
use failure::Fallible;
pub use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptions, Tab};
use std::sync::Arc;

// let usrname: &str = dotenv::var("CNC_USR").unwrap().as_str();
// let pw: &str = dotenv::var("CNC_PW").unwrap().as_str();
const USRNAME: &'static str = "johnkangsumrith";
const PW: &'static str = "";

pub fn get_clock_status() -> Fallible<bool> {
    let browser: Browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://www.cnc-claimsource.com/")?;
    tab.wait_until_navigated()?;

    //Login
    tab.wait_for_element("input#username")?.click()?;
    tab.type_str(USRNAME)?;
    tab.wait_for_element("input#password")?.click()?;
    tab.type_str(PW)?;
    tab.wait_for_element("input#loginBtn")?.click()?;
    tab.wait_for_element("div#clockbox")?;

    //Clock Page
    tab.navigate_to("https://www.cnc-claimsource.com/sf_clock.php?did=615")?;
    tab.wait_until_navigated()?;

    let status = tab.wait_for_element("input#clockin");
    let status = match status {
        Ok(_) => false,
        Err(_) => true,
    };

    Ok(status)
}

pub fn do_clock_event() -> Fallible<bool> {
    let browser: Browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://www.cnc-claimsource.com/")?;
    tab.wait_until_navigated()?;

    //Login
    tab.wait_for_element("input#username")?.click()?;
    tab.type_str(USRNAME)?;
    tab.wait_for_element("input#password")?.click()?;
    tab.type_str(PW)?;
    tab.wait_for_element("input#loginBtn")?.click()?;
    tab.wait_for_element("div#clockbox")?;

    //Clock Page
    tab.navigate_to("https://www.cnc-claimsource.com/sf_clock.php?did=615")?;
    tab.wait_until_navigated()?;

    let tag = tab.wait_for_element("input#clockin");
    let tag = match tag {
        Ok(_) => "input#clockin",
        Err(_) => "input#clockout",
    };

    tab.wait_for_element(tag)?.click()?;

    return if tag == "input#clockin" {
        Ok(true)
    } else {
        Ok(false)
    };
}
