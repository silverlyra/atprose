pub use cid::Cid;

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(not(feature = "chrono"))]
pub type DateTime = String;

#[cfg(feature = "language")]
#[cfg_attr(docsrs, doc(cfg(feature = "language")))]
pub type Language = oxilangtag::LanguageTag<String>;

#[cfg(not(feature = "language"))]
pub type Language = String;

#[cfg(any(feature = "plc", feature = "rkey"))]
pub(crate) mod encoding;

mod identity;
pub use identity::{
    did::{Did, InvalidDid, PlcId},
    handle::{Handle, InvalidHandle},
    identifier::Identifier,
};

pub(crate) mod ns;
pub use ns::{InvalidNsid, Nsid, TypeId};

pub(crate) mod record;
#[cfg(feature = "rkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "rkey")))]
pub use record::key::tid::Tid;
pub use record::{
    key::Rkey,
    uri::{AtUri, AtUriResource, AtUriTarget, InvalidUri},
};
