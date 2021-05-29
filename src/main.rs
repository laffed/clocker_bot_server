#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate rocket_contrib;

mod clocker;
use rocket_contrib::json::{Json, JsonValue};

use clocker::{do_clock_event, get_clock_status};
use dotenv::dotenv;

#[get("/status")]
fn status() -> Json<JsonValue> {
    let call_status = get_clock_status();
    let call_status = match call_status {
        Ok(is) => {
            if is {
                "in"
            } else {
                "out"
            }
        }
        Err(_) => "err",
    };

    Json(json!({
        "status": call_status,
    }))
}

#[get("/clock")]
fn clocker() -> Json<JsonValue> {
    let call_status = do_clock_event();
    let call_status = match call_status {
        Ok(is) => {
            if is {
                "in"
            } else {
                "out"
            }
        }
        Err(_) => "err",
    };

    Json(json!({
        "status": call_status,
    }))
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![status, clocker])
        .launch();
}
