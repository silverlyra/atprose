use std::{fmt, str::FromStr};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use crate::encoding::{decode_u64, encode_u64, DecodeError};

/// A [timestamp identifier][tid].
///
/// [tid]: https://atproto.com/specs/record-key#record-key-type-tid
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[repr(transparent)]
pub struct Tid(u64);

impl Tid {
    #[inline]
    pub fn new(ts: u64, seq: u16) -> Self {
        let ts = (ts & 0x1F_FFFF_FFFF_FFFF) << 10;
        let seq = (seq & 0x3FF) as u64;

        Self(ts | seq)
    }

    pub const fn timestamp(&self) -> u64 {
        (self.0 >> 10) & 0x1FFF_FFFF_FFFF_FFFF
    }

    pub const fn seq(&self) -> u16 {
        (self.0 & 0x3FF) as u16
    }

    #[cfg(feature = "chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
    pub fn datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_micros(self.timestamp() as i64)
            .expect("beyond domain of chrono::DateTime")
    }

    pub fn decode(input: impl AsRef<str>) -> Result<Self, DecodeError> {
        let tid = decode_u64(input)?;

        Ok(Self(tid))
    }

    pub fn encode(&self) -> String {
        encode_u64(self.0)
    }
}

impl FromStr for Tid {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode(s)
    }
}

impl fmt::Display for Tid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.encode();
        write!(f, "{id}")
    }
}

impl fmt::Debug for Tid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.encode();
        f.debug_tuple("Tid").field(&id).finish()
    }
}

impl From<Tid> for u64 {
    fn from(value: Tid) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    #![cfg_attr(not(feature = "chrono"), allow(unused_variables))]

    use super::Tid;

    #[test]
    fn test_create_tid() {
        let ts = 1_707_228_000_000_000;
        let id = Tid::new(ts, 511);
        assert_eq!(ts, id.timestamp());
        assert_eq!(511, id.seq());
        assert_eq!(id.0, 0x1842dbf9f66001ff);
        assert_eq!("3kkqvzbva22jz".to_owned(), id.to_string());
    }

    #[test]
    fn test_decode_tid() {
        let cases = [
            (
                "3kkqvzbva22jz",
                0x1842dbf9f66001ff,
                "2024-02-06 14:00:00 UTC",
                1_707_228_000_000_000,
                511,
            ),
            (
                "3kqcaxrhm7q22",
                0x185906eddb22d800,
                "2024-04-17 02:37:14.073270 UTC",
                1713321434073270,
                0,
            ),
            (
                "3kljftdquw52e",
                0x1845ebca6dae0c0a,
                "2024-02-16 07:46:54.218115 UTC",
                1708069614218115,
                10,
            ),
            (
                "3jui7kd54zh2y",
                0x17e9c582462fb41e,
                "2023-04-29 03:42:21.953005 UTC",
                1682739741953005,
                30,
            ),
        ];

        for (value, expected, dt, ts, clock) in cases {
            let id = Tid::decode(value).expect(value);
            assert_eq!(id.0, expected, "{:016x} != {expected:016x}", id.0);

            assert_eq!(ts, id.timestamp(), "{ts:016x} != {:016x}", id.timestamp());
            assert_eq!(clock, id.seq());

            #[cfg(feature = "chrono")]
            assert_eq!(dt.to_owned(), id.datetime().to_string());
        }
    }
}
