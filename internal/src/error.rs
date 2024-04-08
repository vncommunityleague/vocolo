#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    AlreadyExists(String),

    #[error("Unknown Tournament")]
    UnknownTournament,

    #[error("Unknown Mappool")]
    UnknownMappool,

    #[error("Unknown Mappool Map")]
    UnknownMappoolMap,

    #[error("Unknown Match")]
    UnknownMatch,

    #[error("Tournament is not yet open for registration")]
    RegistrationNotOpen,

    #[error("Tournament can no longer be registered")]
    RegistrationClosed,

    #[error("Player(s) already registered")]
    AlreadyRegistered,

    #[error("You are not permitted")]
    Unauthorized,

    // Internal
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    Database(String),

    #[error(transparent)]
    BsonSer(#[from] bson::ser::Error),

    #[error(transparent)]
    BsonDe(#[from] bson::de::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use http::StatusCode;

        let description = self.to_string();
        let (status_code, error_message) = match self {
            Error::AlreadyExists(_) => (StatusCode::BAD_REQUEST, "already_exists"),
            Error::UnknownTournament => (StatusCode::NOT_FOUND, "unknown_tournament"),
            Error::UnknownMappool => (StatusCode::NOT_FOUND, "unknown_mappool"),
            Error::UnknownMappoolMap => (StatusCode::NOT_FOUND, "unknown_mappool_map"),
            Error::UnknownMatch => (StatusCode::NOT_FOUND, "unknown_match"),

            Error::RegistrationNotOpen => (StatusCode::BAD_REQUEST, "registration_not_open"),
            Error::RegistrationClosed => (StatusCode::BAD_REQUEST, "registration_closed"),
            Error::AlreadyRegistered => (StatusCode::BAD_REQUEST, "already_registered"),

            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),

            // Internal
            Error::Internal(_)
            | Error::Database(_)
            | Error::BsonDe(_)
            | Error::BsonSer(_)
            | Error::Reqwest(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        };

        (
            status_code,
            axum::Json(serde_json::json!({
                "error": &error_message,
                "description": &description,
            })),
        )
            .into_response()
    }
}
