//! Contains the implementation of [Mailer] to send emails through smtp.
//!
//! Use [FerroxMailerFairing] as fairing for rocket.

pub extern crate lettre;

use std::ops::Deref;
use std::sync::OnceLock;

use lettre::SmtpTransport;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{async_trait, error, Build, Rocket};

/// Fairing initializing [Mailer].
pub struct FerroxMailerFairing;

#[async_trait]
impl Fairing for FerroxMailerFairing {
    fn info(&self) -> Info {
        Info {
            name: "ferrox-mailer",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        MAILER.get_or_init(init_mailer);

        Ok(rocket)
    }
}

static MAILER: OnceLock<Mailer> = OnceLock::new();

fn init_mailer() -> Mailer {
    let transport = SmtpTransport::from_url(
        &*std::env::var("MAILER_DSN").expect("No MAILER_DSN provided"),
    ).unwrap().build();

    match transport.test_connection() {
        Ok(success) => {
            if !success {
                error!("Mailer noop failed");
            }
        }
        Err(e) => {
            error!("Failed to test mailer: {}", e);
        }
    }

    Mailer {
        transport,
    }
}

/// Contains [SmtpTransport] to send mails.
///
/// Can be retrieved by [Mailer::get] or [Mailer::get_or_init].
pub struct Mailer {
    transport: SmtpTransport,
}

impl Mailer {
    /// Retrieves the current [Mailer].
    ///
    /// # Safety
    /// This requires [Self::get_or_init] to be called first.
    ///
    /// This usually happens through the [FerroxMailerFairing].
    pub fn get() -> &'static Self {
        MAILER.get().unwrap()
    }

    /// Retrieves or initializing the [Mailer].
    pub fn get_or_init() -> &'static Self {
        MAILER.get_or_init(init_mailer)
    }
}

impl Deref for Mailer {
    type Target = SmtpTransport;

    fn deref(&self) -> &Self::Target {
        &self.transport
    }
}

#[cfg(test)]
mod tests {
    use ferrox_env::EnvLoader;
    use crate::Mailer;

    #[test]
    fn test_mailer() {
        EnvLoader::load_test();
        let mailer = Mailer::get_or_init();
        let test = mailer.test_connection();
        assert!(test.is_ok());
        assert!(test.unwrap());
    }
}