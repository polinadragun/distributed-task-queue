use tokio::net::TcpListener;
use dotenv::dotenv;
use crate::api::routes::create_routes;
use crate::config::Config;
use crate::worker::worker::start_workers;
mod db;
mod api;
mod config;
mod worker;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let config = Config::from_env();
    let app = create_routes(config.clone());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");

    start_workers(config.clone()).await;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
