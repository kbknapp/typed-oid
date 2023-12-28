use data_encoding::BASE32HEX_NOPAD;
use uuid::Uuid;

use crate::error::{Error, Result};

pub(crate) fn uuid_from_str(s: &str) -> Result<Uuid> {
    if s.is_empty() {
        return Err(Error::MissingValue);
    }
    Ok(Uuid::from_slice(&BASE32HEX_NOPAD.decode(s.as_bytes())?)?)
}
