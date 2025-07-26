pub mod syscalls;
pub mod iagon;
mod auth;

use axum::{middleware, Router};
use crate::config::get_settings;
use crate::middlewares::auth_validation::AuthUser;

pub fn router() -> Router {
    let config = get_settings();
    let mut system_router = syscalls::router();
    let mut iagon_router = iagon::router();

    if config.app.auth_enabled {
        let auth_validation = middleware::from_extractor::<AuthUser>();
        system_router = system_router.route_layer(auth_validation.clone());
        iagon_router = iagon_router.route_layer(auth_validation);
    }

    Router::new()
    .nest("/system", system_router)
    .nest("/iagon", iagon_router)
    .nest("/auth", auth::router())
}