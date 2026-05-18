//! Wrappers for `CSImportExtension`.

use core::ffi::{c_char, c_void};
use std::ffi::CStr;

use crate::attribute_set::CSSearchableItemAttributeSet;
use crate::error::{CoreSpotlightError, ErrorPayload};
use crate::ffi;
use crate::private::{cstring_from_str, error_from_status, impl_object_wrapper};

impl_object_wrapper!(CSImportExtension);

unsafe extern "C" {
    fn strdup(value: *const c_char) -> *mut c_char;
}

type UpdateFn = dyn Fn(CSSearchableItemAttributeSet, &str) -> Result<(), CoreSpotlightError>
    + Send
    + Sync
    + 'static;

struct ImportExtensionState {
    update: Box<UpdateFn>,
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
        if let Ok(ptr) = duplicate_c_string(&json, "import extension error payload") {
            unsafe {
                *out_error = ptr;
            }
        }
    }
}

unsafe fn state_from_context<'a>(context: *mut c_void) -> &'a ImportExtensionState {
    &*context.cast::<ImportExtensionState>()
}

pub(crate) unsafe extern "C" fn release_import_extension_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    drop(Box::from_raw(context.cast::<ImportExtensionState>()));
}

pub(crate) unsafe extern "C" fn import_extension_update(
    context: *mut c_void,
    attributes_ptr: *mut c_void,
    content_url: *const c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    let result = (|| -> Result<(), CoreSpotlightError> {
        if context.is_null() || attributes_ptr.is_null() || content_url.is_null() {
            return Err(CoreSpotlightError::bridge(
                i64::from(ffi::status::INVALID_ARGUMENT),
                "missing import extension callback arguments",
            ));
        }
        let state = state_from_context(context);
        let attributes = unsafe {
            CSSearchableItemAttributeSet::from_retained_ptr(
                attributes_ptr,
                "import extension attribute set",
            )
        }?;
        let content_url = unsafe { CStr::from_ptr(content_url) }
            .to_str()
            .map_err(|error| {
                CoreSpotlightError::bridge(
                    -1,
                    format!("invalid import extension content URL utf-8: {error}"),
                )
            })?;
        (state.update)(attributes, content_url)
    })();

    match result {
        Ok(()) => ffi::status::OK,
        Err(error) => {
            write_error_payload(&error, out_error);
            ffi::status::FAILURE
        }
    }
}

impl CSImportExtension {
    /// Wraps the `CSImportExtension` initializer.
    pub fn new<F>(update: F) -> Result<Self, CoreSpotlightError>
    where
        F: Fn(CSSearchableItemAttributeSet, &str) -> Result<(), CoreSpotlightError>
            + Send
            + Sync
            + 'static,
    {
        let state = Box::new(ImportExtensionState {
            update: Box::new(update),
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let mut out_extension = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_import_extension_new(
                context,
                Some(release_import_extension_context),
                Some(import_extension_update),
                &mut out_extension,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            unsafe {
                release_import_extension_context(context);
            }
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_extension, "import extension") }
    }

    /// Wraps the corresponding `CSImportExtension` operation.
    pub fn simulate_update(
        &self,
        attributes: &CSSearchableItemAttributeSet,
        content_url: impl AsRef<str>,
    ) -> Result<(), CoreSpotlightError> {
        let content_url = cstring_from_str(content_url.as_ref(), "import extension content URL")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_import_extension_simulate_update(
                self.as_ptr(),
                attributes.as_ptr(),
                content_url.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
