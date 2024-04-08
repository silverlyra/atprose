use serde::{Deserialize, Serialize};

/// The metadata defined for every Lexicon type.
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A [`ref`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#ref
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
pub struct Ref {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(rename = "ref")]
    pub target: String,
}

/// A [`union`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#union
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
pub struct Union {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(rename = "refs")]
    pub options: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
}

/// A [`token`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#token
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
pub struct Token {
    #[serde(flatten)]
    pub metadata: Metadata,
}

/// An [`unknown`][spec] type.
///
/// [spec]: https://atproto.com/specs/lexicon#unknown
#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
pub struct Unknown {
    #[serde(flatten)]
    pub metadata: Metadata,
}
