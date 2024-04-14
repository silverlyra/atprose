use serde::{Deserialize, Serialize};

use super::meta::Metadata;

/// A [`null`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#null
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Null {
    #[serde(flatten)]
    pub metadata: Metadata,
}

impl std::ops::Deref for Null {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

/// A [`boolean`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#boolean
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Boolean {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,

    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

impl std::ops::Deref for Boolean {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

/// An [`integer`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#integer
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Integer {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,

    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<i64>>,
}

impl std::ops::Deref for Integer {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

/// A [`string`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#string
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct String {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StringFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<std::string::String>,

    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub value: Option<std::string::String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_graphemes: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_graphemes: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_values: Option<Vec<std::string::String>>,

    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<std::string::String>>,
}

impl std::ops::Deref for String {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

/// A [`String`] [format][spec].
///
/// [spec]: https://atproto.com/specs/lexicon#string-formats
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum StringFormat {
    /// Either a [DID][] or a [handle][].
    ///
    /// [DID]: https://atproto.com/specs/did
    /// [Handle]: https://atproto.com/specs/handle
    AtIdentifier,

    /// An [`at://`][scheme] URI.
    ///
    /// [scheme]: https://atproto.com/specs/at-uri-scheme
    AtUri,

    /// A [date-time][] string.
    ///
    /// [date-time]: https://atproto.com/specs/lexicon#datetime
    Datetime,

    /// A [DID][].
    ///
    /// [DID]: https://atproto.com/specs/did
    Did,

    /// A [Handle][].
    ///
    /// [Handle]: https://atproto.com/specs/handle
    Handle,

    /// An [NSID][] (namespaced identifier).
    ///
    /// [NSID]: https://atproto.com/specs/nsid
    Nsid,

    /// An IETF [language tag].
    ///
    /// [language tag]: https://atproto.com/specs/lexicon#language
    Language,

    /// A generic URI.
    Uri,
}

/// A [`blob`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#blob
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<std::string::String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<usize>,
}

impl std::ops::Deref for Blob {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

/// A [`bytes`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#bytes
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bytes {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

impl std::ops::Deref for Bytes {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

/// A [`cid-link`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#cid-link
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Link {
    #[serde(flatten)]
    pub metadata: Metadata,
}

impl std::ops::Deref for Link {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}
