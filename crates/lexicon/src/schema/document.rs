use serde::{Deserialize, Serialize};

use super::{
    concrete::{Blob, Boolean, Bytes, Integer, Link, String},
    container::{Array, Object, Record},
    meta::{Metadata, Ref, Union, Unknown},
    rpc::{Procedure, Query},
};
use crate::Map;

/// A Lexicon [schema file][file].
///
/// [file]: https://atproto.com/specs/lexicon#lexicon-files
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Document {
    /// Lexicon language version. In this version, a fixed value of `1`
    #[serde(default, rename = "lexicon")]
    pub version: u32,

    pub id: std::string::String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u32>,

    #[serde(flatten)]
    pub metadata: Metadata,

    pub defs: Map<std::string::String, Definition>,
}

/// Lexicon language version used in a [`Document`].
#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version(pub u32);

impl Default for Version {
    fn default() -> Self {
        Self(1)
    }
}

/// A top-level definition in a Lexicon [`Document`].
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Definition {
    Record(Record),
    Query(Query),
    Procedure(Procedure),

    Array(Array),
    Object(Object),

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
