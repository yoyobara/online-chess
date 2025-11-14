use jsonwebtoken::{encode, EncodingKey, Header};
use tower_cookies::cookie::time::{Duration, OffsetDateTime};
use tower_cookies::Cookie;

use crate::configs::ENV_CONFIG;
use crate::constants::auth::AUTH_COOKIE_NAME;
use crate::extractors::{AuthUser, Claims};

pub fn create_auth_cookie(user: AuthUser) -> Cookie<'static> {
    let now = OffsetDateTime::now_utc();
    let expiry = now.saturating_add(Duration::WEEK);

    let claims = Claims {
        iat: now.unix_timestamp(),
        exp: expiry.unix_timestamp(),
        user,
    };

    let encoded = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ENV_CONFIG.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build((AUTH_COOKIE_NAME, encoded))
        .expires(expiry)
        .max_age(Duration::WEEK)
        .http_only(true)
        .path("/")
        .build();

    cookie
}
