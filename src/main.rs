mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let app = routes::router();
    let address : &str = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);

    axum::serve(listener, app).await.unwrap();
}
