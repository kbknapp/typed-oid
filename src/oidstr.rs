use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use data_encoding::BASE32HEX_NOPAD;
#[cfg(feature = "uuid_v7")]
use uuid::timestamp::{context::NoContext, Timestamp};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    prefix::Prefix,
    uuid::uuid_from_str_b32h,
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
    /// > **NOTE:** The Prefix must be ASCII characters of `A-Z,a-z,0-9` (this
    /// > restriction is arbitrary and could be lifted in the future.
    #[cfg(feature = "uuid_v4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_v4")))]
    pub fn new_v4<P>(prefix: P) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Ok(Self {
            prefix: prefix.try_into()?,
            uuid: Uuid::new_v4(),
        })
    }

    /// Create a new OID with a given [`Prefix`] and generating a new UUIDv7
    /// (UNIX Epoch based on current system clock)
    #[cfg(feature = "uuid_v7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_v7")))]
    pub fn new_v7_now<P>(prefix: P) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Ok(Self {
            prefix: prefix.try_into()?,
            uuid: Uuid::new_v7(Timestamp::now(NoContext)),
        })
    }

    /// Create a new OID with a given [`Prefix`] and generating a new UUIDv7
    /// (UNIX Epoch based)
    #[cfg(feature = "uuid_v7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_v7")))]
    pub fn new_v7<P>(prefix: P, ts: Timestamp) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Self::with_uuid(prefix, Uuid::new_v7(ts))
    }

    /// Create a new OID with a given [`Prefix`] and a given UUID.
    ///
    /// > **NOTE:** The Prefix must be ASCII characters of `A-Z,a-z,0-9` (this
    /// > restriction is arbitrary and could be lifted in the future.
    pub fn with_uuid<P>(prefix: P, uuid: Uuid) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Ok(Self {
            prefix: prefix.try_into()?,
            uuid,
        })
    }

    /// Create a new OID with a given [`Prefix`] and a given string-ish UUID.
    ///
    /// > **NOTE:** The Prefix must be ASCII characters of `A-Z,a-z,0-9` (this
    /// > restriction is arbitrary and could be lifted in the future.
    pub fn try_with_uuid<P, S>(prefix: P, uuid: S) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
        S: AsRef<str>,
    {
        Self::with_uuid(prefix, uuid.as_ref().try_into()?)
    }

    /// Attemp to create an Oid from a base32hex encoded UUID string-ish value
    pub fn try_with_uuid_base32<P, S>(prefix: P, base32_uuid: S) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
        S: AsRef<str>,
    {
        Self::with_uuid(prefix, uuid_from_str_b32h(base32_uuid.as_ref())?)
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
                uuid: uuid_from_str_b32h(val)?,
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

impl Hash for OidStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prefix.hash(state);
        self.uuid.hash(state);
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl ::serde::Serialize for OidStr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
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
#[cfg(any(feature = "uuid_v4", feature = "uuid_v7"))]
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
    fn str_to_oid_long() {
        let res = "TestingTesting-0OQPKOAADLRUJ000J7U2UGNS2G".parse::<OidStr>();
        assert_eq!(
            res.unwrap(),
            OidStr {
                prefix: "TestingTesting".parse().unwrap(),
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

    #[test]
    fn from_uuid_str() {
        let oid = OidStr::try_with_uuid("Tst", "063dc3a0-3925-7c7f-8000-ca84a12ee183").unwrap();
        assert!(
            WildMatch::new("Tst-??????????????????????????").matches(&oid.to_string()),
            "{oid}"
        );
    }

    #[test]
    fn from_uuid_str_b32h() {
        let oid = OidStr::try_with_uuid_base32("Tst", "0OUS781P4LU7V000PA2A2BN1GC").unwrap();
        assert_eq!("Tst-0OUS781P4LU7V000PA2A2BN1GC", &oid.to_string());
    }

    #[test]
    fn hash() {
        use std::collections::HashMap;
        let oid: OidStr = "TST-0OQPKOAADLRUJ000J7U2UGNS2G".parse().unwrap();

        let mut map = HashMap::new();
        map.insert(oid, "test");
    }
}
