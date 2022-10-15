use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use validator::Validate;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::models::user::User;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
    #[serde(rename = "confirmPassword")]
    #[validate(must_match = "password")]
    confirm_password: String
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let user = User::new("Test User".to_string(), payload.email, payload.password).await.unwrap();
    let token = user.to_jwt().await;

    (StatusCode::OK, Json(json!({
        "message": "Successfully logged in!",
        "token": token
    }))).into_response()
}
