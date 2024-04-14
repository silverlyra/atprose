use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{
    concrete::{Blob, Boolean, Bytes, Integer, Link, String},
    meta::{Metadata, Ref, Union, Unknown},
};
use crate::Map;

/// A [`record`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#record
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(flatten)]
    pub metadata: Metadata,

    pub key: RecordKey,

    #[serde(rename = "record")]
    pub def: RecordDefinition,
}

impl Record {
    pub fn new(key: RecordKey, def: Object) -> Self {
        Self {
            metadata: Default::default(),
            key,
            def: RecordDefinition::Object(def),
        }
    }
}

/// The [key format][rkey] of a [`Record`].
///
/// [rkey]: https://atproto.com/specs/record-key
#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub enum RecordKey {
    /// A [timestamp identifier][tid].
    ///
    /// [tid]: https://atproto.com/specs/record-key#record-key-type-tid
    #[default]
    Tid,

    /// Each "collection" of this record type will have a single record with a
    /// [well-known fixed key][literal].
    ///
    /// [literal]: https://atproto.com/specs/record-key#record-key-type-literalvalue
    Literal(std::string::String),

    /// [Any string][any] meeting the schema requirements.
    ///
    /// [any]: https://atproto.com/specs/record-key#record-key-type-any
    Any,
}

impl FromStr for RecordKey {
    type Err = (); // TODO(lyra)

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(literal) = s.strip_prefix("literal:") {
            Ok(Self::Literal(literal.to_owned()))
        } else {
            match s {
                "any" => Ok(Self::Any),
                "tid" => Ok(Self::Tid),
                _ => Err(()),
            }
        }
    }
}

impl<'de> Deserialize<'de> for RecordKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = std::string::String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("unknown record key"))
    }
}

impl Display for RecordKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordKey::Any => write!(f, "any"),
            RecordKey::Literal(literal) => write!(f, "literal:{literal}"),
            RecordKey::Tid => write!(f, "tid"),
        }
    }
}

impl Serialize for RecordKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum RecordDefinition {
    Object(Object),
}

/// An [`array`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#array
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[serde(flatten)]
    pub metadata: Metadata,

    pub items: ArrayItem,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

/// The type of an [array][Array]'s items.
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ArrayItem {
    Blob(Blob),
    Boolean(Boolean),
    Bytes(Bytes),
    Integer(Integer),
    Link(Link),
    String(String),
    Unknown(Unknown),

    Ref(Ref),
    Union(Union),
}

/// An [`object`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#object
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(flatten)]
    pub metadata: Metadata,

    pub properties: Map<std::string::String, Property>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<std::string::String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nullable: Vec<std::string::String>,
}

/// A property of an [object][Object].
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Property {
    Blob(Blob),
    Boolean(Boolean),
    Bytes(Bytes),
    Integer(Integer),
    Link(Link),
    String(String),
    Unknown(Unknown),

    Array(Array),

    Ref(Ref),
    Union(Union),
}
