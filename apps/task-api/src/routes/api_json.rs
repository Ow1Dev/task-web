use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};

use crate::routes::ApiError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::Json(rejection.body_text())
    }
}

impl<T> AppJson<T> {
    pub fn ok(body: T) -> (StatusCode, AppJson<T>) {
        (StatusCode::OK, AppJson(body))
    }

    pub fn created(body: T) -> (StatusCode, AppJson<T>) {
        (StatusCode::CREATED, AppJson(body))
    }
}
