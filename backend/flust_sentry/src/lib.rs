//! Contains the implementation of sentry for flust.
//!
//! Use [RocketSentry::fairing] to create a fairing for rocket.

use std::sync::Mutex;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{async_trait, info, Build, Rocket};
use sentry::{release_name, ClientInitGuard, ClientOptions};

/// Fairing implementing sentry for this instance.
pub struct RocketSentry {
    guard: Mutex<Option<ClientInitGuard>>,
}

impl RocketSentry {
    /// Initializes this [Fairing].
    #[must_use]
    pub fn fairing() -> impl Fairing {
        RocketSentry {
            guard: Mutex::new(None),
        }
    }

    fn init(&self, dsn: &str) {
        let guard = sentry::init((dsn, ClientOptions {
            release: release_name!(),
            ..Default::default()
        }));

        if guard.is_enabled() {
            let mut self_guard = self.guard.lock().unwrap();
            *self_guard = Some(guard);
        } else {
            info!("Sentry not enabled");
        }
    }
}

#[async_trait]
impl Fairing for RocketSentry {
    fn info(&self) -> Info {
        Info {
            name: "rocket-sentry",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        match std::env::var("SENTRY_DSN") {
            Ok(dsn) => {
                self.init(&*dsn);
            }
            Err(_) => {
                info!("No sentry dsn provided, disabled sentry");
            }
        };

        Ok(rocket)
    }
}