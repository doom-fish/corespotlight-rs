use core::ffi::{c_char, c_void};
use core::ptr::NonNull;
use std::ffi::{CStr, CString};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{CoreSpotlightError, ErrorPayload};
use crate::ffi;

#[derive(Debug)]
pub(crate) struct RetainedObject(NonNull<c_void>);

unsafe impl Send for RetainedObject {}

impl Clone for RetainedObject {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::cs_retain_object(self.as_ptr()) };
        Self(NonNull::new(ptr).expect("Core Spotlight retain returned a null pointer"))
    }
}

impl Drop for RetainedObject {
    fn drop(&mut self) {
        unsafe {
            ffi::cs_release_object(self.as_ptr());
        }
    }
}

impl RetainedObject {
    pub(crate) unsafe fn from_retained_ptr(
        ptr: *mut c_void,
        context: &str,
    ) -> Result<Self, CoreSpotlightError> {
        NonNull::new(ptr).map(Self).ok_or_else(|| {
            CoreSpotlightError::bridge(-2, format!("missing object pointer for {context}"))
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

macro_rules! impl_object_wrapper {
    ($name:ident) => {
        #[doc = concat!("Wraps `", stringify!($name), "`.")]
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) inner: $crate::private::RetainedObject,
        }

        unsafe impl Send for $name {}

        impl $name {
            pub(crate) unsafe fn from_retained_ptr(
                ptr: *mut core::ffi::c_void,
                context: &str,
            ) -> Result<Self, $crate::error::CoreSpotlightError> {
                Ok(Self {
                    inner: $crate::private::RetainedObject::from_retained_ptr(ptr, context)?,
                })
            }

            pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
                self.inner.as_ptr()
            }
        }
    };
}

pub(crate) use impl_object_wrapper;

pub(crate) fn cstring_from_str(value: &str, context: &str) -> Result<CString, CoreSpotlightError> {
    CString::new(value).map_err(|error| {
        CoreSpotlightError::bridge(
            -1,
            format!("{context} contains an interior NUL byte: {error}"),
        )
    })
}

pub(crate) fn optional_cstring_from_str(
    value: Option<&str>,
    context: &str,
) -> Result<Option<CString>, CoreSpotlightError> {
    value
        .map(|value| cstring_from_str(value, context))
        .transpose()
}

pub(crate) fn opt_cstring_ptr(value: Option<&CString>) -> *const c_char {
    value.map_or(core::ptr::null(), |value| value.as_c_str().as_ptr())
}

pub(crate) fn json_cstring<T: Serialize + ?Sized>(
    value: &T,
    context: &str,
) -> Result<CString, CoreSpotlightError> {
    let json = serde_json::to_string(value).map_err(|error| {
        CoreSpotlightError::bridge(-1, format!("failed to encode {context} as JSON: {error}"))
    })?;
    cstring_from_str(&json, context)
}

pub(crate) unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::cs_string_free(ptr);
    Some(string)
}

pub(crate) fn parse_json_str<T: DeserializeOwned>(
    json: &str,
    context: &str,
) -> Result<T, CoreSpotlightError> {
    serde_json::from_str(json).map_err(|error| {
        CoreSpotlightError::bridge(
            -1,
            format!("failed to parse {context} JSON: {error}; payload={json}"),
        )
    })
}

pub(crate) unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, CoreSpotlightError> {
    let json = take_string(ptr).ok_or_else(|| {
        CoreSpotlightError::bridge(-1, format!("missing JSON payload for {context}"))
    })?;
    parse_json_str(&json, context)
}

pub(crate) unsafe fn parse_error_ptr(ptr: *mut c_char) -> CoreSpotlightError {
    if ptr.is_null() {
        return CoreSpotlightError::bridge(
            -2,
            "Core Spotlight bridge returned an error without payload",
        );
    }
    let json = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::cs_string_free(ptr);
    match serde_json::from_str::<ErrorPayload>(&json) {
        Ok(payload) => CoreSpotlightError::from_payload(payload),
        Err(error) => CoreSpotlightError::bridge(
            -1,
            format!("failed to parse Core Spotlight error payload: {error}; payload={json}"),
        ),
    }
}

pub(crate) unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> CoreSpotlightError {
    if !err_msg.is_null() {
        return parse_error_ptr(err_msg);
    }
    let message = match status {
        ffi::status::INVALID_ARGUMENT => "invalid argument",
        ffi::status::TIMED_OUT => "timed out waiting for Core Spotlight",
        _ => "Core Spotlight bridge failure",
    };
    CoreSpotlightError::bridge(i64::from(status), message)
}

pub(crate) fn system_time_to_unix_seconds(time: SystemTime) -> Result<f64, CoreSpotlightError> {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .map_err(|error| {
            CoreSpotlightError::bridge(-1, format!("time predates the unix epoch: {error}"))
        })
}

pub(crate) fn system_time_from_unix_seconds(seconds: f64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs_f64(seconds)
}
