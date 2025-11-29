
mod routes;
mod config;

#[tokio::main]
async fn main(){

    let config = config::Config::from_env();

    let app = routes::routes();

    let listener = tokio::net::TcpListener::bind(config.addr()).await.unwrap();

    println!("Server is running on {}", config.addr());
    
    axum::serve(listener, app).await.unwrap();
}

