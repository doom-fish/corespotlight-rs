use std::time::SystemTime;

use crate::attribute_set::CSSearchableItemAttributeSet;
use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::private::{
    error_from_status, impl_object_wrapper, opt_cstring_ptr, optional_cstring_from_str,
    system_time_from_unix_seconds, system_time_to_unix_seconds,
};

impl_object_wrapper!(CSSearchableItem);

impl CSSearchableItem {
    pub fn new(
        unique_identifier: Option<&str>,
        domain_identifier: Option<&str>,
        attribute_set: &CSSearchableItemAttributeSet,
    ) -> Result<Self, CoreSpotlightError> {
        let unique_identifier =
            optional_cstring_from_str(unique_identifier, "unique identifier")?;
        let domain_identifier =
            optional_cstring_from_str(domain_identifier, "domain identifier")?;
        let mut out_item = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_new(
                opt_cstring_ptr(unique_identifier.as_ref()),
                opt_cstring_ptr(domain_identifier.as_ref()),
                attribute_set.as_ptr(),
                &mut out_item,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_item, "searchable item") }
    }

    pub fn expiration_date(&self) -> Option<SystemTime> {
        let mut unix_seconds = 0.0;
        let has_value = unsafe {
            ffi::cs_searchable_item_get_expiration_date(self.as_ptr(), &mut unix_seconds)
        };
        if has_value == 0 {
            return None;
        }
        Some(system_time_from_unix_seconds(unix_seconds))
    }

    pub fn set_expiration_date(
        &self,
        expiration_date: Option<SystemTime>,
    ) -> Result<(), CoreSpotlightError> {
        let unix_seconds = expiration_date
            .map(system_time_to_unix_seconds)
            .transpose()?
            .unwrap_or_default();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_set_expiration_date(
                self.as_ptr(),
                unix_seconds,
                i32::from(expiration_date.is_some()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
