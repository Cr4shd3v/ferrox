//! Contains the [CORS] fairing which sets the correct headers for flutter_rust_bridge.
//! 
//! In Development mode it also allows to use this backend for debug mode flutter.

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{async_trait, Request, Response};

/// sets the correct headers for flutter_rust_bridge.
///
/// In Development mode it also allows to use this backend for debug mode flutter.
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