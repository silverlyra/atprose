use std::{fmt, ops::Deref, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Lexicon [NSID][] (e.g., `app.bsky.feed.post`).
///
/// ```
/// use atprose_types::Nsid;
///
/// # fn main() -> Result<(), ()> {
/// let id: Nsid = "app.bsky.feed.post".parse()?;
/// assert_eq!(id.authority, "app.bsky.feed");
/// assert_eq!(id.package, "post");
/// # Ok(())
/// # }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Nsid {
    pub authority: String,
    pub package: String,
}

impl Nsid {
    pub fn new(authority: impl Into<String>, package: impl Into<String>) -> Self {
        Self {
            authority: authority.into(),
            package: package.into(),
        }
    }
}

impl<S: Into<String>> From<(S, S)> for Nsid {
    fn from(value: (S, S)) -> Self {
        let (authority, package) = value;

        Nsid::new(authority, package)
    }
}

impl FromStr for Nsid {
    type Err = InvalidNsid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((authority, package)) = s.rsplit_once('.') {
            Ok(Self::new(authority, package))
        } else {
            Err(InvalidNsid::Authority)
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Nsid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("unknown record key"))
    }
}

impl fmt::Display for Nsid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.authority, self.package)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Nsid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum InvalidNsid {
    #[error("invalid nsid authority")]
    Authority,
}

/// A lexicon type, identified by its [namespace][Nsid] and local name.
///
/// ```
/// use atprose_types::TypeId;
///
/// # fn main() -> Result<(), ()> {
/// let id: TypeId = "app.bsky.feed.post".parse()?;
/// assert_eq!(id.authority, "app.bsky.feed");
/// assert_eq!(id.package, "post");
/// assert_eq!(id.name, None);
///
/// let id: TypeId = "app.bsky.feed.defs#postView".parse()?;
/// assert_eq!(id.authority, "app.bsky.feed");
/// assert_eq!(id.package, "defs");
/// assert_eq!(id.name, Some("postView".to_owned()));
/// # Ok(())
/// # }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct TypeId {
    pub ns: Nsid,
    /// The local name, or `None` if the name was `"main"`.
    pub name: Option<String>,
}

impl TypeId {
    pub fn new(ns: Nsid, name: impl Into<Option<String>>) -> Self {
        Self {
            ns,
            name: name.into(),
        }
    }

    pub fn of(ns: &Nsid, name: &str) -> Self {
        match name {
            "main" => Self::new(ns.to_owned(), None),
            name => Self::new(ns.to_owned(), Some(name.to_owned())),
        }
    }

    pub fn resolve(target: &str, base: &Nsid) -> Result<Self, ()> {
        let Some((nsid, name)) = target.split_once('#') else {
            return Err(());
        };

        if nsid.is_empty() {
            Ok(Self::of(base, name))
        } else {
            let Ok(nsid) = nsid.parse() else {
                return Err(());
            };

            Ok(Self::of(&nsid, name))
        }
    }
}

impl FromStr for TypeId {
    type Err = InvalidNsid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ns, name) = match s.rsplit_once('#') {
            Some((ns, "main")) => (ns, None),
            Some((ns, name)) => (ns, Some(name.to_owned())),
            None => (s, None),
        };

        let nsid = ns.parse()?;
        Ok(Self::new(nsid, name))
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.ns, self.name.as_ref()) {
            (ns, None) => write!(f, "{ns}"),
            (ns, Some(name)) => write!(f, "{ns}:{name}"),
        }
    }
}

impl Deref for TypeId {
    type Target = Nsid;

    fn deref(&self) -> &Self::Target {
        &self.ns
    }
}
