mod concrete;
mod container;
mod document;
mod meta;
mod rpc;

pub use self::{
    concrete::{Blob, Boolean, Bytes, Integer, Link, String, StringFormat},
    container::{Array, ArrayItem, Object, Record, RecordDefinition, RecordKey},
    document::{Definition, Document, Version},
    meta::{Metadata, Ref, Token, Union, Unknown},
};
pub use atprose_types::{Nsid, TypeId};

use crate::Map;

pub type Schema = Map<Nsid, Document>;
