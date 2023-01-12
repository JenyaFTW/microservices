use crate::lib::json_extractor::Json;
use crate::lib::rest_response::RestResponse;
use crate::models::user::User;
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6, max = 6))]
    code: String
}

pub async fn verify_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> impl IntoResponse {
    let user_result = User::from_email(&state.db, payload.email).await;

    let user = match user_result {
        Ok(u) => u,
        Err(_) => {
            return RestResponse::<Value>::with_message(
                StatusCode::FORBIDDEN,
                "User with such email not found.".to_string(),
            )
        }
    };

    if user.verified {
        return RestResponse::<Value>::with_message(
            StatusCode::FORBIDDEN,
            "User already verified.".to_string(),
        )
    }

    user.verify(&state.db).await.expect("verified");

    RestResponse::<Value>::with_message(
        StatusCode::OK,
        "Successfully verified account!".to_string()
    )
}
