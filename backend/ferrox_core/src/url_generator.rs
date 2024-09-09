//! Contains the implementation for generating absolute urls and api paths.
//!
//! Use [UrlGeneratorFairing] as fairing for rocket.
//!
//! See [UrlGenerator].

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::Origin;
use rocket::{Build, Rocket};
use std::sync::OnceLock;

/// Fairing to provide [UrlGenerator].
pub struct UrlGeneratorFairing;

static URL_GENERATOR: OnceLock<UrlGenerator> = OnceLock::new();

fn init_url_generator() -> UrlGenerator {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL is not set");

    UrlGenerator {
        base: base_url,
    }
}

/// Struct to generate absolute urls and api paths.
///
/// Retrieve through [UrlGenerator::get] or [UrlGenerator::get_or_init].
pub struct UrlGenerator {
    base: String,
}

impl UrlGenerator {
    /// Api path prefix used by [Self::api_path].
    pub const API_PATH: &'static str = "/api";

    /// Generates an absolute URL from an [Origin].
    ///
    /// This is usually used with the `uri!` macro.
    pub fn absolute_url(&self, origin: Origin) -> String {
        format!("{}{}", self.base, origin.path())
    }

    /// Prepends the api prefix "/api".
    pub fn api_path(origin: Origin) -> String {
        format!("{}{}", Self::API_PATH, origin.path())
    }

    /// Retrieves the [UrlGenerator].
    ///
    /// # Safety
    /// This requires [Self::get_or_init] to be called first.
    ///
    /// This usually happens through the [UrlGeneratorFairing].
    pub fn get() -> &'static Self {
        URL_GENERATOR.get().unwrap()
    }

    /// Retrieves or initializing the [UrlGenerator].
    pub fn get_or_init() -> &'static Self {
        URL_GENERATOR.get_or_init(init_url_generator)
    }
}

#[async_trait]
impl Fairing for UrlGeneratorFairing {
    fn info(&self) -> Info {
        Info {
            name: "url-generator",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        UrlGenerator::get_or_init();

        Ok(rocket)
    }
}