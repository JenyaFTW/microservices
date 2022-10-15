use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use validator::Validate;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct MeResponse {
    id: Uuid,
    name: String,
    email: String
}

pub async fn me_handler() -> impl IntoResponse {
    let user = User::new("Test User".to_string(), "test@company.com".to_string(), "Test123".to_string()).await.unwrap();

    let res = MeResponse {
        id: user.id,
        name: user.name,
        email: user.email
    };
    (StatusCode::OK, Json(res)).into_response()
}
