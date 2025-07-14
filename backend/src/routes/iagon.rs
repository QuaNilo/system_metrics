use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/status", get(iagon_node_status))
}

async fn iagon_node_status() {
    // TODO GET IAGON NODE STATUS
}