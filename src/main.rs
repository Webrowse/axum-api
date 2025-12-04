mod config;
mod routes;
mod state;
mod tasks; // must be at top if using main.rs as crate root

use sqlx::PgPool;


use axum::{Router};
use axum::routing::{post, get, put, delete};
use crate::tasks::routes as task_routes;


#[tokio::main]
async fn main() {
    let config = config::Config::from_env();

    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Error connecting DB");

    let state = state::AppState{db};

    let app = routes::routes().with_state(state);

    let listener = tokio::net::TcpListener::bind(config.addr()).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

