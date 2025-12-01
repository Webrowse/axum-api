use axum::{Router, routing::{ get, post }};

mod health;
mod auth;

pub use health::health;
pub use auth::register;

use crate::state::AppState;

pub fn routes() -> Router<AppState>{
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/auth/register", post(register))
}

async fn root() -> &'static str {
    "Welcome to the API written in Rust"
}