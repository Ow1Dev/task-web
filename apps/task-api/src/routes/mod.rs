use axum::{Json, http::StatusCode, response::IntoResponse};

pub mod v1;

mod api_json;
pub mod not_found;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound,
    #[error("Deserialization error: {0}")]
    Json(String),
    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
}

impl ApiError {
    pub fn as_api_error<'a>(&self) -> crate::models::error::ApiError<'a> {
        crate::models::error::ApiError {
            error: match self {
                Self::NotFound => "not_found",
                Self::Json(..) => "json_error",
                Self::Database(..) => "database_error",
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
            ApiError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Json(..) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(&self.as_api_error());
        (self.status_code(), body).into_response()
    }
}
