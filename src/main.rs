mod handler;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = repository::Repository::connect().await?;
    let app = handler::make_router(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
