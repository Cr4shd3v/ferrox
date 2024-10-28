use crate::prelude::*;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::{Cookie, SameSite};
use rocket::serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::error::Error;
use std::fs;
use std::sync::OnceLock;
use rocket::async_trait;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

static HMAC_SECRET: OnceLock<String> = OnceLock::new();

fn init_secret() -> String {
    let path = format!("{}/secret.local", std::env::var("PWD").unwrap());
    if fs::metadata(&*path).is_err() {
        let secret = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(64)
            .map(char::from)
            .collect::<String>();
        fs::write(&*path, &secret).unwrap();
        secret
    } else {
        fs::read_to_string(&*path).unwrap()
    }
}

/// Trait defining a way of logging in.
///
/// This is required for the [crate::Authenticated] guard to work.
#[async_trait]
pub trait Login: Send + Sync {
    /// Name of this type of login. (e.g. user)
    const LOGIN_NAME: &'static str;

    /// Returns the [Uuid] of this login.
    ///
    /// Usually refers to the [Uuid] of the corresponding entity.
    fn get_id(&self) -> Uuid;

    /// Creates the JWT token for this login.
    async fn create_token(&self, conn: &mut PooledConnection) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(HMAC_SECRET.get_or_init(init_secret).as_bytes()).unwrap();

        let claim = LoginClaim {
            id: self.get_id(),
            login_name: Self::LOGIN_NAME.to_string(),
            valid_to: OffsetDateTime::now_utc().checked_add(Duration::days(30)).unwrap(),
            roles: self.get_roles(conn).await.0.clone(),
        };

        claim.sign_with_key(&key).unwrap()
    }

    /// Creates a cookie based on the JWT provided by [Self::create_token].
    async fn create_cookie(&self, conn: &mut PooledConnection) -> Cookie<'static> {
        let mut cookie = Cookie::new(AUTH_COOKIE_NAME, self.create_token(conn).await);
        #[cfg(debug_assertions)]
        cookie.set_same_site(SameSite::None);
        cookie
    }

    /// Constructs the logout cookie.
    fn logout_cookie() -> Cookie<'static> {
        Cookie::build(AUTH_COOKIE_NAME).same_site(SameSite::None).build()
    }

    /// Retrieves a [Roles] struct from self for checking permissions.
    async fn get_roles(&self, conn: &mut PooledConnection) -> Roles;

    /// Retrieves a [RolesMut] struct from self for editing roles.
    async fn get_roles_mut(&mut self, conn: &mut PooledConnection) -> RolesMut;

    /// Provide a way to retrieve the login by an [Uuid].
    ///
    /// Usually happens through a query from the database.
    async fn get_by_id(id: Uuid, conn: &mut PooledConnection) -> Result<Option<Self>, Box<dyn Error>> where Self: Sized;

    /// Hashes a new password using this login.
    fn hash_pw(raw_pw: &[u8]) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        Ok(argon.hash_password(raw_pw, &salt)?.to_string())
    }

    /// Verifies the password for this login.
    fn verify_password(raw_pw: &[u8], pw_hash: &str) -> Result<(), argon2::password_hash::Error> {
        let argon = Argon2::default();
        let hash = PasswordHash::new(pw_hash)?;
        argon.verify_password(raw_pw, &hash)
    }
}

/// Claim of the JWT token of a login.
#[derive(Serialize, Deserialize)]
pub struct LoginClaim {
    /// [Uuid] for the login.
    pub id: Uuid,
    /// Name of the login
    pub login_name: String,
    /// Until when this JWT is valid
    pub valid_to: OffsetDateTime,
    /// Roles of the login.
    ///
    /// This causes a logout when roles are changing.
    pub roles: Vec<String>,
}

impl LoginClaim {
    /// Reads the [LoginClaim] from the JWT string.
    pub(crate) fn read_token(token: &str) -> Result<LoginClaim, String> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(HMAC_SECRET.get_or_init(init_secret).as_bytes()).unwrap();

        let claims: LoginClaim = token.verify_with_key(&key).unwrap();

        Ok(claims)
    }
}