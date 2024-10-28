//! Contains various modules implementing an authentication system.
//!
//! Core of this system are [Login], [Authenticated] and [Permission].

mod login;
mod authenticated;
mod roles;
mod permissions;

pub use authenticated::*;
pub use login::*;
pub use permissions::*;
pub use roles::*;
