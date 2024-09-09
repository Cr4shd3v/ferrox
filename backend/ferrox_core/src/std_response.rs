//! Contains the standard response for the frontend. See [StdResponse].

use rocket::response::content::RawJson;
use rocket::response::Responder;
use rocket::Request;
use serde::{Deserialize, Serialize};

/// Default response struct. Should be used in all responses to the frontend.
#[derive(Serialize, Deserialize)]
pub struct StdResponse<T: Serialize> {
    /// Marks whether the request was successful or not.
    pub success: bool,
    /// If success is true, this will contain `T`.
    pub data: Option<T>,
    /// If success is false, this will contain the error message.
    pub msg: Option<String>,
}

impl<T: Serialize> StdResponse<T> {
    /// Shortcut function to create a successful response with `data`.
    pub fn success(data: T) -> StdResponse<T> {
        StdResponse {
            success: true,
            data: Some(data),
            msg: None,
        }
    }

    /// Creates a failure response with `msg`.
    pub fn failure(msg: &str) -> StdResponse<T> {
        StdResponse {
            success: false,
            data: None,
            msg: Some(msg.to_string()),
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'r> for StdResponse<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'r> {
        RawJson(serde_json::to_string(&self).unwrap()).respond_to(request)
    }
}