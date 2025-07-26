use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct LoginPayload{
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct JwtPayload{
    pub sub: i32,
    pub exp: usize,
}



#[derive(Clone, PartialEq, Debug, FromRow)]
pub struct User{
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub two_factor_enabled: Option<bool>,
    pub two_factor_secret: Option<String>,
}