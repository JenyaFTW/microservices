use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use validator::Validate;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::models::user::User;

#[derive(Serialize, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(email)]
    email: String,
    #[validate(length(max = 64))]
    name: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
    #[serde(rename = "confirmPassword")]
    #[validate(must_match = "password")]
    confirm_password: String
}

pub async fn signup_handler(Json(payload): Json<SignupRequest>) -> impl IntoResponse {
    let user = User::new(payload.name, payload.email, payload.password).await.unwrap();
    let token = user.to_jwt().await;

    (StatusCode::OK, Json(json!({
        "message": "Successfully registered new user!",
        "token": token
    }))).into_response()
}
