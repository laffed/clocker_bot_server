#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
// #[macro_use]
// extern crate serde_derive;
// #[macro_use]
// extern crate serde_json;

// use rocket_contrib::json::Json;
// use serde_json::Value;

mod clocker;

use clocker::{do_clock_event, get_clock_status};

#[get("/status")]
fn status() -> &'static str {
    let call_status = get_clock_status();
    let call_status = match call_status {
        Ok(is) => return if is { "in" } else { "out" },
        Err(_) => "err",
    };
    call_status
}

#[get("/clock")]
fn clocker() -> &'static str {
    let call_status = do_clock_event();
    let call_status = match call_status {
        Ok(is) => return if is { "in" } else { "out" },
        Err(_) => "err",
    };
    call_status
}

fn main() {
    rocket::ignite()
        .mount("/", routes![status, clocker])
        .launch();
}
