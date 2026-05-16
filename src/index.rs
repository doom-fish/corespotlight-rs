use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::item::CSSearchableItem;
use crate::private::{cstring_from_str, error_from_status, impl_object_wrapper, json_cstring};

impl_object_wrapper!(CSSearchableIndex);

impl CSSearchableIndex {
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
        let status =
            unsafe { ffi::cs_searchable_index_delete_all(self.as_ptr(), 30, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
