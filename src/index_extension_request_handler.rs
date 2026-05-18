//! Wrappers for `CSIndexExtensionRequestHandler`.

use core::ffi::c_void;

use crate::delegate::{self, CSSearchableIndexDelegateCallbacks, SearchableIndexDelegateState};
use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::index::CSSearchableIndex;
use crate::item::CSSearchableItem;
use crate::private::{error_from_status, impl_object_wrapper};

impl_object_wrapper!(CSIndexExtensionRequestHandler);

impl CSIndexExtensionRequestHandler {
    /// Wraps the `CSIndexExtensionRequestHandler` initializer.
    pub fn new(callbacks: CSSearchableIndexDelegateCallbacks) -> Result<Self, CoreSpotlightError> {
        let state = Box::new(SearchableIndexDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let mut out_handler = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_index_extension_request_handler_new(
                context,
                Some(delegate::release_delegate_context),
                Some(delegate::delegate_reindex_all),
                Some(delegate::delegate_reindex_identifiers),
                Some(delegate::delegate_did_throttle),
                Some(delegate::delegate_did_finish_throttle),
                Some(delegate::delegate_data_for_item),
                Some(delegate::delegate_file_url_for_item),
                Some(delegate::delegate_searchable_items_for_identifiers),
                Some(delegate::delegate_searchable_items_did_update),
                &mut out_handler,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            unsafe {
                delegate::release_delegate_context(context);
            }
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_handler, "index extension request handler") }
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_reindex_all(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_reindex_all_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
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

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_did_throttle(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_did_throttle_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_did_finish_throttle(
        &self,
        index: &CSSearchableIndex,
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_did_finish_throttle_for_ptr(self.as_ptr(), index)
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_data_request(
        &self,
        index: &CSSearchableIndex,
        item_identifier: &str,
        type_identifier: &str,
    ) -> Result<Vec<u8>, CoreSpotlightError> {
        delegate::simulate_data_request_for_ptr(
            self.as_ptr(),
            index,
            item_identifier,
            type_identifier,
        )
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_file_url_request(
        &self,
        index: &CSSearchableIndex,
        item_identifier: &str,
        type_identifier: &str,
        in_place: bool,
    ) -> Result<Option<String>, CoreSpotlightError> {
        delegate::simulate_file_url_request_for_ptr(
            self.as_ptr(),
            index,
            item_identifier,
            type_identifier,
            in_place,
        )
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_searchable_items_for_identifiers<I, S>(
        &self,
        identifiers: I,
    ) -> Result<Vec<CSSearchableItem>, CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        delegate::simulate_searchable_items_for_identifiers_for_ptr(self.as_ptr(), identifiers)
    }

    /// Wraps the corresponding `CSIndexExtensionRequestHandler` operation.
    pub fn simulate_searchable_items_did_update(
        &self,
        items: &[CSSearchableItem],
    ) -> Result<(), CoreSpotlightError> {
        delegate::simulate_searchable_items_did_update_for_ptr(self.as_ptr(), items)
    }
}
