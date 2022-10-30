use crate::lib::rest_response::RestResponse;
use crate::models::user::{User, UserClaims};
use crate::routes::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn me_handler(claims: UserClaims, State(state): State<AppState>) -> impl IntoResponse {
    let user_result = User::from_id(&state.db, claims.sub).await;
    let user = match user_result {
        Ok(u) => u,
        Err(_) => {
            return RestResponse::with_message(
                StatusCode::FORBIDDEN,
                "Error while fetching user.".to_string(),
            )
        }
    };

    RestResponse::<User>::with_data(StatusCode::OK, user)
}
