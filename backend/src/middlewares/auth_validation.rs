use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use cookie::Cookie;
use crate::config::get_settings;
use crate::data_classes::auth::JwtPayload;

pub struct AuthUser(JwtPayload);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) -> Result<Self, Self::Rejection> {
        let secret_key = &get_settings().app.secret;
        // Get raw cookie header
        let cookie_header = parts.headers.get("cookie")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing cookie header"))?;

        // Parse and find the auth_token
        let auth_token = cookie_header
            .split(';')
            .find_map(|cookie_str| {
                Cookie::parse_encoded(cookie_str.trim()).ok().filter(|c| c.name() == "auth_token")
            })
            .map(|c| c.value().to_string())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing auth_token"))?;

        // Verify JWT
        let token_data: TokenData<JwtPayload> = decode::<JwtPayload>(
            &auth_token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default(),
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        Ok(AuthUser(token_data.claims))
    }
}