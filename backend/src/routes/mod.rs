pub mod syscalls;
pub mod iagon;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/system", syscalls::router())
        .nest("/iagon" ,iagon::router())
}