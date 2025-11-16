use axum::{extract::FromRequestParts, http::StatusCode};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{constants::auth::AUTH_COOKIE_NAME, state::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub player_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub exp: i64,
    pub user: AuthUser,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "cookies extractors failed"))?;

        let jwt_token = cookies
            .get(AUTH_COOKIE_NAME)
            .map(|c| c.value().to_string())
            .ok_or((StatusCode::UNAUTHORIZED, "missing jwt cookie"))?;

        let decoded = decode::<Claims>(
            &jwt_token,
            &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid jwt"))?;

        Ok(decoded.claims.user)
    }
}
