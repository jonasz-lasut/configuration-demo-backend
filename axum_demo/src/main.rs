use core::result::Result::Ok;
use tokio::net::TcpListener;
use tracing::{info, instrument};

mod db;
mod errors;
mod handlers;
mod helpers;
mod models;
mod router;
mod serialize;
mod templates;

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().json())
        .init();

    let name = helpers::get_env("APPLICATION_NAME");

    let pg_pool = db::initialize_pgpool().await;
    let app = router::init_router(pg_pool, name);

    let bind_addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    info!("🚀 Starting server at {}", bind_addr);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
