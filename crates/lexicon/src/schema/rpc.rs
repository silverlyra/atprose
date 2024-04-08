use serde::{Deserialize, Serialize};

use super::{
    concrete::{Boolean, Integer, String},
    container::Object,
    meta::{Metadata, Ref, Union, Unknown},
};
use crate::Map;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<QuerySchema>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Body>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Notice>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
    #[serde(flatten)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<QuerySchema>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Body>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Body>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Notice>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(flatten)]
    pub metadata: Metadata,

    pub encoding: std::string::String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<BodySchema>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum BodySchema {
    Ref(Ref),
    Union(Union),
    Object(Object),
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type")]
pub enum QuerySchema {
    #[serde(rename = "params")]
    Parameters(Parameters),
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(flatten)]
    pub metadata: Metadata,

    pub properties: Map<std::string::String, ParameterValue>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<std::string::String>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ParameterValue {
    Boolean(Boolean),
    Integer(Integer),
    String(String),
    Unknown(Unknown),
    Array(ParameterArray),
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParameterArray {
    #[serde(flatten)]
    pub metadata: Metadata,

    pub items: ParameterArrayItem,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ParameterArrayItem {
    Boolean(Boolean),
    Integer(Integer),
    String(String),
    Unknown(Unknown),
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
    pub name: std::string::String,

    #[serde(flatten)]
    pub metadata: Metadata,
}
