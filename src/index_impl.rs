use crate::default_index_extension_request_handler::DefaultIndexExtensionRequestHandler;
use crate::delegate::CSSearchableIndexDelegate;
use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::index_extension_request_handler::CSIndexExtensionRequestHandler;
use crate::item::CSSearchableItem;
use crate::private::{cstring_from_str, error_from_status, impl_object_wrapper, json_cstring};

impl_object_wrapper!(CSSearchableIndex);

impl CSSearchableIndex {
    pub fn is_indexing_available() -> bool {
        unsafe { ffi::cs_searchable_index_is_indexing_available() != 0 }
    }

    pub fn default_searchable_index() -> Result<Self, CoreSpotlightError> {
        let mut out_index = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_default_searchable_index(&mut out_index, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_index, "searchable index") }
    }

    pub fn new(name: impl AsRef<str>) -> Result<Self, CoreSpotlightError> {
        let name = cstring_from_str(name.as_ref(), "searchable index name")?;
        let mut out_index = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_new(name.as_ptr(), &mut out_index, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_index, "searchable index") }
    }

    pub fn new_with_protection_class(
        name: impl AsRef<str>,
        protection_class: Option<&str>,
    ) -> Result<Self, CoreSpotlightError> {
        let name = cstring_from_str(name.as_ref(), "searchable index name")?;
        let protection_class = protection_class
            .map(|value| cstring_from_str(value, "searchable index protection class"))
            .transpose()?;
        let mut out_index = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_new_with_protection_class(
                name.as_ptr(),
                protection_class
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut out_index,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_index, "searchable index") }
    }

    fn set_delegate_ptr(
        &self,
        delegate_ptr: *mut core::ffi::c_void,
    ) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_set_delegate(self.as_ptr(), delegate_ptr, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn set_delegate(
        &self,
        delegate: Option<&CSSearchableIndexDelegate>,
    ) -> Result<(), CoreSpotlightError> {
        self.set_delegate_ptr(
            delegate.map_or(core::ptr::null_mut(), CSSearchableIndexDelegate::as_ptr),
        )
    }

    pub fn set_extension_request_handler(
        &self,
        handler: Option<&CSIndexExtensionRequestHandler>,
    ) -> Result<(), CoreSpotlightError> {
        self.set_delegate_ptr(
            handler.map_or(core::ptr::null_mut(), CSIndexExtensionRequestHandler::as_ptr),
        )
    }

    pub fn set_default_extension_request_handler(
        &self,
        handler: Option<&DefaultIndexExtensionRequestHandler>,
    ) -> Result<(), CoreSpotlightError> {
        self.set_delegate_ptr(handler.map_or(
            core::ptr::null_mut(),
            DefaultIndexExtensionRequestHandler::as_ptr,
        ))
    }

    pub fn index_searchable_items(
        &self,
        items: &[CSSearchableItem],
    ) -> Result<(), CoreSpotlightError> {
        let payload = items
            .iter()
            .map(|item| item.as_ptr() as usize as u64)
            .collect::<Vec<_>>();
        let items_json = json_cstring(&payload, "searchable item pointers")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_index_items(
                self.as_ptr(),
                items_json.as_ptr(),
                30,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn delete_searchable_items_with_identifiers<I, S>(
        &self,
        identifiers: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let identifiers = identifiers.into_iter().map(Into::into).collect::<Vec<_>>();
        let identifiers_json = json_cstring(&identifiers, "identifier array")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_delete_identifiers(
                self.as_ptr(),
                identifiers_json.as_ptr(),
                30,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn delete_searchable_items_with_domain_identifiers<I, S>(
        &self,
        identifiers: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let identifiers = identifiers.into_iter().map(Into::into).collect::<Vec<_>>();
        let identifiers_json = json_cstring(&identifiers, "domain identifier array")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_delete_domain_identifiers(
                self.as_ptr(),
                identifiers_json.as_ptr(),
                30,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn delete_all_searchable_items(&self) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_searchable_index_delete_all(self.as_ptr(), 30, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn begin_index_batch(&self) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { ffi::cs_searchable_index_begin_batch(self.as_ptr(), &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn end_index_batch_with_client_state(
        &self,
        client_state: &[u8],
    ) -> Result<(), CoreSpotlightError> {
        let client_state_json = json_cstring(client_state, "client state")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_end_batch_with_client_state(
                self.as_ptr(),
                client_state_json.as_ptr(),
                30,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn end_index_batch_with_expected_client_state(
        &self,
        expected_client_state: Option<&[u8]>,
        new_client_state: &[u8],
    ) -> Result<(), CoreSpotlightError> {
        let expected_client_state_json = expected_client_state
            .map(|value| json_cstring(value, "expected client state"))
            .transpose()?;
        let new_client_state_json = json_cstring(new_client_state, "new client state")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_end_batch_with_expected_client_state(
                self.as_ptr(),
                expected_client_state_json
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                new_client_state_json.as_ptr(),
                30,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn fetch_last_client_state(&self) -> Result<Vec<u8>, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_fetch_last_client_state(
                self.as_ptr(),
                30,
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { crate::private::parse_json_ptr(out_json, "last client state") }
    }

    pub fn fetch_data_for_bundle_identifier(
        &self,
        bundle_identifier: impl AsRef<str>,
        item_identifier: impl AsRef<str>,
        content_type: impl AsRef<str>,
    ) -> Result<Vec<u8>, CoreSpotlightError> {
        let bundle_identifier =
            cstring_from_str(bundle_identifier.as_ref(), "bundle identifier")?;
        let item_identifier = cstring_from_str(item_identifier.as_ref(), "item identifier")?;
        let content_type = cstring_from_str(content_type.as_ref(), "content type")?;
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_index_fetch_data_for_bundle_identifier(
                self.as_ptr(),
                bundle_identifier.as_ptr(),
                item_identifier.as_ptr(),
                content_type.as_ptr(),
                30,
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { crate::private::parse_json_ptr(out_json, "external provider data") }
    }
}
