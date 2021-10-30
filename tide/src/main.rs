
#[async_std::main]
async fn main() -> tide::Result<()> {
    let _ = dotenv::dotenv().ok();

    let mut app = tide::new();
    app.at("/ping").get(|_| async { Ok("pong") });
    app.listen("0.0.0.0:8000").await?;
    Ok(())
}