//! Development-only module to allow the frontend in development mode.

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{async_trait, Request, Response};

/// Development-only fairing to allow starting the frontend in development mode.
pub struct CORS;

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        if cfg!(debug_assertions) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, DELETE, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
        
        response.set_header(Header::new("Cross-Origin-Opener-Policy", "same-origin"));
        response.set_header(Header::new("Cross-Origin-Embedder-Policy", "require-corp"));
    }
}