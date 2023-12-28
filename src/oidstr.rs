use std::{fmt, str::FromStr};

use data_encoding::BASE32HEX_NOPAD;
#[cfg(feature = "uuid_v7")]
use uuid::timestamp::{context::NoContext, Timestamp};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    prefix::Prefix,
    uuid::uuid_from_str,
};

/// An Object ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OidStr {
    prefix: Prefix,
    uuid: Uuid,
}

impl OidStr {
    /// Create a new OID with a given [`Prefix`] and generating a new UUID
    ///
    /// **NOTE:** The Prefix must be ASCII characters (this restriction is
    /// arbitrary and could be lifted in the future by exposing an API to
    /// tune the [`Prefix`] length)
    #[cfg(feature = "uuid_v4")]
    pub fn new_v4<P>(prefix: P) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Self::with_uuid(prefix, Uuid::new_v4())
    }

    /// Create a new OID with a given [`Prefix`] and generating a new UUIDv7
    /// (UNIX Epoch based on current system clock)
    #[cfg(feature = "uuid_v7")]
    pub fn new_v7_now<P>(prefix: P) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Self::with_uuid(prefix, Uuid::new_v7(Timestamp::now(NoContext)))
    }

    /// Create a new OID with a given [`Prefix`] and generating a new UUIDv7
    /// (UNIX Epoch based)
    #[cfg(feature = "uuid_v7")]
    pub fn new_v7<P>(prefix: P, ts: Timestamp) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Self::with_uuid(prefix, Uuid::new_v7(ts))
    }

    /// Create a new OID with a given [`Prefix`] and a given UUID. If the UUID
    /// is not a version 7 an error isr returned.
    ///
    /// **NOTE:** The Prefix must be 3 ASCII characters (this restriction is
    /// arbitrary and could be lifted in the future by exposing an API to
    /// tune the [`Prefix`] length)
    pub fn with_uuid<P>(prefix: P, uuid: Uuid) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Ok(Self {
            prefix: prefix.try_into()?,
            uuid,
        })
    }

    /// Get the [`Prefix`] of the OID
    pub fn prefix(&self) -> &Prefix { &self.prefix }

    /// Get the value portion of the  of the OID, which is the base32 encoded
    /// string following the `-` separator
    pub fn value(&self) -> String { BASE32HEX_NOPAD.encode(self.uuid.as_bytes()) }

    /// Get the UUID of the OID
    pub fn uuid(&self) -> &Uuid { &self.uuid }
}

impl FromStr for OidStr {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((pfx, val)) = s.split_once('-') {
            if pfx.is_empty() {
                return Err(Error::MissingPrefix);
            }

            return Ok(Self {
                prefix: pfx.parse()?,
                uuid: uuid_from_str(val)?,
            });
        }

        Err(Error::MissingSeparator)
    }
}

impl fmt::Display for OidStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.prefix, self.value())
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for OidStr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for OidStr {
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
    use wildmatch::WildMatch;

    use super::*;

    #[test]
    #[cfg(feature = "uuid_v4")]
    fn oid_to_str_v4() -> Result<()> {
        let oid = OidStr::new_v4("TST")?;
        assert!(WildMatch::new("TST-??????????????????????????").matches(&oid.to_string()));
        Ok(())
    }

    #[test]
    #[cfg(feature = "uuid_v7")]
    fn oid_to_str_v7() -> Result<()> {
        let oid = OidStr::new_v7_now("TST")?;
        assert!(WildMatch::new("TST-??????????????????????????").matches(&oid.to_string()));
        Ok(())
    }

    #[test]
    fn str_to_oid() {
        let res = "TST-0OQPKOAADLRUJ000J7U2UGNS2G".parse::<OidStr>();
        assert_eq!(
            res.unwrap(),
            OidStr {
                prefix: "TST".parse().unwrap(),
                uuid: "06359a61-4a6d-77e9-8000-99fc2f42fc14".parse().unwrap(),
            }
        );
    }

    #[test]
    fn str_to_oid_err_prefix() {
        let res = "-0OQPKOAADLRUJ000J7U2UGNS2G".parse::<OidStr>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingPrefix);
    }

    #[test]
    fn str_to_oid_err_value() {
        let res = "TST-".parse::<OidStr>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingValue);
    }

    #[test]
    fn str_to_oid_err_decode() {
        let res = "TST-&OQPKOAADLRUJ000J7U2UGNS2G".parse::<OidStr>();
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::Base32Decode(_)));
    }

    #[test]
    fn str_to_oid_err_no_sep() {
        let res = "0OQPKOAADLRUJ000J7U2UGNS2G".parse::<OidStr>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingSeparator);
    }

    #[test]
    fn str_to_oid_err_two_sep() {
        let res = "TST-0OQPKOAAD-LRUJ000J7U2UGNS2G".parse::<OidStr>();
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::Base32Decode(_)));
    }

    #[test]
    fn oid_to_uuid() {
        let oid: OidStr = "TST-0OQPKOAADLRUJ000J7U2UGNS2G".parse().unwrap();
        assert_eq!(
            oid.uuid(),
            &"06359a61-4a6d-77e9-8000-99fc2f42fc14"
                .parse::<Uuid>()
                .unwrap()
        );
    }
}
