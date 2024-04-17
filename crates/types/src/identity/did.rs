use std::{fmt, str::FromStr};

use super::handle::{Handle, InvalidHandle};
use crate::encoding::{decode_into, encode, DecodeError};

/// A valid [AT protocol DID][did]: either a `did:plc` [identifier][PlcId], or a
/// `did:web` [handle][Handle].
///
/// [did]: https://atproto.com/specs/did
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Did {
    /// A [`did:plc`][plc] identifier, with its globally-unique [ID][PlcId].
    ///
    /// [plc]: https://web.plc.directory/
    Plc(PlcId),
    /// A [`did:web`][web] identifier, restricted to contain only valid AT
    /// protocol [handles][Handle].
    ///
    /// [web]: https://w3c-ccg.github.io/did-method-web/
    Web(Handle),
}

#[cfg(feature = "plc")]
#[cfg_attr(docsrs, doc(cfg(feature = "plc")))]
impl From<PlcId> for Did {
    fn from(value: PlcId) -> Self {
        Self::Plc(value)
    }
}

impl From<Handle> for Did {
    fn from(value: Handle) -> Self {
        Self::Web(value)
    }
}

impl FromStr for Did {
    type Err = InvalidDid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let did = s.strip_prefix("did:").ok_or(InvalidDid::Prefix)?;

        let (scheme, id) = did.split_once(':').ok_or(InvalidDid::Scheme)?;

        let parsed = match scheme {
            "plc" => Self::Plc(id.parse().map_err(InvalidDid::from)?),
            "web" => Self::Web(id.parse().map_err(InvalidDid::from)?),
            _ => return Err(InvalidDid::Scheme),
        };

        Ok(parsed)
    }
}

impl fmt::Display for Did {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Did::Plc(id) => write!(f, "did:plc:{id}"),
            Did::Web(handle) => write!(f, "did:web:{handle}"),
        }
    }
}

#[derive(thiserror::Error, PartialEq, Clone, Debug)]
pub enum InvalidDid {
    #[error("missing did: prefix")]
    Prefix,
    #[error("unknown did scheme")]
    Scheme,
    #[error("invalid did:plc: {0}")]
    Plc(DecodeError),
    #[error("invalid did:web: {0}")]
    Web(#[from] InvalidHandle),
}

impl From<DecodeError> for InvalidDid {
    fn from(value: DecodeError) -> Self {
        Self::Plc(value)
    }
}

/// An identifier in the [`plc` DID scheme][scheme]
///
/// [scheme]: https://web.plc.directory/spec/v0.1/did-plc
#[cfg(feature = "plc")]
#[cfg_attr(docsrs, doc(cfg(feature = "plc")))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct PlcId([u8; Self::SIZE]);

impl PlcId {
    pub const SIZE: usize = 15;

    #[inline]
    pub const fn new(id: [u8; Self::SIZE]) -> Self {
        Self(id)
    }

    pub fn decode(input: impl AsRef<str>) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; Self::SIZE];
        decode_into(input, &mut buffer)?;

        Ok(Self(buffer))
    }

    pub fn encode(&self) -> String {
        encode(self.0)
    }
}

impl FromStr for PlcId {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode(s)
    }
}

impl fmt::Display for PlcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.encode();
        write!(f, "{id}")
    }
}

impl fmt::Debug for PlcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.encode();
        f.debug_tuple("PlcId").field(&id).finish()
    }
}

impl AsRef<[u8]> for PlcId {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

#[cfg(not(feature = "language"))]
pub type PlcId = string;

#[cfg(all(test, feature = "plc"))]
mod test {
    use super::{Did, PlcId};
    use crate::Handle;

    #[test]
    fn test_parse_did() {
        assert_eq!(
            Did::Plc(PlcId::new([
                0x25, 0xaa, 0x8f, 0xb6, 0xf9, 0xc3, 0xa8, 0xdf, 0x64, 0xf7, 0x89, 0xe5, 0xee, 0x39,
                0x19
            ])),
            "did:plc:ewvi7nxzyoun6zhxrhs64oiz".parse().unwrap()
        );

        assert_eq!(
            Did::Web(Handle::new("bsky.app")),
            "did:web:bsky.app".parse().unwrap()
        );
    }

    #[test]
    fn test_decode_plc_id() {
        let cases = [
            (
                "j67mwmangcbxch7knfm7jo2b",
                [
                    79, 190, 203, 48, 13, 48, 131, 113, 31, 234, 105, 89, 244, 187, 65,
                ],
            ),
            (
                "ragtjsm2j2vknwkz3zp4oxrd",
                [
                    136, 13, 52, 201, 154, 78, 170, 166, 217, 89, 222, 95, 199, 94, 35,
                ],
            ),
        ];

        for (value, expected) in cases {
            let id = PlcId::decode(value).expect(value);
            assert_eq!(id.as_ref(), &expected[..]);
        }
    }

    #[test]
    fn test_format_plc_id() {
        let id = PlcId::decode("j67mwmangcbxch7knfm7jo2b").expect("decode did:plc:_");
        assert_eq!(id.to_string(), "j67mwmangcbxch7knfm7jo2b");
        assert_eq!(format!("{id:?}"), "PlcId(\"j67mwmangcbxch7knfm7jo2b\")");
    }
}
