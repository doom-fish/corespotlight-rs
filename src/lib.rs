#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::redundant_pub_crate,
    clippy::ref_option,
    clippy::return_self_not_must_use,
    clippy::struct_field_names,
    clippy::type_complexity,
    clippy::use_self
)]

pub mod attribute_set;
pub mod error;
pub mod ffi;
pub mod index;
pub mod item;
mod private;

pub use attribute_set::CSSearchableItemAttributeSet;
pub use error::{CoreSpotlightError, CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN};
pub use index::CSSearchableIndex;
pub use item::CSSearchableItem;

/// Common imports.
pub mod prelude {
    pub use crate::attribute_set::CSSearchableItemAttributeSet;
    pub use crate::error::{CoreSpotlightError, CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN};
    pub use crate::index::CSSearchableIndex;
    pub use crate::item::CSSearchableItem;
}
