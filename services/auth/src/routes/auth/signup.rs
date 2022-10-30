use crate::lib::json_extractor::Json;
use crate::lib::rest_response::RestResponse;
use crate::models::user::{User, UserError};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(email)]
    email: String,
    password: String,
    #[serde(rename = "confirmPassword")]
    #[validate(must_match = "password")]
    confirm_password: String,
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> impl IntoResponse {
    let user_result = User::new(&state.db, payload.email, payload.password).await;

    match user_result {
        Ok(u) => u,
        Err(e) => {
            return match e {
                UserError::AlreadyExists => RestResponse::with_message(
                    StatusCode::FORBIDDEN,
                    "A user with such email address is already registered.".to_string(),
                ),
                _ => RestResponse::with_message(
                    StatusCode::FORBIDDEN,
                    "Encountered an error while trying to sign up.".to_string(),
                ),
            }
        }
    };

    RestResponse::with_data(StatusCode::OK, "Success!".to_string())
}
