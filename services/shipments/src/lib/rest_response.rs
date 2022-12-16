use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct RestResponse<T>
where
    T: Serialize,
{
    #[serde(skip)]
    code: Option<u16>,
    status: Option<String>,
    message: Option<String>,
    data: Option<T>,
}

impl<T> RestResponse<T>
where
    T: Serialize,
{
    pub fn new(code: StatusCode, message: &str, data: T) -> Self {
        let mut status = "error";
        if code.as_u16() < 400 {
            status = "success";
        }

        RestResponse {
            status: Some(status.to_string()),
            code: Some(code.as_u16()),
            message: Some(message.to_string()),
            data: Some(data),
        }
    }

    pub fn with_message(code: StatusCode, message: &str) -> Self {
        let mut status = "error";
        if code.as_u16() < 400 {
            status = "success";
        }

        RestResponse {
            status: Some(status.to_string()),
            code: Some(code.as_u16()),
            message: Some(message.to_string()),
            data: None,
        }
    }

    pub fn with_data(code: StatusCode, data: T) -> Self {
        let mut status = "error";
        if code.as_u16() < 400 {
            status = "success";
        }

        RestResponse {
            status: Some(status.to_string()),
            code: Some(code.as_u16()),
            message: None,
            data: Some(data),
        }
    }
}

impl<T> IntoResponse for RestResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.code.unwrap()).unwrap(),
            Json(self),
        )
            .into_response()
    }
}
