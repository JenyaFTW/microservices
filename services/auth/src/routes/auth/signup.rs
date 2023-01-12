use crate::lib::json_extractor::Json;
use crate::lib::rest_response::RestResponse;
use crate::models::user::{User, UserError};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use lapin::BasicProperties;
use lapin::options::BasicPublishOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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
    let rand_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let user_result = User::new(&state.db, payload.email, payload.password, rand_code).await;


    let user = match user_result {
        Ok(u) => u,
        Err(e) => {
            return match e {
                UserError::AlreadyExists => RestResponse::with_message(
                    StatusCode::FORBIDDEN,
                    "A user with such email address is already registered.".to_string(),
                ),
                e => RestResponse::with_message(
                    StatusCode::FORBIDDEN,
                    "Encountered an error while trying to sign up.".to_string(),
                ),
            }
        }
    };

    let payload = json!({
        "code": user.code,
        "email": user.email
    });

    state.channel.basic_publish(
        "emails",
        "verify_email",
        BasicPublishOptions::default(),
        payload.to_string().as_str().as_bytes(),
        BasicProperties::default(),
    ).await.expect("basic_publish");

    RestResponse::with_data(StatusCode::OK, "Success!".to_string())
}
