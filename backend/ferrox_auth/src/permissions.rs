use std::marker::PhantomData;
use crate::{Role, Roles};

/// Trait defining permission checking for tuples and [AnyPerm].
///
/// ### Implementations
/// - Tuples of [Permission] will act as AND operator, so all permissions must be true
/// - [Role] all implement [Permission] and will check for [Role::ROLE_NAME] in [Roles]
/// - [AnyPerm] takes a tuple of [Permission] where any of them may be true
/// - [NotPerm] negates the value of a [Permission]
pub trait Permission {
    /// Check for all conditions to be met.
    fn is_granted(roles: &Roles) -> bool;
}

/// Helper trait for [AnyPerm] functionality.
pub trait AnyPermissions {
    /// Check for any [Permission] of this tuple to be true. (OR operation)
    fn is_granted(roles: &Roles) -> bool;
}

impl<R: Role> Permission for R {
    fn is_granted(roles: &Roles) -> bool {
        roles.is_granted::<R>()
    }
}

impl Permission for () {
    fn is_granted(_: &Roles) -> bool {
        true
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Permission),+> Permission for ($($name),+) {
            fn is_granted(roles: &Roles) -> bool {
                $(<$name>::is_granted(roles))&&+
            }
        }

        impl<$($name: Permission),+> AnyPermissions for ($($name),+) {
            fn is_granted(roles: &Roles) -> bool {
                $(<$name>::is_granted(roles))||+
            }
        }
    };
}

tuple_impls!(R1 R2);
tuple_impls!(R1 R2 R3);
tuple_impls!(R1 R2 R3 R4);
tuple_impls!(R1 R2 R3 R4 R5);
tuple_impls!(R1 R2 R3 R4 R5 R6);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9 R10);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9 R10 R11);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9 R10 R11 R12);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9 R10 R11 R12 R13);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9 R10 R11 R12 R13 R14);
tuple_impls!(R1 R2 R3 R4 R5 R6 R7 R8 R9 R10 R11 R12 R13 R14 R15);

/// Used to check for [AnyPermissions] to be true in a [crate::Authenticated] request guard.
pub struct AnyPerm<T: AnyPermissions>(PhantomData<T>);

impl<T: AnyPermissions> Permission for AnyPerm<T> {
    fn is_granted(roles: &Roles) -> bool {
        T::is_granted(roles)
    }
}

/// Used to negate the check for a [Permission].
pub struct NotPerm<T: Permission>(PhantomData<T>);

impl<T: Permission> Permission for NotPerm<T> {
    fn is_granted(roles: &Roles) -> bool {
        !T::is_granted(roles)
    }
}


