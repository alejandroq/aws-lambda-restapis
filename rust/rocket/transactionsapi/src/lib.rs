#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Rocket;
use log::{info, warn};

#[get("/")]
fn hello() -> &'static str {
    warn!("Called the default");
    "Hello, World!"
}

#[get("/world")]
fn world() -> &'static str {
    info!("Something {}", 1);
    "Goodbye, World!"
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello, world])
}
