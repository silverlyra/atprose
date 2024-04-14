#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::broken_intra_doc_links))]

pub mod schema;

pub use indexmap::{IndexMap as Map, IndexSet as Set};

#[cfg(feature = "load")]
mod load;

#[cfg(feature = "load")]
#[cfg_attr(docsrs, doc(cfg(feature = "load")))]
pub use load::{load, load_document};
