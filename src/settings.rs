//! Wrappers for CoreSpotlight settings, option sets, and exported constants.

use core::fmt;

use serde::Deserialize;

use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::private::{
    error_from_status, impl_object_wrapper, json_cstring, opt_cstring_ptr,
    optional_cstring_from_str, parse_json_ptr, take_string,
};

impl_object_wrapper!(CSSearchQueryContext);
impl_object_wrapper!(CSUserQueryContext);
impl_object_wrapper!(CSSuggestion);

/// Error codes mirrored from `CSIndexErrorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSIndexErrorCode {
    /// Wraps `CSIndexErrorCode.unknownError`.
    UnknownError = -1,
    /// Wraps `CSIndexErrorCode.indexUnavailableError`.
    IndexUnavailableError = -1000,
    /// Wraps `CSIndexErrorCode.invalidItemError`.
    InvalidItemError = -1001,
    /// Wraps `CSIndexErrorCode.invalidClientStateError`.
    InvalidClientStateError = -1002,
    /// Wraps `CSIndexErrorCode.remoteConnectionError`.
    RemoteConnectionError = -1003,
    /// Wraps `CSIndexErrorCode.quotaExceeded`.
    QuotaExceeded = -1004,
    /// Wraps `CSIndexErrorCode.indexingUnsupported`.
    IndexingUnsupported = -1005,
    /// Wraps `CSIndexErrorCode.mismatchedClientState`.
    MismatchedClientState = -1006,
}

/// Error codes mirrored from `CSSearchQueryErrorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSSearchQueryErrorCode {
    /// Wraps `CSSearchQueryErrorCode.unknown`.
    Unknown = -2000,
    /// Wraps `CSSearchQueryErrorCode.indexUnreachable`.
    IndexUnreachable = -2001,
    /// Wraps `CSSearchQueryErrorCode.invalidQuery`.
    InvalidQuery = -2002,
    /// Wraps `CSSearchQueryErrorCode.cancelled`.
    Cancelled = -2003,
}

/// Suggestion kinds mirrored from `CSSuggestionKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSSuggestionKind {
    /// Wraps `CSSuggestionKind.none`.
    None = 0,
    /// Wraps `CSSuggestionKind.custom`.
    Custom = 1,
    /// Wraps `CSSuggestionKind.default`.
    Default = 2,
}

impl CSSuggestionKind {
    const fn from_raw(value: i64) -> Self {
        match value {
            1 => Self::Custom,
            2 => Self::Default,
            _ => Self::None,
        }
    }
}

/// Interaction kinds mirrored from `CSUserQuery.UserInteraction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSUserInteraction {
    /// Wraps selection interactions.
    Select = 0,
    /// Wraps focus interactions.
    Focus = 1,
}

impl CSUserInteraction {
    /// Wraps the default `CSUserInteraction` value used by CoreSpotlight.
    pub const DEFAULT: Self = Self::Select;

    /// Returns the raw `CSUserInteraction` value used by CoreSpotlight.
    pub const fn raw_value(self) -> i64 {
        match self {
            Self::Select => 0,
            Self::Focus => 1,
        }
    }
}

/// Option-set wrapper for `CSSearchQueryContext.SourceOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSSearchQuerySourceOptions(u64);

impl CSSearchQuerySourceOptions {
    /// Wraps the default `CSSearchQuerySourceOptions` value used by CoreSpotlight.
    pub const DEFAULT: Self = Self(0);
    /// Wraps the `CSSearchQuerySourceOptions` associated constant `ALLOW_MAIL`.
    pub const ALLOW_MAIL: Self = Self(1 << 0);

    /// Wraps the `CSSearchQuerySourceOptions` initializer.
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the raw `CSSearchQuerySourceOptions` value used by CoreSpotlight.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Reports whether this `CSSearchQuerySourceOptions` value contains another option.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl core::ops::BitOr for CSSearchQuerySourceOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CSSearchQuerySourceOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Option-set wrapper for `CSSearchableItemUpdateListenerOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSSearchableItemUpdateListenerOptions(u64);

impl CSSearchableItemUpdateListenerOptions {
    /// Wraps the default `CSSearchableItemUpdateListenerOptions` value used by CoreSpotlight.
    pub const DEFAULT: Self = Self(0);
    /// Wraps the `CSSearchableItemUpdateListenerOptions` associated constant `SUMMARIZATION`.
    pub const SUMMARIZATION: Self = Self(1 << 1);
    /// Wraps the `CSSearchableItemUpdateListenerOptions` associated constant `PRIORITY`.
    pub const PRIORITY: Self = Self(1 << 2);

    /// Wraps the `CSSearchableItemUpdateListenerOptions` initializer.
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the raw `CSSearchableItemUpdateListenerOptions` value used by CoreSpotlight.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Reports whether this `CSSearchableItemUpdateListenerOptions` value contains another option.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl core::ops::BitOr for CSSearchableItemUpdateListenerOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CSSearchableItemUpdateListenerOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalizedSuggestionPayload {
    string: String,
    highlighted_ranges: Vec<LocalizedSuggestionRange>,
}

/// Highlight range returned by `CSSuggestion.localizedAttributedSuggestion`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LocalizedSuggestionRange {
    /// Start offset within the localized suggestion string.
    pub start: usize,
    /// Length of the highlighted range.
    pub length: usize,
}

/// Localized suggestion payload returned by `CSSuggestion`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedSuggestion {
    /// Localized suggestion string.
    pub string: String,
    /// Highlighted ranges within `string`.
    pub highlighted_ranges: Vec<LocalizedSuggestionRange>,
}

impl fmt::Display for LocalizedSuggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.string)
    }
}

fn set_string_array(
    object_ptr: *mut core::ffi::c_void,
    values: &[String],
    context: &str,
    setter: unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_char,
        *mut *mut core::ffi::c_char,
    ) -> i32,
) -> Result<(), CoreSpotlightError> {
    let values_json = json_cstring(values, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe { setter(object_ptr, values_json.as_ptr(), &mut out_error) };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn get_string_array(
    object_ptr: *mut core::ffi::c_void,
    getter: unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_char,
        *mut *mut core::ffi::c_char,
    ) -> i32,
    context: &str,
) -> Result<Vec<String>, CoreSpotlightError> {
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe { getter(object_ptr, &mut out_json, &mut out_error) };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}

fn set_optional_string(
    object_ptr: *mut core::ffi::c_void,
    value: Option<&str>,
    context: &str,
    setter: unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_char,
        *mut *mut core::ffi::c_char,
    ) -> i32,
) -> Result<(), CoreSpotlightError> {
    let value = optional_cstring_from_str(value, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe { setter(object_ptr, opt_cstring_ptr(value.as_ref()), &mut out_error) };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn get_optional_string(
    object_ptr: *mut core::ffi::c_void,
    getter: unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_char,
) -> Option<String> {
    unsafe { take_string(getter(object_ptr)) }
}

fn get_required_string(
    getter: unsafe extern "C" fn() -> *mut core::ffi::c_char,
    label: &str,
) -> Result<String, CoreSpotlightError> {
    unsafe { take_string(getter()) }
        .ok_or_else(|| CoreSpotlightError::bridge(-2, format!("missing {label}")))
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn core_spotlight_version_number() -> f64 {
    unsafe { ffi::cs_core_spotlight_version_number() }
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn core_spotlight_version_string() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_core_spotlight_version_string,
        "CoreSpotlight version string",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn core_spotlight_api_version() -> i32 {
    unsafe { ffi::cs_core_spotlight_api_version() }
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn index_error_domain() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_index_error_domain,
        "CoreSpotlight index error domain",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn search_query_error_domain() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_search_query_error_domain,
        "CoreSpotlight search query error domain",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn searchable_item_action_type() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_searchable_item_action_type,
        "CSSearchableItemActionType",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn searchable_item_activity_identifier() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_searchable_item_activity_identifier,
        "CSSearchableItemActivityIdentifier",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn query_continuation_action_type() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_query_continuation_action_type,
        "CSQueryContinuationActionType",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn search_query_string_key() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_search_query_string_key, "CSSearchQueryString")
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn suggestion_highlight_attribute_name() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_suggestion_highlight_attribute_name,
        "CSSuggestionHighlightAttributeName",
    )
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn mailbox_inbox() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_inbox, "CSMailboxInbox")
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn mailbox_drafts() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_drafts, "CSMailboxDrafts")
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn mailbox_sent() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_sent, "CSMailboxSent")
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn mailbox_junk() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_junk, "CSMailboxJunk")
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn mailbox_trash() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_trash, "CSMailboxTrash")
}

/// Returns the corresponding CoreSpotlight constant or metadata value.
pub fn mailbox_archive() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_archive, "CSMailboxArchive")
}

impl CSSearchQueryContext {
    /// Wraps the `CSSearchQueryContext` initializer.
    pub fn new() -> Result<Self, CoreSpotlightError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_search_query_context_new(&mut out_context, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_context, "search query context") }
    }

    /// Wraps the corresponding `CSSearchQueryContext` getter.
    pub fn fetch_attributes(&self) -> Result<Vec<String>, CoreSpotlightError> {
        get_string_array(
            self.as_ptr(),
            ffi::cs_search_query_context_get_fetch_attributes,
            "search query fetch attributes",
        )
    }

    /// Wraps the corresponding `CSSearchQueryContext` setter.
    pub fn set_fetch_attributes<I, S>(&self, values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values = values.into_iter().map(Into::into).collect::<Vec<_>>();
        set_string_array(
            self.as_ptr(),
            &values,
            "search query fetch attributes",
            ffi::cs_search_query_context_set_fetch_attributes,
        )
    }

    /// Wraps the corresponding `CSSearchQueryContext` getter.
    pub fn filter_queries(&self) -> Result<Vec<String>, CoreSpotlightError> {
        get_string_array(
            self.as_ptr(),
            ffi::cs_search_query_context_get_filter_queries,
            "search query filter queries",
        )
    }

    /// Wraps the corresponding `CSSearchQueryContext` setter.
    pub fn set_filter_queries<I, S>(&self, values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values = values.into_iter().map(Into::into).collect::<Vec<_>>();
        set_string_array(
            self.as_ptr(),
            &values,
            "search query filter queries",
            ffi::cs_search_query_context_set_filter_queries,
        )
    }

    /// Wraps the corresponding `CSSearchQueryContext` getter.
    pub fn keyboard_language(&self) -> Option<String> {
        get_optional_string(
            self.as_ptr(),
            ffi::cs_search_query_context_get_keyboard_language,
        )
    }

    /// Wraps the corresponding `CSSearchQueryContext` setter.
    pub fn set_keyboard_language(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "search query keyboard language",
            ffi::cs_search_query_context_set_keyboard_language,
        )
    }

    /// Wraps the corresponding `CSSearchQueryContext` getter.
    pub fn source_options(&self) -> CSSearchQuerySourceOptions {
        CSSearchQuerySourceOptions::new(unsafe {
            ffi::cs_search_query_context_get_source_options(self.as_ptr())
        })
    }

    /// Wraps the corresponding `CSSearchQueryContext` setter.
    pub fn set_source_options(
        &self,
        source_options: CSSearchQuerySourceOptions,
    ) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_search_query_context_set_source_options(
                self.as_ptr(),
                source_options.bits(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl CSUserQueryContext {
    /// Wraps the `CSUserQueryContext` initializer.
    pub fn new() -> Result<Self, CoreSpotlightError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_user_query_context_new(&mut out_context, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_context, "user query context") }
    }

    /// Wraps a convenience initializer for `CSUserQueryContext`.
    pub fn with_current_suggestion(
        suggestion: Option<&CSSuggestion>,
    ) -> Result<Self, CoreSpotlightError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_context_with_current_suggestion(
                suggestion.map_or(core::ptr::null_mut(), CSSuggestion::as_ptr),
                &mut out_context,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_context, "user query context") }
    }

    /// Wraps the corresponding `CSUserQueryContext` method.
    pub fn enable_ranked_results(&self) -> bool {
        unsafe { ffi::cs_user_query_context_get_enable_ranked_results(self.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `CSUserQueryContext` setter.
    pub fn set_enable_ranked_results(&self, enabled: bool) {
        unsafe {
            ffi::cs_user_query_context_set_enable_ranked_results(self.as_ptr(), i32::from(enabled));
        }
    }

    /// Wraps the corresponding `CSUserQueryContext` getter.
    pub fn disable_semantic_search(&self) -> Result<bool, CoreSpotlightError> {
        let mut out_value = 0;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_context_get_disable_semantic_search(
                self.as_ptr(),
                &mut out_value,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(out_value != 0)
    }

    /// Wraps the corresponding `CSUserQueryContext` setter.
    pub fn set_disable_semantic_search(&self, disabled: bool) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_context_set_disable_semantic_search(
                self.as_ptr(),
                i32::from(disabled),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSUserQueryContext` getter.
    pub fn max_result_count(&self) -> isize {
        unsafe { ffi::cs_user_query_context_get_max_result_count(self.as_ptr()) as isize }
    }

    /// Wraps the corresponding `CSUserQueryContext` setter.
    pub fn set_max_result_count(&self, value: isize) {
        unsafe { ffi::cs_user_query_context_set_max_result_count(self.as_ptr(), value as i64) };
    }

    /// Wraps the corresponding `CSUserQueryContext` getter.
    pub fn max_suggestion_count(&self) -> isize {
        unsafe { ffi::cs_user_query_context_get_max_suggestion_count(self.as_ptr()) as isize }
    }

    /// Wraps the corresponding `CSUserQueryContext` setter.
    pub fn set_max_suggestion_count(&self, value: isize) {
        unsafe { ffi::cs_user_query_context_set_max_suggestion_count(self.as_ptr(), value as i64) };
    }

    /// Wraps the corresponding `CSUserQueryContext` getter.
    pub fn max_ranked_result_count(&self) -> Result<isize, CoreSpotlightError> {
        let mut out_value = 0_i64;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_context_get_max_ranked_result_count(
                self.as_ptr(),
                &mut out_value,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(out_value as isize)
    }

    /// Wraps the corresponding `CSUserQueryContext` setter.
    pub fn set_max_ranked_result_count(&self, value: isize) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_context_set_max_ranked_result_count(
                self.as_ptr(),
                value as i64,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}

impl CSSuggestion {
    /// Wraps the corresponding `CSSuggestion` getter.
    pub fn localized_attributed_suggestion(
        &self,
    ) -> Result<LocalizedSuggestion, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_suggestion_get_localized_attributed_suggestion(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: LocalizedSuggestionPayload =
            unsafe { parse_json_ptr(out_json, "localized attributed suggestion")? };
        Ok(LocalizedSuggestion {
            string: payload.string,
            highlighted_ranges: payload.highlighted_ranges,
        })
    }

    /// Wraps the corresponding `CSSuggestion` getter.
    pub fn suggestion_kind(&self) -> CSSuggestionKind {
        CSSuggestionKind::from_raw(unsafe { ffi::cs_suggestion_get_kind(self.as_ptr()) })
    }

    /// Wraps the corresponding `CSSuggestion` comparison API.
    pub fn compare(&self, other: &Self) -> core::cmp::Ordering {
        match unsafe { ffi::cs_suggestion_compare(self.as_ptr(), other.as_ptr()) } {
            value if value < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        }
    }

    /// Wraps the corresponding `CSSuggestion` comparison API.
    pub fn compare_by_rank(&self, other: &Self) -> core::cmp::Ordering {
        match unsafe { ffi::cs_suggestion_compare_by_rank(self.as_ptr(), other.as_ptr()) } {
            value if value < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        }
    }
}
