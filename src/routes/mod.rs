pub mod syscalls;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(syscalls::router())
}