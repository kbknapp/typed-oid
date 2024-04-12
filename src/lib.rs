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
}
