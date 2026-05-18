//! Helpers for the bridge-backed `DefaultIndexExtensionRequestHandler` test double.

use crate::delegate;
use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::index::CSSearchableIndex;
use crate::item::CSSearchableItem;
use crate::private::{error_from_status, impl_object_wrapper, parse_json_ptr};

impl_object_wrapper!(DefaultIndexExtensionRequestHandler);

impl DefaultIndexExtensionRequestHandler {
    /// Wraps the `DefaultIndexExtensionRequestHandler` initializer.
    pub fn new() -> Result<Self, CoreSpotlightError> {
        let mut out_handler = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_default_index_extension_request_handler_new(&mut out_handler, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_handler, "default index extension request handler") }
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` getter.
    pub fn reindex_all_count(&self) -> u64 {
        unsafe {
            ffi::cs_default_index_extension_request_handler_get_reindex_all_count(self.as_ptr())
        }
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` getter.
    pub fn reindex_identifiers_count(&self) -> u64 {
        unsafe {
            ffi::cs_default_index_extension_request_handler_get_reindex_identifiers_count(
                self.as_ptr(),
            )
        }
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` getter.
    pub fn did_throttle_count(&self) -> u64 {
        unsafe {
            ffi::cs_default_index_extension_request_handler_get_did_throttle_count(self.as_ptr())
        }
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` getter.
    pub fn did_finish_throttle_count(&self) -> u64 {
        unsafe {
            ffi::cs_default_index_extension_request_handler_get_did_finish_throttle_count(
                self.as_ptr(),
            )
        }
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` getter.
    pub fn last_identifiers(&self) -> Result<Vec<String>, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_default_index_extension_request_handler_get_last_identifiers(
                self.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "default request handler identifiers") }
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` operation.
    pub fn simulate_reindex_all(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_reindex_all_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` operation.
    pub fn simulate_reindex_identifiers<I, S>(
        &self,
        index: &CSSearchableIndex,
        identifiers: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        delegate::simulate_reindex_identifiers_for_ptr(self.as_ptr(), index, identifiers)
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` operation.
    pub fn simulate_did_throttle(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_did_throttle_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` operation.
    pub fn simulate_did_finish_throttle(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_did_finish_throttle_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `DefaultIndexExtensionRequestHandler` operation.
    pub fn simulate_searchable_items_did_update(
        &self,
        items: &[CSSearchableItem],
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_searchable_items_did_update_for_ptr(self.as_ptr(), items)
    }
}
