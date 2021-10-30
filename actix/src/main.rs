use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn ping(_req: HttpRequest) -> impl Responder {
    "pong"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}