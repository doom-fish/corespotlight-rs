//! Wrappers for `CSSearchableIndexDelegate` and Rust callback builders.

use core::ffi::{c_char, c_void};
use std::ffi::CStr;

use serde::Serialize;

use crate::error::{CoreSpotlightError, ErrorPayload};
use crate::ffi;
use crate::index::CSSearchableIndex;
use crate::item::CSSearchableItem;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, json_cstring, parse_json_ptr,
    parse_json_str,
};

impl_object_wrapper!(CSSearchableIndexDelegate);

unsafe extern "C" {
    fn strdup(value: *const c_char) -> *mut c_char;
}

type ReindexAllFn = dyn Fn(CSSearchableIndex) + Send + Sync + 'static;
type ReindexIdentifiersFn = dyn Fn(CSSearchableIndex, Vec<String>) + Send + Sync + 'static;
type NotificationFn = dyn Fn(CSSearchableIndex) + Send + Sync + 'static;
type DataForItemFn = dyn Fn(CSSearchableIndex, &str, &str) -> Result<Option<Vec<u8>>, CoreSpotlightError>
    + Send
    + Sync
    + 'static;
type FileUrlForItemFn = dyn Fn(CSSearchableIndex, &str, &str, bool) -> Result<Option<String>, CoreSpotlightError>
    + Send
    + Sync
    + 'static;
type SearchableItemsForIdentifiersFn = dyn Fn(Vec<String>) -> Result<Vec<CSSearchableItem>, CoreSpotlightError>
    + Send
    + Sync
    + 'static;
type SearchableItemsDidUpdateFn = dyn Fn(Vec<CSSearchableItem>) + Send + Sync + 'static;

/// Builder for Rust callbacks that back `CSSearchableIndexDelegate`.
pub struct CSSearchableIndexDelegateCallbacks {
    reindex_all: Box<ReindexAllFn>,
    reindex_identifiers: Box<ReindexIdentifiersFn>,
    did_throttle: Option<Box<NotificationFn>>,
    did_finish_throttle: Option<Box<NotificationFn>>,
    data_for_item: Option<Box<DataForItemFn>>,
    file_url_for_item: Option<Box<FileUrlForItemFn>>,
    searchable_items_for_identifiers: Option<Box<SearchableItemsForIdentifiersFn>>,
    searchable_items_did_update: Option<Box<SearchableItemsDidUpdateFn>>,
}

impl CSSearchableIndexDelegateCallbacks {
    /// Wraps the `CSSearchableIndexDelegate` initializer.
    pub fn new<R1, R2>(reindex_all: R1, reindex_identifiers: R2) -> Self
    where
        R1: Fn(CSSearchableIndex) + Send + Sync + 'static,
        R2: Fn(CSSearchableIndex, Vec<String>) + Send + Sync + 'static,
    {
        Self {
            reindex_all: Box::new(reindex_all),
            reindex_identifiers: Box::new(reindex_identifiers),
            did_throttle: None,
            did_finish_throttle: None,
            data_for_item: None,
            file_url_for_item: None,
            searchable_items_for_identifiers: None,
            searchable_items_did_update: None,
        }
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` method.
    pub fn did_throttle<F>(mut self, callback: F) -> Self
    where
        F: Fn(CSSearchableIndex) + Send + Sync + 'static,
    {
        self.did_throttle = Some(Box::new(callback));
        self
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` method.
    pub fn did_finish_throttle<F>(mut self, callback: F) -> Self
    where
        F: Fn(CSSearchableIndex) + Send + Sync + 'static,
    {
        self.did_finish_throttle = Some(Box::new(callback));
        self
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` method.
    pub fn data_for_item<F>(mut self, callback: F) -> Self
    where
        F: Fn(CSSearchableIndex, &str, &str) -> Result<Option<Vec<u8>>, CoreSpotlightError>
            + Send
            + Sync
            + 'static,
    {
        self.data_for_item = Some(Box::new(callback));
        self
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` method.
    pub fn file_url_for_item<F>(mut self, callback: F) -> Self
    where
        F: Fn(CSSearchableIndex, &str, &str, bool) -> Result<Option<String>, CoreSpotlightError>
            + Send
            + Sync
            + 'static,
    {
        self.file_url_for_item = Some(Box::new(callback));
        self
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` method.
    pub fn searchable_items_for_identifiers<F>(mut self, callback: F) -> Self
    where
        F: Fn(Vec<String>) -> Result<Vec<CSSearchableItem>, CoreSpotlightError>
            + Send
            + Sync
            + 'static,
    {
        self.searchable_items_for_identifiers = Some(Box::new(callback));
        self
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` method.
    pub fn searchable_items_did_update<F>(mut self, callback: F) -> Self
    where
        F: Fn(Vec<CSSearchableItem>) + Send + Sync + 'static,
    {
        self.searchable_items_did_update = Some(Box::new(callback));
        self
    }
}

pub(crate) struct SearchableIndexDelegateState {
    pub(crate) callbacks: CSSearchableIndexDelegateCallbacks,
}

fn duplicate_c_string(value: &str, context: &str) -> Result<*mut c_char, CoreSpotlightError> {
    let value = cstring_from_str(value, context)?;
    let duplicated = unsafe { strdup(value.as_ptr()) };
    if duplicated.is_null() {
        return Err(CoreSpotlightError::bridge(
            -2,
            format!("failed to duplicate {context} as C string"),
        ));
    }
    Ok(duplicated)
}

fn write_json_payload<T: Serialize>(
    value: &T,
    context: &str,
    out_json: *mut *mut c_char,
) -> Result<(), CoreSpotlightError> {
    if out_json.is_null() {
        return Ok(());
    }
    let json = serde_json::to_string(value).map_err(|error| {
        CoreSpotlightError::bridge(-1, format!("failed to encode {context}: {error}"))
    })?;
    unsafe {
        *out_json = duplicate_c_string(&json, context)?;
    }
    Ok(())
}

fn write_error_payload(error: &CoreSpotlightError, out_error: *mut *mut c_char) {
    if out_error.is_null() {
        return;
    }
    let payload = ErrorPayload {
        domain: error.domain.clone(),
        code: error.code,
        message: error.message.clone(),
    };
    if let Ok(json) = serde_json::to_string(&payload) {
        if let Ok(ptr) = duplicate_c_string(&json, "delegate error payload") {
            unsafe {
                *out_error = ptr;
            }
        }
    }
}

unsafe fn state_from_context<'a>(context: *mut c_void) -> &'a SearchableIndexDelegateState {
    &*context.cast::<SearchableIndexDelegateState>()
}

fn identifiers_from_json(
    json: *const c_char,
    context: &str,
) -> Result<Vec<String>, CoreSpotlightError> {
    if json.is_null() {
        return Err(CoreSpotlightError::bridge(
            -1,
            format!("missing {context} JSON payload"),
        ));
    }
    let json = unsafe { CStr::from_ptr(json) }
        .to_string_lossy()
        .into_owned();
    parse_json_str(&json, context)
}

fn items_json(items: &[CSSearchableItem], context: &str) -> Result<String, CoreSpotlightError> {
    let pointers = items
        .iter()
        .map(|item| unsafe { ffi::cs_retain_object(item.as_ptr()) } as usize as u64)
        .collect::<Vec<_>>();
    serde_json::to_string(&pointers).map_err(|error| {
        CoreSpotlightError::bridge(-1, format!("failed to encode {context}: {error}"))
    })
}

pub(crate) unsafe extern "C" fn release_delegate_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    drop(Box::from_raw(
        context.cast::<SearchableIndexDelegateState>(),
    ));
}

pub(crate) unsafe extern "C" fn delegate_reindex_all(context: *mut c_void, index_ptr: *mut c_void) {
    if context.is_null() || index_ptr.is_null() {
        return;
    }
    let state = state_from_context(context);
    if let Ok(index) = CSSearchableIndex::from_retained_ptr(index_ptr, "delegate index") {
        (state.callbacks.reindex_all)(index);
    }
}

pub(crate) unsafe extern "C" fn delegate_reindex_identifiers(
    context: *mut c_void,
    index_ptr: *mut c_void,
    identifiers_json: *const c_char,
) {
    if context.is_null() || index_ptr.is_null() {
        return;
    }
    let state = state_from_context(context);
    let Ok(index) = CSSearchableIndex::from_retained_ptr(index_ptr, "delegate index") else {
        return;
    };
    let Ok(identifiers) = identifiers_from_json(identifiers_json, "delegate identifiers") else {
        return;
    };
    (state.callbacks.reindex_identifiers)(index, identifiers);
}

pub(crate) unsafe extern "C" fn delegate_did_throttle(
    context: *mut c_void,
    index_ptr: *mut c_void,
) {
    if context.is_null() || index_ptr.is_null() {
        return;
    }
    let state = state_from_context(context);
    let Some(callback) = state.callbacks.did_throttle.as_ref() else {
        return;
    };
    if let Ok(index) = CSSearchableIndex::from_retained_ptr(index_ptr, "delegate throttled index") {
        callback(index);
    }
}

pub(crate) unsafe extern "C" fn delegate_did_finish_throttle(
    context: *mut c_void,
    index_ptr: *mut c_void,
) {
    if context.is_null() || index_ptr.is_null() {
        return;
    }
    let state = state_from_context(context);
    let Some(callback) = state.callbacks.did_finish_throttle.as_ref() else {
        return;
    };
    if let Ok(index) =
        CSSearchableIndex::from_retained_ptr(index_ptr, "delegate throttle finished index")
    {
        callback(index);
    }
}

pub(crate) unsafe extern "C" fn delegate_data_for_item(
    context: *mut c_void,
    index_ptr: *mut c_void,
    item_identifier: *const c_char,
    type_identifier: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    if context.is_null() || index_ptr.is_null() {
        return ffi::status::INVALID_ARGUMENT;
    }
    let state = state_from_context(context);
    let Some(callback) = state.callbacks.data_for_item.as_ref() else {
        return ffi::status::OK;
    };
    let result = (|| {
        let index =
            unsafe { CSSearchableIndex::from_retained_ptr(index_ptr, "delegate data index")? };
        let item_identifier = if item_identifier.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(item_identifier) }
                .to_str()
                .map_err(|error| {
                    CoreSpotlightError::bridge(
                        -1,
                        format!("invalid item identifier utf-8: {error}"),
                    )
                })?
        };
        let type_identifier = if type_identifier.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(type_identifier) }
                .to_str()
                .map_err(|error| {
                    CoreSpotlightError::bridge(
                        -1,
                        format!("invalid type identifier utf-8: {error}"),
                    )
                })?
        };
        let data = callback(index, item_identifier, type_identifier)?;
        write_json_payload(&data.unwrap_or_default(), "delegate data payload", out_json)
    })();
    match result {
        Ok(()) => ffi::status::OK,
        Err(error) => {
            write_error_payload(&error, out_error);
            error.code as i32
        }
    }
}

pub(crate) unsafe extern "C" fn delegate_file_url_for_item(
    context: *mut c_void,
    index_ptr: *mut c_void,
    item_identifier: *const c_char,
    type_identifier: *const c_char,
    in_place: i32,
    out_url: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    if context.is_null() || index_ptr.is_null() {
        return ffi::status::INVALID_ARGUMENT;
    }
    let state = state_from_context(context);
    let Some(callback) = state.callbacks.file_url_for_item.as_ref() else {
        return ffi::status::OK;
    };
    let result = (|| {
        let index =
            unsafe { CSSearchableIndex::from_retained_ptr(index_ptr, "delegate file url index")? };
        let item_identifier = if item_identifier.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(item_identifier) }
                .to_str()
                .map_err(|error| {
                    CoreSpotlightError::bridge(
                        -1,
                        format!("invalid item identifier utf-8: {error}"),
                    )
                })?
        };
        let type_identifier = if type_identifier.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(type_identifier) }
                .to_str()
                .map_err(|error| {
                    CoreSpotlightError::bridge(
                        -1,
                        format!("invalid type identifier utf-8: {error}"),
                    )
                })?
        };
        if out_url.is_null() {
            return Ok(());
        }
        unsafe {
            *out_url = match callback(index, item_identifier, type_identifier, in_place != 0)? {
                Some(url) => duplicate_c_string(&url, "delegate file URL payload")?,
                None => core::ptr::null_mut(),
            };
        }
        Ok(())
    })();
    match result {
        Ok(()) => ffi::status::OK,
        Err(error) => {
            write_error_payload(&error, out_error);
            error.code as i32
        }
    }
}

pub(crate) unsafe extern "C" fn delegate_searchable_items_for_identifiers(
    context: *mut c_void,
    identifiers_json: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    if context.is_null() {
        return ffi::status::INVALID_ARGUMENT;
    }
    let state = state_from_context(context);
    let Some(callback) = state.callbacks.searchable_items_for_identifiers.as_ref() else {
        return ffi::status::OK;
    };
    let result = (|| {
        let identifiers =
            identifiers_from_json(identifiers_json, "delegate searchable items identifiers")?;
        let items = callback(identifiers)?;
        let items_json = items_json(&items, "delegate searchable items response")?;
        if !out_json.is_null() {
            unsafe {
                *out_json = duplicate_c_string(&items_json, "delegate searchable items JSON")?;
            }
        }
        Ok(())
    })();
    match result {
        Ok(()) => ffi::status::OK,
        Err(error) => {
            write_error_payload(&error, out_error);
            error.code as i32
        }
    }
}

pub(crate) unsafe extern "C" fn delegate_searchable_items_did_update(
    context: *mut c_void,
    items_json_ptr: *const c_char,
) {
    if context.is_null() {
        return;
    }
    let state = state_from_context(context);
    let Some(callback) = state.callbacks.searchable_items_did_update.as_ref() else {
        return;
    };
    let json = if items_json_ptr.is_null() {
        return;
    } else {
        unsafe { CStr::from_ptr(items_json_ptr) }
            .to_string_lossy()
            .into_owned()
    };
    let Ok(pointers) = parse_json_str::<Vec<u64>>(&json, "updated searchable items") else {
        return;
    };
    let items = pointers
        .into_iter()
        .map(|pointer| {
            let raw_pointer = pointer as usize as *mut c_void;
            unsafe { CSSearchableItem::from_retained_ptr(raw_pointer, "updated searchable item") }
        })
        .collect::<Result<Vec<_>, _>>();
    if let Ok(items) = items {
        callback(items);
    }
}

pub(crate) fn simulate_reindex_all_for_ptr(
    object_ptr: *mut c_void,
    index: &CSSearchableIndex,
) -> Result<(), CoreSpotlightError> {
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_reindex_all(
            object_ptr,
            index.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

pub(crate) fn simulate_reindex_identifiers_for_ptr<I, S>(
    object_ptr: *mut c_void,
    index: &CSSearchableIndex,
    identifiers: I,
) -> Result<(), CoreSpotlightError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let identifiers = identifiers.into_iter().map(Into::into).collect::<Vec<_>>();
    let identifiers_json = json_cstring(&identifiers, "delegate identifiers")?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_reindex_identifiers(
            object_ptr,
            index.as_ptr(),
            identifiers_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

pub(crate) fn simulate_did_throttle_for_ptr(
    object_ptr: *mut c_void,
    index: &CSSearchableIndex,
) -> Result<(), CoreSpotlightError> {
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_did_throttle(
            object_ptr,
            index.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

pub(crate) fn simulate_did_finish_throttle_for_ptr(
    object_ptr: *mut c_void,
    index: &CSSearchableIndex,
) -> Result<(), CoreSpotlightError> {
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_did_finish_throttle(
            object_ptr,
            index.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

pub(crate) fn simulate_data_request_for_ptr(
    object_ptr: *mut c_void,
    index: &CSSearchableIndex,
    item_identifier: &str,
    type_identifier: &str,
) -> Result<Vec<u8>, CoreSpotlightError> {
    let item_identifier = cstring_from_str(item_identifier, "delegate item identifier")?;
    let type_identifier = cstring_from_str(type_identifier, "delegate type identifier")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_data_request(
            object_ptr,
            index.as_ptr(),
            item_identifier.as_ptr(),
            type_identifier.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    if out_json.is_null() {
        return Ok(Vec::new());
    }
    unsafe { parse_json_ptr(out_json, "delegate data response") }
}

pub(crate) fn simulate_file_url_request_for_ptr(
    object_ptr: *mut c_void,
    index: &CSSearchableIndex,
    item_identifier: &str,
    type_identifier: &str,
    in_place: bool,
) -> Result<Option<String>, CoreSpotlightError> {
    let item_identifier = cstring_from_str(item_identifier, "delegate item identifier")?;
    let type_identifier = cstring_from_str(type_identifier, "delegate type identifier")?;
    let mut out_url = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_file_url_request(
            object_ptr,
            index.as_ptr(),
            item_identifier.as_ptr(),
            type_identifier.as_ptr(),
            i32::from(in_place),
            &mut out_url,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { Ok(crate::private::take_string(out_url)) }
}

pub(crate) fn simulate_searchable_items_for_identifiers_for_ptr<I, S>(
    object_ptr: *mut c_void,
    identifiers: I,
) -> Result<Vec<CSSearchableItem>, CoreSpotlightError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let identifiers = identifiers.into_iter().map(Into::into).collect::<Vec<_>>();
    let identifiers_json = json_cstring(&identifiers, "delegate searchable items identifiers")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_searchable_items_for_identifiers(
            object_ptr,
            identifiers_json.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    let pointers: Vec<u64> =
        unsafe { parse_json_ptr(out_json, "delegate searchable items result")? };
    pointers
        .into_iter()
        .map(|pointer| {
            let raw_pointer = pointer as usize as *mut c_void;
            unsafe { CSSearchableItem::from_retained_ptr(raw_pointer, "delegate searchable item") }
        })
        .collect()
}

pub(crate) fn simulate_searchable_items_did_update_for_ptr(
    object_ptr: *mut c_void,
    items: &[CSSearchableItem],
) -> Result<(), CoreSpotlightError> {
    let items = items
        .iter()
        .map(|item| unsafe { ffi::cs_retain_object(item.as_ptr()) } as usize as u64)
        .collect::<Vec<_>>();
    let items_json = json_cstring(&items, "updated searchable items")?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_searchable_index_delegate_simulate_searchable_items_did_update(
            object_ptr,
            items_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

impl CSSearchableIndexDelegate {
    /// Wraps the `CSSearchableIndexDelegate` initializer.
    pub fn new(callbacks: CSSearchableIndexDelegateCallbacks) -> Result<Self, CoreSpotlightError> {
        let state = Box::new(SearchableIndexDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let mut out_delegate = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_delegate_new(
                context,
                Some(release_delegate_context),
                Some(delegate_reindex_all),
                Some(delegate_reindex_identifiers),
                Some(delegate_did_throttle),
                Some(delegate_did_finish_throttle),
                Some(delegate_data_for_item),
                Some(delegate_file_url_for_item),
                Some(delegate_searchable_items_for_identifiers),
                Some(delegate_searchable_items_did_update),
                &mut out_delegate,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            unsafe {
                release_delegate_context(context);
            }
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_delegate, "searchable index delegate") }
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_reindex_all(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        simulate_reindex_all_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_reindex_identifiers<I, S>(
        &self,
        index: &CSSearchableIndex,
        identifiers: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        simulate_reindex_identifiers_for_ptr(self.as_ptr(), index, identifiers)
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_did_throttle(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        simulate_did_throttle_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_did_finish_throttle(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        simulate_did_finish_throttle_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_data_request(
        &self,
        index: &CSSearchableIndex,
        item_identifier: &str,
        type_identifier: &str,
    ) -> Result<Vec<u8>, CoreSpotlightError> {
        simulate_data_request_for_ptr(self.as_ptr(), index, item_identifier, type_identifier)
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_file_url_request(
        &self,
        index: &CSSearchableIndex,
        item_identifier: &str,
        type_identifier: &str,
        in_place: bool,
    ) -> Result<Option<String>, CoreSpotlightError> {
        simulate_file_url_request_for_ptr(
            self.as_ptr(),
            index,
            item_identifier,
            type_identifier,
            in_place,
        )
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_searchable_items_for_identifiers<I, S>(
        &self,
        identifiers: I,
    ) -> Result<Vec<CSSearchableItem>, CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        simulate_searchable_items_for_identifiers_for_ptr(self.as_ptr(), identifiers)
    }

    /// Wraps the corresponding `CSSearchableIndexDelegate` operation.
    pub fn simulate_searchable_items_did_update(
        &self,
        items: &[CSSearchableItem],
    ) -> Result<(), CoreSpotlightError> {
        simulate_searchable_items_did_update_for_ptr(self.as_ptr(), items)
    }
}
