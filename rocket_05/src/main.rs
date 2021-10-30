use rocket::{build, get, launch, routes};

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    let _ = dotenv::dotenv().ok();
    build().mount("/", routes![ping])
}
