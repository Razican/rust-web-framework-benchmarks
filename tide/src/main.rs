
#[async_std::main]
async fn main() -> tide::Result<()> {
    let _ = dotenv::dotenv().ok();

    let mut app = tide::new();
    app.at("/ping").get(|_| async { Ok("pong") });
    app.listen("127.0.0.1:8000").await?;
    Ok(())
}