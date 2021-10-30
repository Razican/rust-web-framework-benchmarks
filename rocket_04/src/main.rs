#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, ignite, routes};

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

fn main() {
    let _ = dotenv::dotenv().ok();
    ignite().mount("/", routes![ping]).launch();
}
