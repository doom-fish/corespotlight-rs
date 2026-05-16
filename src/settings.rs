use core::fmt;

use serde::Deserialize;

use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::private::{
    error_from_status, impl_object_wrapper, json_cstring, optional_cstring_from_str,
    opt_cstring_ptr, parse_json_ptr, take_string,
};

impl_object_wrapper!(CSSearchQueryContext);
impl_object_wrapper!(CSUserQueryContext);
impl_object_wrapper!(CSSuggestion);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSIndexErrorCode {
    UnknownError = -1,
    IndexUnavailableError = -1000,
    InvalidItemError = -1001,
    InvalidClientStateError = -1002,
    RemoteConnectionError = -1003,
    QuotaExceeded = -1004,
    IndexingUnsupported = -1005,
    MismatchedClientState = -1006,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSSearchQueryErrorCode {
    Unknown = -2000,
    IndexUnreachable = -2001,
    InvalidQuery = -2002,
    Cancelled = -2003,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSSuggestionKind {
    None = 0,
    Custom = 1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum CSUserInteraction {
    Select = 0,
    Focus = 1,
}

impl CSUserInteraction {
    pub const DEFAULT: Self = Self::Select;

    pub const fn raw_value(self) -> i64 {
        match self {
            Self::Select => 0,
            Self::Focus => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSSearchQuerySourceOptions(u64);

impl CSSearchQuerySourceOptions {
    pub const DEFAULT: Self = Self(0);
    pub const ALLOW_MAIL: Self = Self(1 << 0);

    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSSearchableItemUpdateListenerOptions(u64);

impl CSSearchableItemUpdateListenerOptions {
    pub const DEFAULT: Self = Self(0);
    pub const SUMMARIZATION: Self = Self(1 << 1);
    pub const PRIORITY: Self = Self(1 << 2);

    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LocalizedSuggestionRange {
    pub start: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedSuggestion {
    pub string: String,
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

pub fn core_spotlight_version_number() -> f64 {
    unsafe { ffi::cs_core_spotlight_version_number() }
}

pub fn core_spotlight_version_string() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_core_spotlight_version_string, "CoreSpotlight version string")
}

pub fn core_spotlight_api_version() -> i32 {
    unsafe { ffi::cs_core_spotlight_api_version() }
}

pub fn index_error_domain() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_index_error_domain, "CoreSpotlight index error domain")
}

pub fn search_query_error_domain() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_search_query_error_domain,
        "CoreSpotlight search query error domain",
    )
}

pub fn searchable_item_action_type() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_searchable_item_action_type,
        "CSSearchableItemActionType",
    )
}

pub fn searchable_item_activity_identifier() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_searchable_item_activity_identifier,
        "CSSearchableItemActivityIdentifier",
    )
}

pub fn query_continuation_action_type() -> Result<String, CoreSpotlightError> {
    get_required_string(
        ffi::cs_query_continuation_action_type,
        "CSQueryContinuationActionType",
    )
}

pub fn search_query_string_key() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_search_query_string_key, "CSSearchQueryString")
}

pub fn mailbox_inbox() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_inbox, "CSMailboxInbox")
}

pub fn mailbox_drafts() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_drafts, "CSMailboxDrafts")
}

pub fn mailbox_sent() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_sent, "CSMailboxSent")
}

pub fn mailbox_junk() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_junk, "CSMailboxJunk")
}

pub fn mailbox_trash() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_trash, "CSMailboxTrash")
}

pub fn mailbox_archive() -> Result<String, CoreSpotlightError> {
    get_required_string(ffi::cs_mailbox_archive, "CSMailboxArchive")
}

impl CSSearchQueryContext {
    pub fn new() -> Result<Self, CoreSpotlightError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_search_query_context_new(&mut out_context, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_context, "search query context") }
    }

    pub fn fetch_attributes(&self) -> Result<Vec<String>, CoreSpotlightError> {
        get_string_array(
            self.as_ptr(),
            ffi::cs_search_query_context_get_fetch_attributes,
            "search query fetch attributes",
        )
    }

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

    pub fn filter_queries(&self) -> Result<Vec<String>, CoreSpotlightError> {
        get_string_array(
            self.as_ptr(),
            ffi::cs_search_query_context_get_filter_queries,
            "search query filter queries",
        )
    }

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

    pub fn keyboard_language(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_search_query_context_get_keyboard_language)
    }

    pub fn set_keyboard_language(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "search query keyboard language",
            ffi::cs_search_query_context_set_keyboard_language,
        )
    }

    pub fn source_options(&self) -> CSSearchQuerySourceOptions {
        CSSearchQuerySourceOptions::new(unsafe {
            ffi::cs_search_query_context_get_source_options(self.as_ptr())
        })
    }

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
    pub fn new() -> Result<Self, CoreSpotlightError> {
        let mut out_context = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_user_query_context_new(&mut out_context, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_context, "user query context") }
    }

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

    pub fn enable_ranked_results(&self) -> bool {
        unsafe { ffi::cs_user_query_context_get_enable_ranked_results(self.as_ptr()) != 0 }
    }

    pub fn set_enable_ranked_results(&self, enabled: bool) {
        unsafe {
            ffi::cs_user_query_context_set_enable_ranked_results(self.as_ptr(), i32::from(enabled));
        }
    }

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

    pub fn max_result_count(&self) -> isize {
        unsafe { ffi::cs_user_query_context_get_max_result_count(self.as_ptr()) as isize }
    }

    pub fn set_max_result_count(&self, value: isize) {
        unsafe { ffi::cs_user_query_context_set_max_result_count(self.as_ptr(), value as i64) };
    }

    pub fn max_suggestion_count(&self) -> isize {
        unsafe { ffi::cs_user_query_context_get_max_suggestion_count(self.as_ptr()) as isize }
    }

    pub fn set_max_suggestion_count(&self, value: isize) {
        unsafe { ffi::cs_user_query_context_set_max_suggestion_count(self.as_ptr(), value as i64) };
    }

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
    pub fn localized_attributed_suggestion(&self) -> Result<LocalizedSuggestion, CoreSpotlightError> {
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
        let payload: LocalizedSuggestionPayload = unsafe {
            parse_json_ptr(out_json, "localized attributed suggestion")?
        };
        Ok(LocalizedSuggestion {
            string: payload.string,
            highlighted_ranges: payload.highlighted_ranges,
        })
    }

    pub fn suggestion_kind(&self) -> CSSuggestionKind {
        CSSuggestionKind::from_raw(unsafe { ffi::cs_suggestion_get_kind(self.as_ptr()) })
    }

    pub fn compare(&self, other: &Self) -> core::cmp::Ordering {
        match unsafe { ffi::cs_suggestion_compare(self.as_ptr(), other.as_ptr()) } {
            value if value < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        }
    }

    pub fn compare_by_rank(&self, other: &Self) -> core::cmp::Ordering {
        match unsafe { ffi::cs_suggestion_compare_by_rank(self.as_ptr(), other.as_ptr()) } {
            value if value < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        }
    }
}
