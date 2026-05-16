use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, json_cstring, opt_cstring_ptr,
    optional_cstring_from_str, parse_json_ptr, take_string,
};

impl_object_wrapper!(CSSearchableItemAttributeSet);

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

fn get_optional_f64(
    object_ptr: *mut core::ffi::c_void,
    getter: unsafe extern "C" fn(*mut core::ffi::c_void, *mut f64) -> i32,
) -> Option<f64> {
    let mut value = 0.0;
    let has_value = unsafe { getter(object_ptr, &mut value) };
    if has_value == 0 {
        return None;
    }
    Some(value)
}

fn set_optional_f64(
    object_ptr: *mut core::ffi::c_void,
    value: Option<f64>,
    setter: unsafe extern "C" fn(*mut core::ffi::c_void, f64, i32, *mut *mut core::ffi::c_char) -> i32,
) -> Result<(), CoreSpotlightError> {
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        setter(
            object_ptr,
            value.unwrap_or_default(),
            i32::from(value.is_some()),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

impl CSSearchableItemAttributeSet {
    pub fn new(item_content_type: impl AsRef<str>) -> Result<Self, CoreSpotlightError> {
        let item_content_type = cstring_from_str(item_content_type.as_ref(), "item content type")?;
        let mut out_attribute_set = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_new(
                item_content_type.as_ptr(),
                &mut out_attribute_set,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_attribute_set, "searchable item attribute set") }
    }

    pub fn item_content_type(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_attribute_set_get_item_content_type)
    }

    pub fn set_item_content_type(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "item content type",
            ffi::cs_attribute_set_set_item_content_type,
        )
    }

    pub fn title(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_attribute_set_get_title)
    }

    pub fn set_title(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(self.as_ptr(), value, "title", ffi::cs_attribute_set_set_title)
    }

    pub fn content_description(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_attribute_set_get_content_description)
    }

    pub fn set_content_description(
        &self,
        value: Option<&str>,
    ) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "content description",
            ffi::cs_attribute_set_set_content_description,
        )
    }

    pub fn display_name(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_attribute_set_get_display_name)
    }

    pub fn set_display_name(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "display name",
            ffi::cs_attribute_set_set_display_name,
        )
    }

    pub fn keywords(&self) -> Result<Vec<String>, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_get_keywords(self.as_ptr(), &mut out_json, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "keywords") }
    }

    pub fn set_keywords<I, S>(&self, values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values = values.into_iter().map(Into::into).collect::<Vec<_>>();
        let values_json = json_cstring(&values, "keywords")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_set_keywords(
                self.as_ptr(),
                values_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn thumbnail_data(&self) -> Result<Vec<u8>, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_get_thumbnail_data(self.as_ptr(), &mut out_json, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "thumbnail data") }
    }

    pub fn set_thumbnail_data(&self, data: &[u8]) -> Result<(), CoreSpotlightError> {
        let data_json = json_cstring(data, "thumbnail data")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_set_thumbnail_data(
                self.as_ptr(),
                data_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    pub fn thumbnail_url(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_attribute_set_get_thumbnail_url)
    }

    pub fn set_thumbnail_url(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "thumbnail URL",
            ffi::cs_attribute_set_set_thumbnail_url,
        )
    }

    pub fn content_url(&self) -> Option<String> {
        get_optional_string(self.as_ptr(), ffi::cs_attribute_set_get_content_url)
    }

    pub fn set_content_url(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        set_optional_string(
            self.as_ptr(),
            value,
            "content URL",
            ffi::cs_attribute_set_set_content_url,
        )
    }

    pub fn latitude(&self) -> Option<f64> {
        get_optional_f64(self.as_ptr(), ffi::cs_attribute_set_get_latitude)
    }

    pub fn set_latitude(&self, value: Option<f64>) -> Result<(), CoreSpotlightError> {
        set_optional_f64(self.as_ptr(), value, ffi::cs_attribute_set_set_latitude)
    }

    pub fn longitude(&self) -> Option<f64> {
        get_optional_f64(self.as_ptr(), ffi::cs_attribute_set_get_longitude)
    }

    pub fn set_longitude(&self, value: Option<f64>) -> Result<(), CoreSpotlightError> {
        set_optional_f64(self.as_ptr(), value, ffi::cs_attribute_set_set_longitude)
    }

    pub fn rating(&self) -> Option<f64> {
        get_optional_f64(self.as_ptr(), ffi::cs_attribute_set_get_rating)
    }

    pub fn set_rating(&self, value: Option<f64>) -> Result<(), CoreSpotlightError> {
        set_optional_f64(self.as_ptr(), value, ffi::cs_attribute_set_set_rating)
    }
}
