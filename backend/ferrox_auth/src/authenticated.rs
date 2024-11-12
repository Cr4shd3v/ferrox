use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};
use time::OffsetDateTime;
use ferrox_db::DbPool;
use crate::{Login, LoginClaim, Permission};

/// Request guard for authenticated endpoints.
///
/// Provides the [Login] instance and checks provided [Permission].
///
/// This will automatically respond with [Status::Unauthorized] if conditions are not met.
pub struct Authenticated<T: Login, P: Permission = ()>(T, PhantomData<P>);

impl<T: Login> Deref for Authenticated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Login> DerefMut for Authenticated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Contains the name of the authentication cookie.
#[cfg(feature = "auth-from-cookie")]
pub const AUTH_COOKIE_NAME: &'static str = "Authentication";

/// Contains the name of the authentication header.
#[cfg(feature = "auth-from-header")]
pub const AUTH_HEADER_NAME: &'static str = "Authentication";

#[async_trait]
impl<'r, T: Login, P: Permission> FromRequest<'r> for Authenticated<T, P> {
    type Error = &'static str;

    #[cfg(feature = "auth-from-cookie")]
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request.cookies().get_private(AUTH_COOKIE_NAME);
        if cookie.is_none() {
            return Outcome::Error((Status::Unauthorized, "Cookie not found"));
        }

        handle_parse_token(cookie.unwrap().value()).await
    }
    
    #[cfg(all(feature = "auth-from-header", not(feature = "auth-from-cookie")))]
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers().get(AUTH_HEADER_NAME).collect::<Vec<&str>>();
        if headers.is_empty() {
            return Outcome::Error((Status::Unauthorized, "Header not found"));
        }

        handle_parse_token(headers.first().unwrap()).await
    }
}

async fn handle_parse_token<T: Login, P: Permission>(input: &str) -> rocket::outcome::Outcome<Authenticated<T, P>, (Status, &'static str), Status> {
    match LoginClaim::read_token(input) {
        Ok(claim) => {
            if claim.login_name != T::LOGIN_NAME {
                return Outcome::Error((Status::Unauthorized, "Invalid login"))
            }

            if claim.valid_to > OffsetDateTime::now_utc() {
                let mut conn = DbPool::get_conn().await.unwrap();
                let user = T::get_by_id(claim.id, &mut conn).await.unwrap();
                if let Some(user) = user {
                    let roles = user.get_roles(&mut conn).await;
                    if **roles != claim.roles || !P::is_granted(&roles) {
                        return Outcome::Error((Status::Unauthorized, "Outdated login"))
                    }

                    Outcome::Success(Authenticated(user, PhantomData))
                } else {
                    Outcome::Error((Status::Unauthorized, "User not found"))
                }
            } else {
                Outcome::Error((Status::Unauthorized, "Login expired"))
            }
        },
        Err(_) => Outcome::Error((Status::Unauthorized, "Failed to read token")),
    }
}
