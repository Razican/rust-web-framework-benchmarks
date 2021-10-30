use warp::Filter;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv().ok();

    // GET /ping => 200 OK with body "pong"
    let hello = warp::path!("ping").map(|| "pong");

    warp::serve(hello)
        .run(([127, 0, 0, 1], 8000))
        .await;
}