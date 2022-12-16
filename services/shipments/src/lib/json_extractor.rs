use crate::lib::rest_response::RestResponse;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::Value;
use validator::Validate;

pub async fn handler(Json(value): Json<Value>) -> impl IntoResponse {
    Json(dbg!(value));
}

pub struct Json<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
    T: Validate,
{
    type Rejection = RestResponse<Option<Vec<String>>>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => {
                match value.validate() {
                    Ok(_) => (),
                    Err(e) => {
                        let mut fields: Vec<String> = Vec::new();
                        for (key, _) in &e.field_errors() {
                            fields.push(key.to_string())
                        }

                        return Err(RestResponse::new(
                            StatusCode::UNPROCESSABLE_ENTITY,
                            "Invalid body",
                            Option::from(fields),
                        ));
                    }
                };

                Ok(Self(value.0))
            }
            Err(rejection) => {
                let code = match rejection {
                    JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
                    JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                Err(RestResponse::with_message(code, "Internal server error"))
            }
        }
    }
}
