//! Contains implementations for loading environment files through [dotenvy].
//!
//! See [EnvLoader].

/// Struct providing handy functions for loading environments.
pub struct EnvLoader;

impl EnvLoader {
    /// Loads all default environment files.
    ///
    /// Load order (overwriting previous values):
    /// - [std::env]
    /// - .env
    /// - .env.local
    pub fn load() {
        dotenvy::from_filename_override(format!("{}/.env", std::env::var("PWD").unwrap())).ok();
        dotenvy::from_filename_override(format!("{}/.env.local", std::env::var("PWD").unwrap())).ok();
    }

    /// Loads all environment files for testing.
    ///
    /// Load order (overwriting previous values):
    /// - [Self::load]
    /// - .env.test
    /// - .env.test.local
    pub fn load_test() {
        Self::load();

        println!("Loading test configurations");
        dotenvy::from_filename_override(format!("{}/.env.test", std::env::var("PWD").unwrap())).ok();
        dotenvy::from_filename_override(format!("{}/.env.test.local", std::env::var("PWD").unwrap())).ok();
    }
}