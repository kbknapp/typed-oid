use std::{fmt, marker::PhantomData, str::FromStr};

use data_encoding::BASE32HEX_NOPAD;
#[cfg(feature = "uuid_v7")]
use uuid::timestamp::{context::NoContext, Timestamp};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    prefix::Prefix,
    uuid::uuid_from_str,
    OidPrefix,
};

/// A Typed Object ID where the Prefix is part of the type
///
/// # Examples
///
/// A nice property of this two different prefix are two different types, and
/// thus the following fails to compile:
///
/// ```compile_fail
/// # use typed_oid::{Oid,OidPrefix};
/// struct A;
/// impl OidPrefix for A {}
///
/// struct B;
/// impl OidPrefix for B {}
///
/// // The same UUID for both
/// let uuid = Uuid::new_v4();
/// let oid_a: Oid<A> = Oid::with_uuid(uuid.clone());
/// let oid_b: Oid<B> = Oid::with_uuid(uuid);
///
/// // This fails to compile because `Oid<A>` is a different type than `Oid<B>` and no
/// // PartialEq or Eq is implemented between these two types.
/// oid_a == oid_b
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Oid<P> {
    uuid: Uuid,
    // Using fn for variance (invariant with respect to P) whereas using *mut would also be
    // invariant with respect for P, but would then now allow the Auto-traits Send+Sync.
    _prefix: PhantomData<fn(P) -> P>,
}

impl<P: OidPrefix> Oid<P> {
    /// Create a new `Oid` with a UUIDv4 (random)
    #[cfg(feature = "uuid_v4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_v4")))]
    pub fn new_v4() -> Self { Self::with_uuid(Uuid::new_v4()) }

    /// Create a new `Oid` with a UUIDv7 (UNIX Epoch based for current system
    /// clock)
    #[cfg(feature = "uuid_v7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_v7")))]
    pub fn new_v7_now() -> Self { Self::with_uuid(Uuid::new_v7(Timestamp::now(NoContext))) }

    /// Create a new `Oid` with a UUIDv7 (UNIX Epoch based)
    #[cfg(feature = "uuid_v7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_v7")))]
    pub fn new_v7(ts: Timestamp) -> Self { Self::with_uuid(Uuid::new_v7(ts)) }

    /// Create a new Oid with a given UUID
    pub fn with_uuid(uuid: Uuid) -> Self {
        Self {
            uuid,
            _prefix: PhantomData,
        }
    }

    /// Get the [`Prefix`] of the TOID
    ///
    /// # Panics
    ///
    /// If the Type `P` translates to an invalid prefix
    pub fn prefix(&self) -> Prefix { Prefix::from_str(P::prefix()).expect("Invalid Prefix") }

    /// Get the value portion of the  of the TOID, which is the base32 encoded
    /// string following the `-` separator
    pub fn value(&self) -> String { BASE32HEX_NOPAD.encode(self.uuid.as_bytes()) }

    /// Get the UUID of the TOID
    pub fn uuid(&self) -> &Uuid { &self.uuid }
}

impl<P: OidPrefix> fmt::Display for Oid<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", P::prefix(), self.value())
    }
}

impl<P: OidPrefix> FromStr for Oid<P> {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((pfx, val)) = s.split_once('-') {
            if pfx.is_empty() {
                return Err(Error::MissingPrefix);
            }

            if pfx != P::prefix() {
                return Err(Error::InvalidPrefix {
                    valid_until: pfx
                        .chars()
                        .zip(P::prefix().chars())
                        .enumerate()
                        .find(|(_i, (c1, c2))| c1 != c2)
                        .map(|(i, _)| i)
                        .unwrap(),
                });
            }

            return Ok(Self {
                uuid: uuid_from_str(val)?,
                _prefix: PhantomData,
            });
        }

        Err(Error::MissingSeparator)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<P: OidPrefix> ::serde::Serialize for Oid<P> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, P: OidPrefix> ::serde::Deserialize<'de> for Oid<P> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(::serde::de::Error::custom)
    }
}

#[cfg(test)]
mod oid_tests {
    #[cfg(any(feature = "uuid_v4", feature = "uuid_v7"))]
    use wildmatch::WildMatch;

    #[cfg(any(feature = "uuid_v4", feature = "uuid_v7"))]
    use super::*;

    #[test]
    #[cfg(any(feature = "uuid_v4", feature = "uuid_v7"))]
    fn typed_oid() {
        #[derive(Debug)]
        struct Tst;
        impl OidPrefix for Tst {}

        #[cfg_attr(all(feature = "uuid_v4", feature = "uuid_v7"), allow(unused_variables))]
        #[cfg(feature = "uuid_v4")]
        let oid: Oid<Tst> = Oid::new_v4();
        #[cfg(feature = "uuid_v7")]
        let oid: Oid<Tst> = Oid::new_v7_now();
        assert!(
            WildMatch::new("Tst-??????????????????????????").matches(&oid.to_string()),
            "{oid}"
        );

        let res = "Tst-0OUS781P4LU7V000PA2A2BN1GC".parse::<Oid<Tst>>();
        assert!(res.is_ok());
        let oid: Oid<Tst> = res.unwrap();
        assert_eq!(
            oid.uuid(),
            &"063dc3a0-3925-7c7f-8000-ca84a12ee183"
                .parse::<Uuid>()
                .unwrap()
        );

        let res = "Frm-0OUS781P4LU7V000PA2A2BN1GC".parse::<Oid<Tst>>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::InvalidPrefix { valid_until: 0 });
    }

    #[test]
    #[cfg(any(feature = "uuid_v4", feature = "uuid_v7"))]
    fn long_typed_oid() {
        #[derive(Debug)]
        struct TestingTesting;
        impl OidPrefix for TestingTesting {}

        #[cfg_attr(all(feature = "uuid_v4", feature = "uuid_v7"), allow(unused_variables))]
        #[cfg(feature = "uuid_v4")]
        let oid: Oid<TestingTesting> = Oid::new_v4();
        #[cfg(feature = "uuid_v7")]
        let oid: Oid<TestingTesting> = Oid::new_v7_now();
        assert!(
            WildMatch::new("TestingTesting-??????????????????????????").matches(&oid.to_string()),
            "{oid}"
        );

        let res = "TestingTesting-0OUS781P4LU7V000PA2A2BN1GC".parse::<Oid<TestingTesting>>();
        assert!(res.is_ok());
        let oid: Oid<TestingTesting> = res.unwrap();
        assert_eq!(
            oid.uuid(),
            &"063dc3a0-3925-7c7f-8000-ca84a12ee183"
                .parse::<Uuid>()
                .unwrap()
        );

        let res = "Frm-0OUS781P4LU7V000PA2A2BN1GC".parse::<Oid<TestingTesting>>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::InvalidPrefix { valid_until: 0 });
    }
}
}
