use std::{fmt::Display, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

/// A Lexicon [NSID][] (e.g., `app.bsky.feed.post`).
///
/// ```
/// use atprose_lexicon::schema::Nsid;
///
/// # fn main() -> Result<(), ()> {
/// let id: Nsid = "app.bsky.feed.post".parse()?;
/// assert_eq!(id.authority, "app.bsky.feed");
/// assert_eq!(id.package, "post");
/// # Ok(())
/// # }
/// ```
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
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

impl FromStr for Nsid {
    type Err = (); // TODO(lyra)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((authority, package)) = s.rsplit_once('.') {
            Ok(Self::new(authority, package))
        } else {
            Err(())
        }
    }
}

impl<'de> Deserialize<'de> for Nsid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = std::string::String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("unknown record key"))
    }
}

impl Display for Nsid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.authority, self.package)
    }
}

impl Serialize for Nsid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct TypeId {
    pub ns: Nsid,
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
    type Err = (); // TODO(lyra)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((nsid, name)) = s.split_once('#') {
            if let Ok(nsid) = nsid.parse() {
                return Ok(Self::new(nsid, Some(name.to_owned())));
            }
        }
        Err(())
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
