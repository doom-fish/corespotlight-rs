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
pub mod default_index_extension_request_handler;
pub mod delegate;
pub mod error;
pub mod ffi;
pub mod index;
pub mod index_extension_request_handler;
pub mod item;
pub mod query;
pub mod settings;
mod attribute_set_fields;
mod private;

pub use attribute_set::{
    CSCustomAttributeKey, CSLocalizedString, CSPerson, CSPersonData,
    CSSearchableItemAttributeDataField, CSSearchableItemAttributeDateArrayField,
    CSSearchableItemAttributeDateField, CSSearchableItemAttributeNumberField,
    CSSearchableItemAttributePersonArrayField, CSSearchableItemAttributeReadOnlyNumberField,
    CSSearchableItemAttributeReadOnlyStringField, CSSearchableItemAttributeSet,
    CSSearchableItemAttributeStringArrayField, CSSearchableItemAttributeStringArrayMapField,
    CSSearchableItemAttributeStringField, CSSearchableItemAttributeURLField, CustomAttributeValue,
};
pub use default_index_extension_request_handler::DefaultIndexExtensionRequestHandler;
pub use delegate::{CSSearchableIndexDelegate, CSSearchableIndexDelegateCallbacks};
pub use error::{CoreSpotlightError, CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN};
pub use index::CSSearchableIndex;
pub use index_extension_request_handler::CSIndexExtensionRequestHandler;
pub use item::CSSearchableItem;
pub use query::{
    CSSearchQuery, CSSearchQueryExecutionResult, CSUserQuery, CSUserQueryExecutionResult,
};
pub use settings::{
    core_spotlight_api_version, core_spotlight_version_number, core_spotlight_version_string,
    index_error_domain, mailbox_archive, mailbox_drafts, mailbox_inbox, mailbox_junk,
    mailbox_sent, mailbox_trash, query_continuation_action_type, search_query_error_domain,
    search_query_string_key, searchable_item_action_type,
    searchable_item_activity_identifier, CSIndexErrorCode, CSSearchQueryContext,
    CSSearchQueryErrorCode, CSSearchQuerySourceOptions, CSSearchableItemUpdateListenerOptions,
    CSSuggestion, CSSuggestionKind, CSUserInteraction, CSUserQueryContext,
    LocalizedSuggestion, LocalizedSuggestionRange,
};

/// Common imports.
pub mod prelude {
    pub use crate::attribute_set::{
        CSCustomAttributeKey, CSLocalizedString, CSPerson, CSPersonData,
        CSSearchableItemAttributeDataField, CSSearchableItemAttributeDateArrayField,
        CSSearchableItemAttributeDateField, CSSearchableItemAttributeNumberField,
        CSSearchableItemAttributePersonArrayField, CSSearchableItemAttributeReadOnlyNumberField,
        CSSearchableItemAttributeReadOnlyStringField, CSSearchableItemAttributeSet,
        CSSearchableItemAttributeStringArrayField, CSSearchableItemAttributeStringArrayMapField,
        CSSearchableItemAttributeStringField, CSSearchableItemAttributeURLField, CustomAttributeValue,
    };
    pub use crate::default_index_extension_request_handler::DefaultIndexExtensionRequestHandler;
    pub use crate::delegate::{CSSearchableIndexDelegate, CSSearchableIndexDelegateCallbacks};
    pub use crate::error::{CoreSpotlightError, CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN};
    pub use crate::index::CSSearchableIndex;
    pub use crate::index_extension_request_handler::CSIndexExtensionRequestHandler;
    pub use crate::item::CSSearchableItem;
    pub use crate::query::{
        CSSearchQuery, CSSearchQueryExecutionResult, CSUserQuery, CSUserQueryExecutionResult,
    };
    pub use crate::settings::{
        core_spotlight_api_version, core_spotlight_version_number, core_spotlight_version_string,
        index_error_domain, mailbox_archive, mailbox_drafts, mailbox_inbox, mailbox_junk,
        mailbox_sent, mailbox_trash, query_continuation_action_type, search_query_error_domain,
        search_query_string_key, searchable_item_action_type,
        searchable_item_activity_identifier, CSIndexErrorCode, CSSearchQueryContext,
        CSSearchQueryErrorCode, CSSearchQuerySourceOptions,
        CSSearchableItemUpdateListenerOptions, CSSuggestion, CSSuggestionKind,
        CSUserInteraction, CSUserQueryContext, LocalizedSuggestion, LocalizedSuggestionRange,
    };
}
