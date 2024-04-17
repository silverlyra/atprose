use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::key::Rkey;
use crate::identity::identifier::{Identifier, InvalidIdentifier};
use crate::ns::{InvalidNsid, Nsid};

/// A parsed [`at://` URI][uri].
///
/// [uri]: https://atproto.com/specs/at-uri-scheme
///
/// ```
/// use atprose_types::AtUri;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # Ok(())
/// # }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct AtUri {
    authority: Identifier,
    resource: Option<AtUriResource>,
}

impl AtUri {
    pub fn new(target: AtUriTarget) -> Self {
        match target {
            AtUriTarget::Repository(authority) => Self {
                authority,
                resource: None,
            },
            AtUriTarget::Collection(authority, collection) => Self {
                authority,
                resource: Some(AtUriResource {
                    collection,
                    record: None,
                }),
            },
            AtUriTarget::Record(authority, collection, record) => Self {
                authority,
                resource: Some(AtUriResource {
                    collection,
                    record: Some(record),
                }),
            },
        }
    }

    pub fn authority(&self) -> &Identifier {
        &self.authority
    }

    pub fn resource(&self) -> Option<&AtUriResource> {
        self.resource.as_ref()
    }

    pub fn collection(&self) -> Option<&Nsid> {
        self.resource().map(|resource| &resource.collection)
    }

    pub fn record(&self) -> Option<&Rkey> {
        self.resource()
            .and_then(|resource| resource.record.as_ref())
    }

    pub fn target(&self) -> AtUriTarget {
        let authority = self.authority.clone();

        if let Some(collection) = self.collection().cloned() {
            if let Some(record) = self.record().cloned() {
                AtUriTarget::Record(authority, collection, record)
            } else {
                AtUriTarget::Collection(authority, collection)
            }
        } else {
            AtUriTarget::Repository(authority)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct AtUriResource {
    pub collection: Nsid,
    pub record: Option<Rkey>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum AtUriTarget {
    Repository(Identifier),
    Collection(Identifier, Nsid),
    Record(Identifier, Nsid, Rkey),
}

impl FromStr for AtUri {
    type Err = InvalidUri;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(uri) = s.strip_prefix("at://") else {
            return Err(InvalidUri::Scheme);
        };

        let mut format = UriFormat::default();
        for (i, s) in uri.match_indices(&['@', '/', '?', '#']) {
            let c = s.chars().next().expect("empty match");
            format = format.consume(uri, (i, c))?;
        }

        let target = format.target(uri)?;

        Ok(Self::new(target))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for AtUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for AtUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let authority = self.authority();

        if let Some(collection) = self.collection() {
            if let Some(record) = self.record() {
                write!(f, "at://{authority}/{collection}/{record}")
            } else {
                write!(f, "at://{authority}/{collection}")
            }
        } else {
            write!(f, "at://{authority}")
        }
    }
}

#[derive(thiserror::Error, PartialEq, Debug, Clone)]
pub enum InvalidUri {
    #[error("invalid at:// URI scheme")]
    Scheme,
    #[error("unrecognized at:// URI path")]
    Path,
    #[error(transparent)]
    Authority(#[from] InvalidIdentifier),
    #[error(transparent)]
    Collection(#[from] InvalidNsid),
    #[error("unexpected ?query in at:// URI")]
    Query,
    #[error("unexpected #fragment in at:// URI")]
    Fragment,
    #[error("unexpected credentials@ in at:// URI")]
    Credentials,
}

#[derive(Debug, Copy, Clone, Default)]
enum UriFormat {
    #[default]
    Repository,
    Collection(usize),
    Record(usize, usize),
}

impl UriFormat {
    pub fn consume(self, input: &str, (index, token): (usize, char)) -> Result<Self, InvalidUri> {
        use UriFormat::{Collection, Record, Repository};

        let end = input.len() - 1;

        match (self, token, index) {
            (_, _, 0) => Err(InvalidUri::Authority(InvalidIdentifier::empty())),
            (state, '/', i) if i == end => Ok(state),
            (Repository, '/', i) => Ok(Collection(i)),
            (Collection(i), '/', j) => Ok(Record(i, j)),
            (_, '?', _) => Err(InvalidUri::Query),
            (_, '#', _) => Err(InvalidUri::Scheme),
            (_, '@', _) => Err(InvalidUri::Credentials),
            _ => Err(InvalidUri::Path),
        }
    }

    pub fn target(self, input: &str) -> Result<AtUriTarget, InvalidUri> {
        let (authority, collection, record) = match self {
            UriFormat::Repository => (input, None, None),
            UriFormat::Collection(i) => (&input[..i], Some(&input[i + 1..]), None),
            UriFormat::Record(i, j) => (&input[..i], Some(&input[i + 1..j]), Some(&input[j + 1..])),
        };

        let authority: Identifier = authority.parse().map_err(InvalidUri::from)?;

        Ok(if let Some(collection) = collection {
            let collection: Nsid = collection.parse().map_err(InvalidUri::from)?;

            if let Some(record) = record {
                #[cfg(feature = "rkey")]
                let record: Rkey = record.parse().map_err(|_| InvalidUri::Path)?;
                #[cfg(not(feature = "rkey"))]
                let record = record.to_owned();

                AtUriTarget::Record(authority, collection, record)
            } else {
                AtUriTarget::Collection(authority, collection)
            }
        } else {
            AtUriTarget::Repository(authority)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{AtUri, Handle, Identifier, InvalidUri, Nsid, Rkey};

    fn parse(value: &str) -> AtUri {
        match value.parse() {
            Ok(uri) => uri,
            Err(err) => panic!("could not parse {value}: {err}"),
        }
    }

    fn fail(value: &str, reason: InvalidUri) {
        let result: Result<AtUri, _> = value.parse();
        match result {
            Ok(_) => panic!("{value} should be an invalid URI"),
            Err(err) => assert_eq!(reason, err, "incorrect error for {value}"),
        }
    }

    fn did(value: &str) -> Identifier {
        Identifier::Did(value.parse().expect("parse DID"))
    }

    fn handle(value: &str) -> Identifier {
        Identifier::Handle(Handle::new(value))
    }

    fn nsid(value: &str) -> Nsid {
        value.parse().expect("parse NSID")
    }

    #[test]
    fn test_parse_uri() {
        let uri = parse("at://atproto.com");
        assert_eq!(handle("atproto.com"), uri.authority);
        assert!(uri.collection().is_none());
        assert!(uri.record().is_none());

        let uri: AtUri = parse("at://foo.com/com.example.foo/123");
        assert_eq!(handle("foo.com"), uri.authority);
        assert_eq!(Some(nsid("com.example.foo")), uri.collection().cloned());
        assert_eq!(Some(Rkey::Custom("123".to_owned())), uri.record().cloned());

        fail("https://bsky.app", InvalidUri::Scheme);
    }
}
