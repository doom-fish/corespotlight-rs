//! Wrappers for `CSSearchQuery`, `CSUserQuery`, and related query types.

use std::time::Duration;

use serde::Deserialize;

use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::item::CSSearchableItem;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, json_cstring, opt_cstring_ptr,
    optional_cstring_from_str, parse_json_ptr,
};
use crate::settings::{CSSearchQueryContext, CSSuggestion, CSUserInteraction, CSUserQueryContext};

impl_object_wrapper!(CSSearchQuery);
impl_object_wrapper!(CSUserQuery);

/// Aggregates results returned by `CSSearchQuery` execution.
#[derive(Debug, Clone)]
pub struct CSSearchQueryExecutionResult {
    /// Items returned by `CSSearchQuery`.
    pub items: Vec<CSSearchableItem>,
    /// Mirrors `CSSearchQuery.foundItemCount`.
    pub found_item_count: usize,
    /// Reports whether `CSSearchQuery` was cancelled.
    pub cancelled: bool,
}

/// Aggregates results returned by `CSUserQuery` execution.
#[derive(Debug, Clone)]
pub struct CSUserQueryExecutionResult {
    /// Items returned by `CSUserQuery`.
    pub items: Vec<CSSearchableItem>,
    /// Mirrors `CSUserQuery.foundItemCount`.
    pub found_item_count: usize,
    /// Suggestions returned by `CSUserQuery`.
    pub suggestions: Vec<CSSuggestion>,
    /// Mirrors `CSUserQuery.foundSuggestionCount`.
    pub found_suggestion_count: usize,
    /// Reports whether `CSUserQuery` was cancelled.
    pub cancelled: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchQueryExecutionPayload {
    item_pointers: Vec<u64>,
    found_item_count: u64,
    cancelled: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserQueryExecutionPayload {
    item_pointers: Vec<u64>,
    found_item_count: u64,
    suggestion_pointers: Vec<u64>,
    found_suggestion_count: u64,
    cancelled: bool,
}

fn timeout_seconds(timeout: Duration) -> i32 {
    let seconds = timeout.as_secs().max(1);
    seconds.min(i32::MAX as u64) as i32
}

fn items_from_pointers(
    pointers: Vec<u64>,
    context: &str,
) -> Result<Vec<CSSearchableItem>, CoreSpotlightError> {
    pointers
        .into_iter()
        .map(|pointer| {
            let pointer = pointer as usize;
            let raw_pointer = pointer as *mut core::ffi::c_void;
            unsafe { CSSearchableItem::from_retained_ptr(raw_pointer, context) }
        })
        .collect()
}

fn suggestions_from_pointers(
    pointers: Vec<u64>,
    context: &str,
) -> Result<Vec<CSSuggestion>, CoreSpotlightError> {
    pointers
        .into_iter()
        .map(|pointer| {
            let pointer = pointer as usize;
            let raw_pointer = pointer as *mut core::ffi::c_void;
            unsafe { CSSuggestion::from_retained_ptr(raw_pointer, context) }
        })
        .collect()
}

fn search_query_protection_classes(
    object_ptr: *mut core::ffi::c_void,
) -> Result<Vec<String>, CoreSpotlightError> {
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_search_query_get_protection_classes(object_ptr, &mut out_json, &mut out_error)
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, "search query protection classes") }
}

fn set_search_query_protection_classes<I, S>(
    object_ptr: *mut core::ffi::c_void,
    values: I,
) -> Result<(), CoreSpotlightError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let values = values.into_iter().map(Into::into).collect::<Vec<_>>();
    let values_json = json_cstring(&values, "search query protection classes")?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_search_query_set_protection_classes(
            object_ptr,
            values_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

impl CSSearchQuery {
    /// Wraps the `CSSearchQuery` initializer.
    pub fn new(
        query_string: impl AsRef<str>,
        query_context: Option<&CSSearchQueryContext>,
    ) -> Result<Self, CoreSpotlightError> {
        let query_string = cstring_from_str(query_string.as_ref(), "search query string")?;
        let mut out_query = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_search_query_new(
                query_string.as_ptr(),
                query_context.map_or(core::ptr::null_mut(), CSSearchQueryContext::as_ptr),
                &mut out_query,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_query, "search query") }
    }

    /// Wraps a convenience initializer for `CSSearchQuery`.
    pub fn new_with_attributes<I, S>(
        query_string: impl AsRef<str>,
        attributes: I,
    ) -> Result<Self, CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let query_string = cstring_from_str(query_string.as_ref(), "search query string")?;
        let attributes = attributes.into_iter().map(Into::into).collect::<Vec<_>>();
        let attributes_json = json_cstring(&attributes, "search query attributes")?;
        let mut out_query = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_search_query_new_with_attributes(
                query_string.as_ptr(),
                attributes_json.as_ptr(),
                &mut out_query,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_query, "search query") }
    }

    /// Wraps the corresponding `CSSearchQuery` operation.
    pub fn execute(
        &self,
        timeout: Duration,
    ) -> Result<CSSearchQueryExecutionResult, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_search_query_execute(
                self.as_ptr(),
                timeout_seconds(timeout),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: SearchQueryExecutionPayload =
            unsafe { parse_json_ptr(out_json, "search query execution result")? };
        Ok(CSSearchQueryExecutionResult {
            items: items_from_pointers(payload.item_pointers, "search query execution item")?,
            found_item_count: payload.found_item_count as usize,
            cancelled: payload.cancelled,
        })
    }

    /// Wraps the corresponding `CSSearchQuery` cancellation API.
    pub fn cancel(&self) {
        unsafe { ffi::cs_search_query_cancel(self.as_ptr()) };
    }

    /// Wraps the corresponding `CSSearchQuery` getter.
    pub fn is_cancelled(&self) -> bool {
        unsafe { ffi::cs_search_query_is_cancelled(self.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `CSSearchQuery` getter.
    pub fn found_item_count(&self) -> usize {
        unsafe { ffi::cs_search_query_found_item_count(self.as_ptr()) as usize }
    }

    /// Wraps the corresponding `CSSearchQuery` getter.
    pub fn protection_classes(&self) -> Result<Vec<String>, CoreSpotlightError> {
        search_query_protection_classes(self.as_ptr())
    }

    /// Wraps the corresponding `CSSearchQuery` setter.
    pub fn set_protection_classes<I, S>(&self, values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        set_search_query_protection_classes(self.as_ptr(), values)
    }
}

impl CSUserQuery {
    /// Wraps the corresponding `CSUserQuery` operation.
    pub fn prepare() -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_user_query_prepare(&mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSUserQuery` operation.
    pub fn prepare_protection_classes<I, S>(values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values = values.into_iter().map(Into::into).collect::<Vec<_>>();
        let values_json = json_cstring(&values, "user query protection classes")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_prepare_protection_classes(values_json.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the `CSUserQuery` initializer.
    pub fn new(
        user_query_string: Option<&str>,
        user_query_context: Option<&CSUserQueryContext>,
    ) -> Result<Self, CoreSpotlightError> {
        let user_query_string = optional_cstring_from_str(user_query_string, "user query string")?;
        let mut out_query = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_new(
                opt_cstring_ptr(user_query_string.as_ref()),
                user_query_context.map_or(core::ptr::null_mut(), CSUserQueryContext::as_ptr),
                &mut out_query,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_query, "user query") }
    }

    /// Wraps the corresponding `CSUserQuery` operation.
    pub fn execute(
        &self,
        timeout: Duration,
    ) -> Result<CSUserQueryExecutionResult, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_execute(
                self.as_ptr(),
                timeout_seconds(timeout),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: UserQueryExecutionPayload =
            unsafe { parse_json_ptr(out_json, "user query execution result")? };
        Ok(CSUserQueryExecutionResult {
            items: items_from_pointers(payload.item_pointers, "user query execution item")?,
            found_item_count: payload.found_item_count as usize,
            suggestions: suggestions_from_pointers(
                payload.suggestion_pointers,
                "user query execution suggestion",
            )?,
            found_suggestion_count: payload.found_suggestion_count as usize,
            cancelled: payload.cancelled,
        })
    }

    /// Wraps the corresponding `CSUserQuery` getter.
    pub fn found_item_count(&self) -> usize {
        unsafe { ffi::cs_search_query_found_item_count(self.as_ptr()) as usize }
    }

    /// Wraps the corresponding `CSUserQuery` getter.
    pub fn found_suggestion_count(&self) -> usize {
        unsafe { ffi::cs_user_query_found_suggestion_count(self.as_ptr()) as usize }
    }

    /// Wraps the corresponding `CSUserQuery` cancellation API.
    pub fn cancel(&self) {
        unsafe { ffi::cs_search_query_cancel(self.as_ptr()) };
    }

    /// Wraps the corresponding `CSUserQuery` getter.
    pub fn is_cancelled(&self) -> bool {
        unsafe { ffi::cs_search_query_is_cancelled(self.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `CSUserQuery` getter.
    pub fn protection_classes(&self) -> Result<Vec<String>, CoreSpotlightError> {
        search_query_protection_classes(self.as_ptr())
    }

    /// Wraps the corresponding `CSUserQuery` setter.
    pub fn set_protection_classes<I, S>(&self, values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        set_search_query_protection_classes(self.as_ptr(), values)
    }

    /// Wraps the corresponding `CSUserQuery` operation.
    pub fn user_engaged_with_item(
        &self,
        item: &CSSearchableItem,
        visible_items: &[CSSearchableItem],
        interaction: CSUserInteraction,
    ) -> Result<(), CoreSpotlightError> {
        let visible_items = visible_items
            .iter()
            .map(|item| item.as_ptr() as usize as u64)
            .collect::<Vec<_>>();
        let visible_items_json = json_cstring(&visible_items, "visible user query items")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_user_engaged_with_item(
                self.as_ptr(),
                item.as_ptr(),
                visible_items_json.as_ptr(),
                interaction.raw_value(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSUserQuery` operation.
    pub fn user_engaged_with_suggestion(
        &self,
        suggestion: &CSSuggestion,
        visible_suggestions: &[CSSuggestion],
        interaction: CSUserInteraction,
    ) -> Result<(), CoreSpotlightError> {
        let visible_suggestions = visible_suggestions
            .iter()
            .map(|suggestion| suggestion.as_ptr() as usize as u64)
            .collect::<Vec<_>>();
        let visible_suggestions_json =
            json_cstring(&visible_suggestions, "visible user query suggestions")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_query_user_engaged_with_suggestion(
                self.as_ptr(),
                suggestion.as_ptr(),
                visible_suggestions_json.as_ptr(),
                interaction.raw_value(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
