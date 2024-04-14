use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{Nsid, TypeId};

/// The metadata defined for every Lexicon type.
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A [`ref`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#ref
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct Ref {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(rename = "ref")]
    pub target: RefTarget,
}

/// A [`union`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#union
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Union {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(rename = "refs")]
    pub options: Vec<RefTarget>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
}

/// A named type referenced by a [`Ref`] or [`Union`].
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct RefTarget {
    pub ns: Option<Nsid>,
    pub name: Option<String>,
}

impl RefTarget {
    pub fn new(ns: Option<Nsid>, name: impl Into<String>) -> Self {
        let name: String = name.into();

        Self {
            ns,
            name: (!name.is_empty()).then_some(name),
        }
    }

    pub fn ns(&self) -> Option<&Nsid> {
        self.ns.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn resolve(&self, base: &Nsid) -> TypeId {
        let nsid = self.ns().unwrap_or(base);

        TypeId::new(nsid.to_owned(), self.name.clone())
    }
}

impl FromStr for RefTarget {
    type Err = (); // TODO(lyra)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ns, name) = match s.split_once('#') {
            Some(("", "")) | Some(("", "main")) => (None, None),
            Some(("", name)) => (None, Some(name)),
            Some((ns, "main")) => (Some(ns.parse()?), None),
            Some((ns, name)) => (Some(ns.parse()?), Some(name)),
            None => (Some(s.parse()?), None),
        };

        Ok(Self {
            ns,
            name: name.map(ToOwned::to_owned),
        })
    }
}

impl Display for RefTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.ns.as_ref(), self.name.as_ref()) {
            (None, None) => write!(f, "#main"),
            (None, Some(name)) => write!(f, "#{name}"),
            (Some(ns), None) => write!(f, "{ns}"),
            (Some(ns), Some(name)) => write!(f, "{ns}#{name}"),
        }
    }
}

impl<'de> Deserialize<'de> for RefTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = std::string::String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("unknown record key"))
    }
}

impl Serialize for RefTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// A [`token`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#token
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Token {
    #[serde(flatten)]
    pub metadata: Metadata,
}

/// An [`unknown`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#unknown
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Unknown {
    #[serde(flatten)]
    pub metadata: Metadata,
}

#[cfg(test)]
mod test {
    use super::RefTarget;
    use crate::schema::Nsid;

    #[test]
    fn test_ref_target_from_str() {
        assert_eq!(
            "#viewRecord".parse::<RefTarget>().unwrap(),
            RefTarget::new(None, "viewRecord")
        );

        assert_eq!(
            "app.bsky.feed.defs#generatorView"
                .parse::<RefTarget>()
                .unwrap(),
            RefTarget::new(Some(Nsid::new("app.bsky.feed", "defs")), "generatorView")
        );
    }
}
