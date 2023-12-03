use rocket::serde::json::Json;
use serde::Serialize;
use serde_json::json;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

#[derive(Debug, Serialize)]
pub enum ApiError {
    Server(String),
    NotFound(String),
    Uuid(String),
}

impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        ApiError::Uuid(format!("UUID error: {err}"))
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: Status,
    error: String,
    description: String,
}

/// Default error handler, called by Rocket when an error occurs.
/// This replaces the default handler that returns HTML.
#[catch(default)]
pub fn handle_error(status: Status, _req: &Request) -> Json<ErrorResponse> {
    let error = ErrorResponse {
        status,
        error: format!("{:?}", status.class()),
        description: status.reason().unwrap_or("Unknown").to_string(),
    };
    Json(error)
}

/// Custom Responder implementation, used to return a JSON response
/// when encountering an ApiError.
impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let response = match self {
            ApiError::Server(message) => ErrorResponse {
                status: Status::InternalServerError,
                error: "Generic".into(),
                description: message,
            },
            ApiError::Uuid(message) => ErrorResponse {
                status: Status::BadRequest,
                error: "UUID".into(),
                description: message,
            },
            ApiError::NotFound(message) => ErrorResponse {
                status: Status::NotFound,
                error: "NotFound".into(),
                description: message,
            },
        };
        let json = json!(response);

        Response::build_from(json.respond_to(req).unwrap())
            .status(response.status)
            .header(ContentType::JSON)
            .ok()
    }
}
