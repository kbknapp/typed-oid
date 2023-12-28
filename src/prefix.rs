use smallvec::SmallVec;
use std::{fmt, str::FromStr};

use crate::error::{Error, Result};

#[inline]
pub(crate) fn valid_prefix_char(c: u8) -> bool {
    (c > b'/' && c < b':') || (c > b'`' && c < b'{') || (c > b'@' && c < b'[')
}

#[cfg(test)]
mod valid_prefix_char_tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(valid_prefix_char(b'0'));
        assert!(valid_prefix_char(b'9'));
        assert!(valid_prefix_char(b'A'));
        assert!(valid_prefix_char(b'Z'));
        assert!(valid_prefix_char(b'a'));
        assert!(valid_prefix_char(b'z'));
    }

    #[test]
    fn invalid() {
        assert!(!valid_prefix_char(b'/'));
        assert!(!valid_prefix_char(b':'));
        assert!(!valid_prefix_char(b'`'));
        assert!(!valid_prefix_char(b'{'));
        assert!(!valid_prefix_char(b'@'));
        assert!(!valid_prefix_char(b'['));
    }
}

/// An Object ID Prefix designed to be similar to a human readable "subject
/// line" for the OID
///
/// The prefix can store up to 8 bytes of 7-bit ASCII characters inline; a
/// prefix of longer than 8 bytes will be "spilled" to the heap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefix {
    bytes: SmallVec<[u8; 8]>,
}

impl Prefix {
    /// Create a Prefix from a slice of bytes. The bytes must be ASCII values of
    /// `0-9`, `A-Z`, or `a-z`, additionally the byte slice length must be
    /// equal to the prefix length.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        // Checking for ASCII 0-9,A-Z,a-z
        if !slice.iter().all(|&c| valid_prefix_char(c)) {
            return Err(Error::InvalidPrefix {
                valid_until: slice
                    .iter()
                    .enumerate()
                    .find(|(_i, &c)| !valid_prefix_char(c))
                    .map(|(i, _)| i)
                    .unwrap(),
            });
        }
        Ok(Self::from_slice_unchecked(slice))
    }

    /// Create a Prefix from a slice of bytes without checking the length or
    /// validity of the bytes
    pub fn from_slice_unchecked(slice: &[u8]) -> Self {
        Self {
            bytes: SmallVec::from_slice(slice),
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: self.bytes must not contain any invalid UTF-8. We don't expose the
        // inner byte array for manipulation, and the only way to construct self
        // checks for a subset of 7-bit ASCII which itself is a subset of UTF-8
        unsafe {
            write!(
                f,
                "{}",
                std::str::from_utf8_unchecked(self.bytes.as_slice())
            )
        }
    }
}

impl FromStr for Prefix {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> { Self::from_slice(s.as_bytes()) }
}

impl TryFrom<&[u8]> for Prefix {
    type Error = Error;

    fn try_from(slice: &[u8]) -> std::result::Result<Self, Self::Error> { Self::from_slice(slice) }
}

impl TryFrom<&str> for Prefix {
    type Error = Error;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> { s.parse() }
}

#[cfg(test)]
mod prefix_tests {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn from_str() {
        let pfx = "PFX".parse::<Prefix>();
        assert!(pfx.is_ok());
        assert_eq!(
            pfx.unwrap(),
            Prefix {
                bytes: smallvec![b'P', b'F', b'X']
            }
        );
    }

    #[test]
    fn from_str_err_char() {
        let pfx = "PF[".parse::<Prefix>();
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::InvalidPrefix { valid_until: 2 });
    }

    #[test]
    fn from_str_mixedcase() {
        let pfx = "PFx".parse::<Prefix>();
        assert!(pfx.is_ok());
        assert_eq!(
            pfx.unwrap(),
            Prefix {
                bytes: smallvec![b'P', b'F', b'x']
            }
        );
    }

    #[test]
    fn from_slice() {
        let arr: [u8; 3] = [b'P', b'F', b'X'];
        let pfx = Prefix::from_slice(arr.as_slice());
        assert!(pfx.is_ok());
        assert_eq!(
            pfx.unwrap(),
            Prefix {
                bytes: SmallVec::from_slice(&arr)
            }
        );
    }

    #[test]
    fn from_slice_err_char() {
        let arr: [u8; 3] = [b'P', b'F', b']'];
        let pfx = Prefix::from_slice(arr.as_slice());
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::InvalidPrefix { valid_until: 2 });
    }

    #[test]
    fn from_slice_mixedcase() {
        let arr: [u8; 3] = [b'P', b'F', b'x'];
        let pfx = Prefix::from_slice(arr.as_slice());
        assert!(pfx.is_ok());
        assert_eq!(
            pfx.unwrap(),
            Prefix {
                bytes: smallvec![b'P', b'F', b'x']
            }
        );
    }

    #[test]
    fn to_string() {
        let pfx: Prefix = "PFx".parse().unwrap();
        assert_eq!("PFx".to_string(), pfx.to_string());
    }
}
