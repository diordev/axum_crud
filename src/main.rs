mod controller;
mod errors;
mod model;
mod routes;
mod state;

use anyhow::Result; // Qulay xatolar zanjiri va Result turi
use axum::Router; // Axum router uchun
use routes::user_routes; // Foydalanuvchilar uchun marshrutlar
use state::AppState; // AppState holati uchun
use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing() {
    // RUST_LOG=debug axum=info sqlx=warn kabi filtrlashni qo‘llab-quvvatlaydi
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,sqlx=warn,axum=info"));
    fmt().with_env_filter(filter).with_target(false).init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::from_env().await?;
    init_tracing(); // Tracingni ishga tushirish
    let app = Router::new()
        .nest("/api", user_routes())
        // kerak bo'lsa boshqa modullarni ham shu tarzda nest qiling
        // masalan, .nest("/api/posts", post_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
