use axum::{http::StatusCode, response::IntoResponse};

pub mod v1;

pub mod not_found;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound = 0,
}

impl ApiError {
    pub fn as_api_error<'a>(&self) -> crate::models::error::ApiError<'a> {
        crate::models::error::ApiError {
            error: match self {
                Self::NotFound => "not_found",
            },
            description: match self {
                _ => self.to_string(),
            },
            details: match self {
                _ => None,
            },
        }
    }

    fn status_code(&self) -> StatusCode {
        match &self {
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self.as_api_error()).unwrap();
        (self.status_code(), body).into_response()
    }
}
