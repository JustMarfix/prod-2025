use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::info;

#[derive(thiserror::Error, Debug)]
pub enum ProdError {
    #[error("JWT error")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    #[error("hashing error")]
    HashingError(#[from] argon2::Error),

    /// If the request was invalid or malformed.
    #[error("the request was invalid {0}")]
    InvalidRequest(#[from] validator::ValidationErrors),

    #[error("some shit happened {0}")]
    ShitHappened(String),

    #[error("{0}")]
    AlreadyExists(String),

    /// An error occured when connection to or using the database.
    #[error("{0}")]
    DatabaseError(#[from] sqlx::Error),

    /// Not found error
    #[error("{0}")]
    NotFound(String),

    /// Conflict Error
    #[error("{0}")]
    Conflict(String),

    /// Forbidden Error
    #[error("{0}")]
    Forbidden(String),

    /// Any other, unknown error sources.
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for ProdError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            Self::AlreadyExists(_) | Self::InvalidRequest(_) | Self::ShitHappened(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            Self::DatabaseError(_) | Self::Unknown(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            Self::Forbidden(_) | Self::InvalidToken(_) => (StatusCode::FORBIDDEN, self.to_string()),
            Self::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
        };

        info!("returning error: {}", message);

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
