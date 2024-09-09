//! Crate containing tons of common backend functionality for [rocket].
//!
//! This crate reexports [lettre].

#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[macro_use] extern crate rocket;
#[cfg(feature = "sentry")]
pub extern crate ferrox_sentry as sentry;

pub mod prelude {
    //! Contains reexports of all modules for easy importing.
    //!
    //! Example:
    //! ```rust
    //!  use ferrox_core::prelude::*;
    //! ```

    pub extern crate lettre;

    pub use crate::auth::*;
    #[cfg(debug_assertions)]
    pub use crate::cors::*;
    pub use crate::db::*;
    pub use crate::db_types::*;
    pub use crate::env_loader::*;
    pub use crate::mailer::*;
    #[cfg(feature = "sentry")]
    pub use crate::sentry::*;
    pub use crate::std_response::*;
    pub use crate::url_generator::*;
}

pub mod env_loader;
pub mod db_types;
pub mod std_response;
pub mod url_generator;
#[cfg(debug_assertions)]
pub mod cors;
pub mod mailer;
pub mod db;
pub mod auth;
