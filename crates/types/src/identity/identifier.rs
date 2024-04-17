use std::{fmt, str::FromStr};

use super::{
    did::{Did, InvalidDid, PlcId},
    handle::{Handle, InvalidHandle},
};

/// An [`at-identifier`][]: a [DID][Did] or a [handle][Handle].
///
/// [`at-identifier`]: https://atproto.com/specs/lexicon#at-identifier
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Identifier {
    Did(Did),
    Handle(Handle),
}

impl From<Did> for Identifier {
    fn from(value: Did) -> Self {
        Self::Did(value)
    }
}

#[cfg(feature = "plc")]
#[cfg_attr(docsrs, doc(cfg(feature = "plc")))]
impl From<PlcId> for Identifier {
    fn from(value: PlcId) -> Self {
        Self::Did(Did::from(value))
    }
}

impl From<Handle> for Identifier {
    fn from(value: Handle) -> Self {
        Self::Handle(value)
    }
}

impl FromStr for Identifier {
    type Err = InvalidIdentifier;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("did:") {
            Self::Did(s.parse().map_err(InvalidIdentifier::from)?)
        } else {
            Self::Handle(s.parse().map_err(InvalidIdentifier::from)?)
        })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Did(did) => write!(f, "{did}"),
            Identifier::Handle(handle) => write!(f, "{handle}"),
        }
    }
}

#[derive(thiserror::Error, PartialEq, Clone, Debug)]
pub enum InvalidIdentifier {
    #[error("invalid DID: {0}")]
    Did(#[from] InvalidDid),
    #[error("invalid handle: {0}")]
    Handle(#[from] InvalidHandle),
}

impl InvalidIdentifier {
    /// Create a new [`InvalidIdentifier`] denoting an
    /// [empty handle][InvalidHandle::Empty].
    pub const fn empty() -> Self {
        Self::Handle(InvalidHandle::Empty)
    }
}
