use axum::response::IntoResponse;

pub async fn echo() -> impl IntoResponse {
    "hello, world!"
}
