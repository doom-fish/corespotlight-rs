use std::time::SystemTime;

use crate::attribute_set::CSSearchableItemAttributeSet;
use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, opt_cstring_ptr,
    optional_cstring_from_str, system_time_from_unix_seconds, system_time_to_unix_seconds,
    take_string,
};
use crate::settings::CSSearchableItemUpdateListenerOptions;

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

    pub fn compare_by_rank(&self, other: &Self) -> core::cmp::Ordering {
        match unsafe { ffi::cs_searchable_item_compare_by_rank(self.as_ptr(), other.as_ptr()) } {
            value if value < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        }
    }

    pub fn unique_identifier(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_searchable_item_get_unique_identifier(self.as_ptr())) }
    }

    pub fn set_unique_identifier(
        &self,
        unique_identifier: impl AsRef<str>,
    ) -> Result<(), CoreSpotlightError> {
        let unique_identifier =
            cstring_from_str(unique_identifier.as_ref(), "unique identifier")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_set_unique_identifier(
                self.as_ptr(),
                unique_identifier.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn domain_identifier(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_searchable_item_get_domain_identifier(self.as_ptr())) }
    }

    pub fn set_domain_identifier(
        &self,
        domain_identifier: Option<&str>,
    ) -> Result<(), CoreSpotlightError> {
        let domain_identifier = optional_cstring_from_str(domain_identifier, "domain identifier")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_set_domain_identifier(
                self.as_ptr(),
                opt_cstring_ptr(domain_identifier.as_ref()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
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

    pub fn attribute_set(&self) -> Result<CSSearchableItemAttributeSet, CoreSpotlightError> {
        let mut out_attribute_set = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_get_attribute_set(
                self.as_ptr(),
                &mut out_attribute_set,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe {
            CSSearchableItemAttributeSet::from_retained_ptr(
                out_attribute_set,
                "searchable item attribute set",
            )
        }
    }

    pub fn set_attribute_set(
        &self,
        attribute_set: &CSSearchableItemAttributeSet,
    ) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_set_attribute_set(
                self.as_ptr(),
                attribute_set.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn is_update(&self) -> bool {
        unsafe { ffi::cs_searchable_item_get_is_update(self.as_ptr()) != 0 }
    }

    pub fn set_is_update(&self, is_update: bool) {
        unsafe { ffi::cs_searchable_item_set_is_update(self.as_ptr(), i32::from(is_update)) };
    }

    pub fn update_listener_options(&self) -> Result<CSSearchableItemUpdateListenerOptions, CoreSpotlightError> {
        let mut raw_value = 0_u64;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_get_update_listener_options(
                self.as_ptr(),
                &mut raw_value,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(CSSearchableItemUpdateListenerOptions::new(raw_value))
    }

    pub fn set_update_listener_options(
        &self,
        options: CSSearchableItemUpdateListenerOptions,
    ) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_searchable_item_set_update_listener_options(
                self.as_ptr(),
                options.bits(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
