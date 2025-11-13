use axum::{extract::FromRequestParts, http::StatusCode};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::constants::auth::AUTH_COOKIE_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub player_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub user: AuthUser,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "cookies extractors failed"))?;

        let jwt_token = cookies
            .get(AUTH_COOKIE_NAME)
            .map(|c| c.value().to_string())
            .ok_or((StatusCode::UNAUTHORIZED, "missing jwt cookie"))?;

        let secret = "mysecret";
        let decoded = decode::<Claims>(
            &jwt_token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid jwt"))?;

        Ok(decoded.claims.user)
    }
}
