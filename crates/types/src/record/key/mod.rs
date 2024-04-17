use std::{convert::Infallible, fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rkey")]
pub(crate) mod tid;

/// The [key] of an AT protocol record; the unique identifier of the record
/// within its [repository][] and collection.
///
/// [key]: https://atproto.com/specs/record-key
/// [repository]: https://atproto.com/guides/overview#data-repositories
#[cfg(feature = "plc")]
#[cfg_attr(docsrs, doc(cfg(feature = "plc")))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Rkey {
    /// The record key is the literal string `self`, indicating the "collection"
    /// of this record type only has a single entry (e.g.
    /// `app.bsky.actor.profile`).
    ///
    /// (`Self` is not used as the enum variant name because it is a Rust
    /// [keyword][].)
    ///
    /// [keyword]: https://doc.rust-lang.org/reference/keywords.html#strict-keywords
    #[doc(alias = "Self")]
    Unique,

    /// The record key is an AT protocol [timestamp ID][].
    ///
    /// [timestamp ID]: https://atproto.com/specs/record-key#record-key-type-tid
    Tid(tid::Tid),

    /// This record uses an unrecognized key scheme.
    Custom(String),
}

impl Rkey {
    pub fn new(value: impl AsRef<str>) -> Self {
        value.as_ref().parse().unwrap()
    }
}

impl FromStr for Rkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "self" {
            Self::Unique
        } else if let Ok(tid) = s.parse() {
            Self::Tid(tid)
        } else {
            Self::Custom(s.to_owned())
        })
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Rkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("unknown record key"))
    }
}

impl fmt::Display for Rkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rkey::Unique => write!(f, "self"),
            Rkey::Tid(id) => write!(f, "{id}"),
            Rkey::Custom(id) => write!(f, "{id}"),
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Rkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<tid::Tid> for Rkey {
    fn from(value: tid::Tid) -> Self {
        Self::Tid(value)
    }
}

#[cfg(not(feature = "plc"))]
pub type Rkey = String;
