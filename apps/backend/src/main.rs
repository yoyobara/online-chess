mod config;
mod constants;
mod extractors;
mod routes;
mod utils;

const ADDR: (&str, u16) = ("0.0.0.0", 3000);

#[tokio::main]
async fn main() {
    let app = routes::router();
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
