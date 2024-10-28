use std::ops::{Deref, DerefMut};

/// Provides a convenient api for checking permissions.
pub struct Roles<'a>(pub &'a Vec<String>);

impl<'a> Deref for Roles<'a> {
    type Target = &'a Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Provides a convenient api for modifying permissions.
pub struct RolesMut<'a>(pub &'a mut Vec<String>);

impl<'a> Deref for RolesMut<'a> {
    type Target = &'a mut Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for RolesMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! common_functions {
    () => {
        /// Checks for the provided role.
        pub fn is_granted<T: Role>(&self) -> bool {
            self.0.contains(&T::ROLE_NAME.to_string())
        }
    };
}

impl<'a> Roles<'a> {
    common_functions!();
}

impl<'a> RolesMut<'a> {
    common_functions!();

    /// Adds a [Role].
    ///
    /// This will not add duplicates and will ignore such instructions.
    pub fn add_role<T: Role>(&mut self) {
        let str = T::ROLE_NAME.to_string();
        if !self.0.contains(&str) {
            self.0.push(str);
        }
    }

    /// Removes a [Role].
    ///
    /// If this role is not granted, nothing will happen.
    pub fn remove_role<T: Role>(&mut self) {
        if let Some(index) = self.0.iter().position(|v| v == T::ROLE_NAME) {
            self.0.remove(index);
        }
    }
}

/// Marks a struct as a role representation.
///
/// A role should be defined with the `define_role!` macro.
pub trait Role {
    /// String representation of this role.
    ///
    /// Usually starts with "ROLE_".
    const ROLE_NAME: &'static str;
}

/// Defines a role struct implementing [Role].
///
/// # Arguments
/// - Identifier of the struct (this macro will create the struct)
/// - Value for [Role::ROLE_NAME]
#[macro_export]
macro_rules! define_role {
    ($structName:tt, $value:expr) => {
        #[allow(missing_docs)]
        pub struct $structName;

        impl ferrox_auth::Role for $structName {
            const ROLE_NAME: &'static str = $value;
        }
    };
}

use crate as ferrox_auth;
define_role!(RoleUser, "ROLE_USER");