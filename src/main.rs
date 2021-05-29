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

use clocker::test;

#[get("/status")]
fn status() -> String {
    let first_test = test();
    let first_test = match first_test {
        Ok(_) => String::from("Everything went swimmingly!"),
        Err(error) => format!("Oh boy: {}", error),
    };
    print!("{}", first_test);
    first_test
}

#[get("/clock")]
fn clocker() -> &'static str {
    "clocking"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![status, clocker])
        .launch();
}
