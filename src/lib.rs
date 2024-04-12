#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
mod oid;
mod oidstr;
mod prefix;
mod uuid;

pub use crate::{
    error::{Error, Result},
    oid::Oid,
    oidstr::OidStr,
    prefix::Prefix,
};

/// Defines the converting a type to a prefix of an OID
///
/// > **NOTE**
/// > This should be a static representation of the type irrelevant of any
/// > value in a type instance
pub trait OidPrefix {
    /// Get the static string representation of the prefix.
    ///
    /// The default representation is to use the type name itself.
    fn prefix() -> &'static str { std::any::type_name::<Self>().split(':').last().unwrap() }

    /// A partial equality check for the prefix. This is useful in cases when
    /// converting from a string to an Typed-OID where the type and string
    /// prefix are not the same.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_oid::{Oid, OidPrefix};
    /// #[derive(Debug)]
    /// struct A;
    /// impl OidPrefix for A {
    ///     fn str_partial_eq(s: &str) -> bool { "apple" == s }
    /// }
    ///
    /// let oid: Oid<A> = "apple-4GKFGPRVND4QT3PDR90PDKF66O".parse().unwrap();
    /// ```
    fn str_partial_eq(s: &str) -> bool { Self::prefix() == s }
}
