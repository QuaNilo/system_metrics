use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{post};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use chrono::{Utc, Duration};
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



#[derive(Debug, FromRow, Serialize)]
struct User{
    user_id: i32,
    username: String,
    email: String,
    password_hash: String,
    role_id: i32,
    created_at: String,
    updated_at: String,
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

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    let is_valid: bool = Argon2::default()
        .verify_password(data.password.as_bytes(), &parsed_hash)
        .is_ok();
    if(!is_valid){
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