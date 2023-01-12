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
pub struct LoginRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
    #[serde(rename = "confirmPassword")]
    #[validate(must_match = "password")]
    confirm_password: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user_result = User::from_email(&state.db, payload.email).await;

    let user = match user_result {
        Ok(u) => u,
        Err(_) => {
            return RestResponse::<Value>::with_message(
                StatusCode::FORBIDDEN,
                "Invalid email/password.".to_string(),
            )
        }
    };

    let password_result = bcrypt::verify(payload.password, &user.password).unwrap();
    if !password_result {
        return RestResponse::<Value>::with_message(
            StatusCode::FORBIDDEN,
            "Invalid email/password.".to_string(),
        );
    }

    if !user.verified {
        return RestResponse::<Value>::with_message(
            StatusCode::FORBIDDEN,
            "Your account isn't verified".to_string()
        )
    }

    let token_str = user.to_jwt().await;

    RestResponse::<Value>::new(
        StatusCode::OK,
        "Success!".to_string(),
        json!({ "token": token_str }),
    )
}
