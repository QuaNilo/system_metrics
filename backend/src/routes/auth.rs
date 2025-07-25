use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{post};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::config::get_settings;
use crate::db::SQL;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
}

#[derive(Deserialize)]
struct LoginPayload{
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse{
    token: String,
}

#[derive(Serialize)]
struct JwtPayload{
    sub: i32,
    exp: usize,
}



#[derive(Clone, PartialEq, Debug, FromRow)]
struct User{
    user_id: i32,
    username: String,
    email: String,
    password_hash: String,
    role_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    two_factor_enabled: Option<bool>,
    two_factor_secret: Option<String>,
}

pub async fn login(
    Json(data): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, (StatusCode, String)>{
    let sql = SQL::new().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
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
    Ok(Json({
        LoginResponse{
            token
        }

    }))
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