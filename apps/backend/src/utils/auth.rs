use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::{ExposeSecret, SecretString};
use tower_cookies::cookie::time::{Duration, OffsetDateTime};
use tower_cookies::Cookie;

use crate::constants::auth::AUTH_COOKIE_NAME;
use crate::extractors::{AuthUser, Claims};

pub fn create_auth_cookie(user: AuthUser, secret: &[u8]) -> Cookie<'static> {
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
        &EncodingKey::from_secret(secret),
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

pub fn hash_password(password: &SecretString) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(password.expose_secret().as_ref(), &salt)
        .unwrap();

    hash.to_string()
}

pub fn verify_password(password_attempt: &SecretString, phc: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(phc).unwrap();

    let verification =
        argon2.verify_password(password_attempt.expose_secret().as_ref(), &parsed_hash);

    verification.is_ok()
}
