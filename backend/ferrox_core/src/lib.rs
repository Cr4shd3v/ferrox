//! Crate containing tons of common backend functionality for [rocket].
//!
//! This crate reexports [lettre].

#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(feature = "sentry")]
pub extern crate ferrox_sentry as sentry;
#[cfg(feature = "env")]
pub extern crate ferrox_env as env;
#[cfg(feature = "mailer")]
pub extern crate ferrox_mailer as mailer;
#[cfg(feature = "auth")]
pub extern crate ferrox_auth as auth;
#[cfg(feature = "db")]
pub extern crate ferrox_auth as db;

pub mod prelude {
    //! Contains reexports of all modules for easy importing.
    //!
    //! Example:
    //! ```rust
    //!  use ferrox_core::prelude::*;
    //! ```

    #[cfg(feature = "auth")]
    pub use crate::auth::*;
    #[cfg(debug_assertions)]
    pub use crate::cors::*;
    #[cfg(feature = "db")]
    pub use crate::db::*;
    pub use crate::db_types::*;
    #[cfg(feature = "env")]
    pub use crate::env::*;
    #[cfg(feature = "mailer")]
    pub use crate::mailer::*;
    #[cfg(feature = "sentry")]
    pub use crate::sentry::*;
    pub use crate::std_response::*;
    pub use crate::url_generator::*;
}

pub mod db_types;
pub mod std_response;
pub mod url_generator;
#[cfg(debug_assertions)]
pub mod cors;
