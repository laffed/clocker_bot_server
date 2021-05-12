#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/status")]
fn status() -> &'static str {
    "clock status"
}

#[get("/status")]
fn clocker() -> &'static str {
    "clocking"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![status, clocker])
        .launch();
}
