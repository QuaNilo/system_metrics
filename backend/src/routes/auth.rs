use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{middleware, Json, Router};
use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::http::{header, StatusCode};
use axum::routing::{post};
use chrono::{Duration, Utc};
use cookie::{Cookie, CookieBuilder, SameSite};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::config::{get_settings, Settings};
use crate::data_classes::auth::{JwtPayload, LoginPayload, User};
use crate::db::SQL;
use crate::middlewares::auth_validation::AuthUser;

pub fn router() -> Router {
    let auth_validation = middleware::from_extractor::<AuthUser>();
    let protected_routes = Router::new()
        .route("/session", post(validate_session))
        .route("/logout", post(logout))
        .route_layer(auth_validation);

    Router::new()
        .route("/login", post(login))
        .merge(protected_routes)
}

pub async fn login(
    Json(data): Json<LoginPayload>,
) -> Result<Response, (StatusCode, String)>{
    let sql = SQL::new().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let config = get_settings();
    let database_user: Option<User> = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
        .bind(data.username)
        .fetch_optional(&sql.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = database_user.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, "Password or username is incorrect".to_string())
    })?;

    let is_valid = verify_password_blocking(data.password.clone(), user.password_hash.clone()).await?;

    if !is_valid {
        return Err((StatusCode::BAD_REQUEST, "Password or username is incorrect".to_string()))
    };

    let token: String = create_jwt(&user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let cookie: CookieBuilder = Cookie::build(("auth_token", token))
        .http_only(true)
        .secure(config.app.http_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::days(7));

    let mut response: Response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(axum::body::Body::empty())
        .unwrap();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie.to_string().parse().unwrap()
    );
    Ok(response)

}

async fn verify_password_blocking(
    password: String,
    user_hash: String,
) -> Result<bool, (StatusCode, String)> {
    let result = tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&user_hash).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?;

    Ok(result.is_ok())
}

async fn create_jwt(database_user: &User) -> Result<String, jsonwebtoken::errors::Error>{
    let settings = get_settings();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let payload: JwtPayload = JwtPayload {
        sub: database_user.user_id,
        exp: expiration as usize,
    };
    encode(&Header::default(), &payload, &EncodingKey::from_secret(settings.app.secret.as_bytes()))
}

async fn validate_session(jar: CookieJar) -> Result<StatusCode, (StatusCode, String)> {
    let config: &Settings = get_settings();
    let token = match jar.get("auth_token").map(|cookie| cookie.value().to_string()) {
        Some(token) => token,
        None => return Err((StatusCode::UNAUTHORIZED, "Token not present!".to_string())),
    };

    // VALIDATE JWT
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    match decode::<JwtPayload>(
        &token,
        &DecodingKey::from_secret(config.app.secret.as_bytes()),
        &validation
    ) {
        Ok(token_data) => Ok(StatusCode::NO_CONTENT),
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Token is invalid!".to_string())),
    }
}

async fn logout(token: String) -> Result<impl IntoResponse, (StatusCode, String)> {
    let config = get_settings();
    let cookie = Cookie::build(("auth_token", ""))
        .http_only(true)
        .secure(config.app.http_secure)
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(0));
    let mut response = Response::new(axum::body::Body::empty());
    *response.status_mut() = StatusCode::NO_CONTENT;
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie.to_string().parse().unwrap()
    );
    Ok(response.into_response())

}