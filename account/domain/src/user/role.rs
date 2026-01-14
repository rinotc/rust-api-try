use strum::{Display, EnumIter, EnumString};

/// Represents the role of a user within the system.
///
/// This enum defines the possible roles that a user can have, and it supports various
/// serialization, display, and iteration functionalities through the use of derived traits.
///
/// # Variants
///
/// * `Admin` - Represents a user with administrative privileges.
/// * `User` - Represents a regular user with standard privileges.
///
/// # Derived Traits
///
/// - `Debug`: Enables debugging output functionality for the enum.
/// - `Clone`: Allows cloning of the enum instances.
/// - `PartialEq` and `Eq`: Enable comparison of enum instances for equality.
/// - `Display`: Provides a human-readable string representation of the enum variants.
/// - `EnumString`: Allows parsing a string into a variant of the enum.
/// - `EnumIter`: Enables iteration over all possible variants of the enum.
///
/// # Attributes
///
/// * `#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]`
///   - This attribute applies to the enum and its variants to ensure that their serialized
///     string representations use SCREAMING_SNAKE_CASE format when converted using Strum's serialization utilities.
///
/// # Examples
///
/// ```rust
/// use strum::IntoEnumIterator;
/// use crate::UserRole;
///
/// // Parsing from string
/// let admin: UserRole = "ADMIN".parse().unwrap();
/// assert_eq!(admin, UserRole::Admin);
///
/// // Serializing to string
/// let user = UserRole::User;
/// assert_eq!(user.to_string(), "USER");
///
/// // Iterating over variants
/// for role in UserRole::iter() {
///     println!("{:?}", role);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString, EnumIter)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}
