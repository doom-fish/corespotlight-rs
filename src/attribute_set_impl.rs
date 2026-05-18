use std::collections::BTreeMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::attribute_set_fields::{
    AttributeDataField, AttributeDateArrayField, AttributeDateField, AttributeNumberField,
    AttributePersonArrayField, AttributeReadOnlyNumberField, AttributeReadOnlyStringField,
    AttributeStringArrayField, AttributeStringArrayMapField, AttributeStringField,
    AttributeUrlField,
};
use crate::error::CoreSpotlightError;
use crate::ffi;
use crate::private::{
    cstring_from_str, error_from_status, impl_object_wrapper, json_cstring, opt_cstring_ptr,
    optional_cstring_from_str, parse_json_ptr, system_time_from_unix_seconds,
    system_time_to_unix_seconds, take_string,
};

pub use crate::attribute_set_fields::{
    AttributeDataField as CSSearchableItemAttributeDataField,
    AttributeDateArrayField as CSSearchableItemAttributeDateArrayField,
    AttributeDateField as CSSearchableItemAttributeDateField,
    AttributeNumberField as CSSearchableItemAttributeNumberField,
    AttributePersonArrayField as CSSearchableItemAttributePersonArrayField,
    AttributeReadOnlyNumberField as CSSearchableItemAttributeReadOnlyNumberField,
    AttributeReadOnlyStringField as CSSearchableItemAttributeReadOnlyStringField,
    AttributeStringArrayField as CSSearchableItemAttributeStringArrayField,
    AttributeStringArrayMapField as CSSearchableItemAttributeStringArrayMapField,
    AttributeStringField as CSSearchableItemAttributeStringField,
    AttributeUrlField as CSSearchableItemAttributeURLField,
};

impl_object_wrapper!(CSSearchableItemAttributeSet);
impl_object_wrapper!(CSLocalizedString);
impl_object_wrapper!(CSCustomAttributeKey);
impl_object_wrapper!(CSPerson);
impl_object_wrapper!(NSUserActivity);

/// Serializable representation of the data exposed by `CSPerson`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSPersonData {
    /// Mirrors `CSPerson.displayName`.
    pub display_name: Option<String>,
    /// Mirrors the handle values exposed by `CSPerson`.
    pub handles: Vec<String>,
    /// Mirrors `CSPerson.handleIdentifier`.
    pub handle_identifier: String,
    /// Mirrors `CSPerson.contactIdentifier`.
    pub contact_identifier: Option<String>,
}

/// Rust representation of values accepted by `CSSearchableItemAttributeSet` custom attributes.
#[derive(Debug, Clone, PartialEq)]
pub enum CustomAttributeValue {
    /// Wraps string custom-attribute values.
    String(String),
    /// Wraps numeric custom-attribute values.
    Number(f64),
    /// Wraps Boolean custom-attribute values.
    Boolean(bool),
    /// Wraps binary custom-attribute values.
    Bytes(Vec<u8>),
    /// Wraps date custom-attribute values.
    Date(SystemTime),
    /// Wraps `CSPerson` custom-attribute values.
    Person(CSPersonData),
    /// Wraps array custom-attribute values.
    Array(Vec<CustomAttributeValue>),
    /// Wraps null custom-attribute values.
    Null,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
enum CustomAttributePayload {
    String(String),
    Number(f64),
    Boolean(bool),
    Bytes(Vec<u8>),
    Date(f64),
    Person(CSPersonData),
    Array(Vec<CustomAttributePayload>),
    Null,
}

impl CustomAttributeValue {
    fn into_payload(self) -> Result<CustomAttributePayload, CoreSpotlightError> {
        match self {
            Self::String(value) => Ok(CustomAttributePayload::String(value)),
            Self::Number(value) => Ok(CustomAttributePayload::Number(value)),
            Self::Boolean(value) => Ok(CustomAttributePayload::Boolean(value)),
            Self::Bytes(value) => Ok(CustomAttributePayload::Bytes(value)),
            Self::Date(value) => Ok(CustomAttributePayload::Date(system_time_to_unix_seconds(
                value,
            )?)),
            Self::Person(value) => Ok(CustomAttributePayload::Person(value)),
            Self::Array(values) => values
                .into_iter()
                .map(Self::into_payload)
                .collect::<Result<Vec<_>, _>>()
                .map(CustomAttributePayload::Array),
            Self::Null => Ok(CustomAttributePayload::Null),
        }
    }

    fn from_payload(payload: CustomAttributePayload) -> Self {
        match payload {
            CustomAttributePayload::String(value) => Self::String(value),
            CustomAttributePayload::Number(value) => Self::Number(value),
            CustomAttributePayload::Boolean(value) => Self::Boolean(value),
            CustomAttributePayload::Bytes(value) => Self::Bytes(value),
            CustomAttributePayload::Date(value) => Self::Date(system_time_from_unix_seconds(value)),
            CustomAttributePayload::Person(value) => Self::Person(value),
            CustomAttributePayload::Array(values) => {
                Self::Array(values.into_iter().map(Self::from_payload).collect())
            }
            CustomAttributePayload::Null => Self::Null,
        }
    }
}

fn string_getter(object_ptr: *mut core::ffi::c_void, field_name: &str) -> Option<String> {
    let field_name = cstring_from_str(field_name, "attribute field name").ok()?;
    unsafe {
        take_string(ffi::cs_attribute_set_get_string(
            object_ptr,
            field_name.as_ptr(),
        ))
    }
}

fn set_string_field(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    value: Option<&str>,
) -> Result<(), CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let value = optional_cstring_from_str(value, field_name.to_str().unwrap_or("attribute value"))?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_string(
            object_ptr,
            field_name.as_ptr(),
            opt_cstring_ptr(value.as_ref()),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn string_array_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    context: &str,
) -> Result<Vec<String>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_get_string_array(
            object_ptr,
            field_name.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}

fn set_string_array_field<I, S>(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    values: I,
    context: &str,
) -> Result<(), CoreSpotlightError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let values = values.into_iter().map(Into::into).collect::<Vec<_>>();
    let values_json = json_cstring(&values, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_string_array(
            object_ptr,
            field_name.as_ptr(),
            values_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn number_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
) -> Result<Option<f64>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut value = 0.0;
    let has_value =
        unsafe { ffi::cs_attribute_set_get_number(object_ptr, field_name.as_ptr(), &mut value) };
    if has_value == 0 {
        return Ok(None);
    }
    Ok(Some(value))
}

fn set_number_field(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    value: Option<f64>,
) -> Result<(), CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_number(
            object_ptr,
            field_name.as_ptr(),
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

fn url_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
) -> Result<Option<String>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    Ok(unsafe {
        take_string(ffi::cs_attribute_set_get_url(
            object_ptr,
            field_name.as_ptr(),
        ))
    })
}

fn set_url_field(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    value: Option<&str>,
) -> Result<(), CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let value = optional_cstring_from_str(value, "attribute URL value")?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_url(
            object_ptr,
            field_name.as_ptr(),
            opt_cstring_ptr(value.as_ref()),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn data_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    context: &str,
) -> Result<Vec<u8>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_get_data(
            object_ptr,
            field_name.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}

fn set_data_field(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    value: &[u8],
    context: &str,
) -> Result<(), CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let value_json = json_cstring(value, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_data(
            object_ptr,
            field_name.as_ptr(),
            value_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn date_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
) -> Result<Option<SystemTime>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut value = 0.0;
    let has_value =
        unsafe { ffi::cs_attribute_set_get_date(object_ptr, field_name.as_ptr(), &mut value) };
    if has_value == 0 {
        return Ok(None);
    }
    Ok(Some(system_time_from_unix_seconds(value)))
}

fn set_date_field(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    value: Option<SystemTime>,
) -> Result<(), CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let unix_seconds = value
        .map(system_time_to_unix_seconds)
        .transpose()?
        .unwrap_or_default();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_date(
            object_ptr,
            field_name.as_ptr(),
            unix_seconds,
            i32::from(value.is_some()),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn date_array_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    context: &str,
) -> Result<Vec<SystemTime>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_get_date_array(
            object_ptr,
            field_name.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    let values: Vec<f64> = unsafe { parse_json_ptr(out_json, context)? };
    Ok(values
        .into_iter()
        .map(system_time_from_unix_seconds)
        .collect())
}

fn set_date_array_field<I>(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    values: I,
    context: &str,
) -> Result<(), CoreSpotlightError>
where
    I: IntoIterator<Item = SystemTime>,
{
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let values = values
        .into_iter()
        .map(system_time_to_unix_seconds)
        .collect::<Result<Vec<_>, _>>()?;
    let values_json = json_cstring(&values, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_date_array(
            object_ptr,
            field_name.as_ptr(),
            values_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn person_array_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    context: &str,
) -> Result<Vec<CSPersonData>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_get_person_array(
            object_ptr,
            field_name.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}

fn set_person_array_field<I>(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    values: I,
    context: &str,
) -> Result<(), CoreSpotlightError>
where
    I: IntoIterator<Item = CSPersonData>,
{
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let values = values.into_iter().collect::<Vec<_>>();
    let values_json = json_cstring(&values, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_person_array(
            object_ptr,
            field_name.as_ptr(),
            values_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

fn string_array_map_getter(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    context: &str,
) -> Result<BTreeMap<String, Vec<String>>, CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_get_string_array_map(
            object_ptr,
            field_name.as_ptr(),
            &mut out_json,
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}

fn set_string_array_map_field(
    object_ptr: *mut core::ffi::c_void,
    field_name: &str,
    values: &BTreeMap<String, Vec<String>>,
    context: &str,
) -> Result<(), CoreSpotlightError> {
    let field_name = cstring_from_str(field_name, "attribute field name")?;
    let values_json = json_cstring(values, context)?;
    let mut out_error = core::ptr::null_mut();
    let status = unsafe {
        ffi::cs_attribute_set_set_string_array_map(
            object_ptr,
            field_name.as_ptr(),
            values_json.as_ptr(),
            &mut out_error,
        )
    };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    Ok(())
}

impl CSSearchableItemAttributeSet {
    /// Wraps the `CSSearchableItemAttributeSet` initializer.
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

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn string(&self, field: AttributeStringField) -> Option<String> {
        string_getter(self.as_ptr(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_string(
        &self,
        field: AttributeStringField,
        value: Option<&str>,
    ) -> Result<(), CoreSpotlightError> {
        set_string_field(self.as_ptr(), field.as_str(), value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn read_only_string(&self, field: AttributeReadOnlyStringField) -> Option<String> {
        string_getter(self.as_ptr(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn string_array(
        &self,
        field: AttributeStringArrayField,
    ) -> Result<Vec<String>, CoreSpotlightError> {
        string_array_getter(self.as_ptr(), field.as_str(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_string_array<I, S>(
        &self,
        field: AttributeStringArrayField,
        values: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        set_string_array_field(self.as_ptr(), field.as_str(), values, field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn number(&self, field: AttributeNumberField) -> Result<Option<f64>, CoreSpotlightError> {
        number_getter(self.as_ptr(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_number(
        &self,
        field: AttributeNumberField,
        value: Option<f64>,
    ) -> Result<(), CoreSpotlightError> {
        set_number_field(self.as_ptr(), field.as_str(), value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn read_only_number(
        &self,
        field: AttributeReadOnlyNumberField,
    ) -> Result<Option<f64>, CoreSpotlightError> {
        number_getter(self.as_ptr(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn url(&self, field: AttributeUrlField) -> Result<Option<String>, CoreSpotlightError> {
        url_getter(self.as_ptr(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_url(
        &self,
        field: AttributeUrlField,
        value: Option<&str>,
    ) -> Result<(), CoreSpotlightError> {
        set_url_field(self.as_ptr(), field.as_str(), value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn data(&self, field: AttributeDataField) -> Result<Vec<u8>, CoreSpotlightError> {
        data_getter(self.as_ptr(), field.as_str(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_data(
        &self,
        field: AttributeDataField,
        value: &[u8],
    ) -> Result<(), CoreSpotlightError> {
        set_data_field(self.as_ptr(), field.as_str(), value, field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn date(
        &self,
        field: AttributeDateField,
    ) -> Result<Option<SystemTime>, CoreSpotlightError> {
        date_getter(self.as_ptr(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_date(
        &self,
        field: AttributeDateField,
        value: Option<SystemTime>,
    ) -> Result<(), CoreSpotlightError> {
        set_date_field(self.as_ptr(), field.as_str(), value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn date_array(
        &self,
        field: AttributeDateArrayField,
    ) -> Result<Vec<SystemTime>, CoreSpotlightError> {
        date_array_getter(self.as_ptr(), field.as_str(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_date_array<I>(
        &self,
        field: AttributeDateArrayField,
        values: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = SystemTime>,
    {
        set_date_array_field(self.as_ptr(), field.as_str(), values, field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn person_array(
        &self,
        field: AttributePersonArrayField,
    ) -> Result<Vec<CSPersonData>, CoreSpotlightError> {
        person_array_getter(self.as_ptr(), field.as_str(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_person_array<I>(
        &self,
        field: AttributePersonArrayField,
        values: I,
    ) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = CSPersonData>,
    {
        set_person_array_field(self.as_ptr(), field.as_str(), values, field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn string_array_map(
        &self,
        field: AttributeStringArrayMapField,
    ) -> Result<BTreeMap<String, Vec<String>>, CoreSpotlightError> {
        string_array_map_getter(self.as_ptr(), field.as_str(), field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_string_array_map(
        &self,
        field: AttributeStringArrayMapField,
        values: &BTreeMap<String, Vec<String>>,
    ) -> Result<(), CoreSpotlightError> {
        set_string_array_map_field(self.as_ptr(), field.as_str(), values, field.as_str())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_localized_string(
        &self,
        field: AttributeStringField,
        value: &CSLocalizedString,
    ) -> Result<(), CoreSpotlightError> {
        let field_name = cstring_from_str(field.as_str(), "attribute field name")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_set_localized_string(
                self.as_ptr(),
                field_name.as_ptr(),
                value.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` operation.
    pub fn move_from(&self, source: Self) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_move_from(self.as_ptr(), source.as_ptr(), &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_custom_value(
        &self,
        key: &CSCustomAttributeKey,
        value: CustomAttributeValue,
    ) -> Result<(), CoreSpotlightError> {
        let payload = value.into_payload()?;
        let payload_json = json_cstring(&payload, "custom attribute value")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_set_custom_value(
                self.as_ptr(),
                key.as_ptr(),
                payload_json.as_ptr(),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn custom_value(
        &self,
        key: &CSCustomAttributeKey,
    ) -> Result<CustomAttributeValue, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_attribute_set_get_custom_value(
                self.as_ptr(),
                key.as_ptr(),
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: CustomAttributePayload =
            unsafe { parse_json_ptr(out_json, "custom attribute")? };
        Ok(CustomAttributeValue::from_payload(payload))
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn item_content_type(&self) -> Option<String> {
        self.string(AttributeStringField::ContentType)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_item_content_type(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        self.set_string(AttributeStringField::ContentType, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn title(&self) -> Option<String> {
        self.string(AttributeStringField::Title)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_title(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        self.set_string(AttributeStringField::Title, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn content_description(&self) -> Option<String> {
        self.string(AttributeStringField::ContentDescription)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_content_description(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        self.set_string(AttributeStringField::ContentDescription, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn display_name(&self) -> Option<String> {
        self.string(AttributeStringField::DisplayName)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_display_name(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        self.set_string(AttributeStringField::DisplayName, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn keywords(&self) -> Result<Vec<String>, CoreSpotlightError> {
        self.string_array(AttributeStringArrayField::Keywords)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_keywords<I, S>(&self, values: I) -> Result<(), CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set_string_array(AttributeStringArrayField::Keywords, values)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn thumbnail_data(&self) -> Result<Vec<u8>, CoreSpotlightError> {
        self.data(AttributeDataField::ThumbnailData)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_thumbnail_data(&self, data: &[u8]) -> Result<(), CoreSpotlightError> {
        self.set_data(AttributeDataField::ThumbnailData, data)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn thumbnail_url(&self) -> Option<String> {
        self.url(AttributeUrlField::ThumbnailURL).ok().flatten()
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_thumbnail_url(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        self.set_url(AttributeUrlField::ThumbnailURL, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn content_url(&self) -> Option<String> {
        self.url(AttributeUrlField::ContentURL).ok().flatten()
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_content_url(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        self.set_url(AttributeUrlField::ContentURL, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn latitude(&self) -> Option<f64> {
        self.number(AttributeNumberField::Latitude).ok().flatten()
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_latitude(&self, value: Option<f64>) -> Result<(), CoreSpotlightError> {
        self.set_number(AttributeNumberField::Latitude, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn longitude(&self) -> Option<f64> {
        self.number(AttributeNumberField::Longitude).ok().flatten()
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_longitude(&self, value: Option<f64>) -> Result<(), CoreSpotlightError> {
        self.set_number(AttributeNumberField::Longitude, value)
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` getter.
    pub fn rating(&self) -> Option<f64> {
        self.number(AttributeNumberField::Rating).ok().flatten()
    }

    /// Wraps the corresponding `CSSearchableItemAttributeSet` setter.
    pub fn set_rating(&self, value: Option<f64>) -> Result<(), CoreSpotlightError> {
        self.set_number(AttributeNumberField::Rating, value)
    }
}

impl CSLocalizedString {
    /// Wraps the `CSLocalizedString` initializer.
    pub fn new(localized_strings: &BTreeMap<String, String>) -> Result<Self, CoreSpotlightError> {
        let localized_strings_json = json_cstring(localized_strings, "localized strings")?;
        let mut out_value = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_localized_string_new(
                localized_strings_json.as_ptr(),
                &mut out_value,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_value, "localized string") }
    }

    /// Wraps the corresponding `CSLocalizedString` getter.
    pub fn localized_string(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_localized_string_get_localized_string(self.as_ptr())) }
    }
}

impl CSCustomAttributeKey {
    /// Wraps the `CSCustomAttributeKey` initializer.
    pub fn new(key_name: impl AsRef<str>) -> Result<Self, CoreSpotlightError> {
        let key_name = cstring_from_str(key_name.as_ref(), "custom attribute key name")?;
        let mut out_key = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_custom_attribute_key_new(key_name.as_ptr(), &mut out_key, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_key, "custom attribute key") }
    }

    #[allow(clippy::fn_params_excessive_bools)]
    /// Wraps a convenience initializer for `CSCustomAttributeKey`.
    pub fn new_with_options(
        key_name: impl AsRef<str>,
        searchable: bool,
        searchable_by_default: bool,
        unique: bool,
        multi_valued: bool,
    ) -> Result<Self, CoreSpotlightError> {
        let key_name = cstring_from_str(key_name.as_ref(), "custom attribute key name")?;
        let mut out_key = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_custom_attribute_key_new_with_options(
                key_name.as_ptr(),
                i32::from(searchable),
                i32::from(searchable_by_default),
                i32::from(unique),
                i32::from(multi_valued),
                &mut out_key,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_key, "custom attribute key") }
    }

    /// Wraps the corresponding `CSCustomAttributeKey` getter.
    pub fn key_name(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_custom_attribute_key_get_key_name(self.as_ptr())) }
    }

    /// Wraps the corresponding `CSCustomAttributeKey` getter.
    pub fn is_searchable(&self) -> bool {
        unsafe { ffi::cs_custom_attribute_key_is_searchable(self.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `CSCustomAttributeKey` getter.
    pub fn is_searchable_by_default(&self) -> bool {
        unsafe { ffi::cs_custom_attribute_key_is_searchable_by_default(self.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `CSCustomAttributeKey` getter.
    pub fn is_unique(&self) -> bool {
        unsafe { ffi::cs_custom_attribute_key_is_unique(self.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `CSCustomAttributeKey` getter.
    pub fn is_multi_valued(&self) -> bool {
        unsafe { ffi::cs_custom_attribute_key_is_multi_valued(self.as_ptr()) != 0 }
    }
}

impl CSPerson {
    /// Wraps the `CSPerson` initializer.
    pub fn new<I, S>(
        display_name: Option<&str>,
        handles: I,
        handle_identifier: impl AsRef<str>,
    ) -> Result<Self, CoreSpotlightError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let display_name = optional_cstring_from_str(display_name, "person display name")?;
        let handles = handles.into_iter().map(Into::into).collect::<Vec<_>>();
        let handles_json = json_cstring(&handles, "person handles")?;
        let handle_identifier =
            cstring_from_str(handle_identifier.as_ref(), "person handle identifier")?;
        let mut out_person = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_person_new(
                opt_cstring_ptr(display_name.as_ref()),
                handles_json.as_ptr(),
                handle_identifier.as_ptr(),
                &mut out_person,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_person, "person") }
    }

    /// Wraps the corresponding `CSPerson` getter.
    pub fn display_name(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_person_get_display_name(self.as_ptr())) }
    }

    /// Wraps the corresponding `CSPerson` getter.
    pub fn handles(&self) -> Result<Vec<String>, CoreSpotlightError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status =
            unsafe { ffi::cs_person_get_handles(self.as_ptr(), &mut out_json, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { parse_json_ptr(out_json, "person handles") }
    }

    /// Wraps the corresponding `CSPerson` getter.
    pub fn handle_identifier(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_person_get_handle_identifier(self.as_ptr())) }
    }

    /// Wraps the corresponding `CSPerson` getter.
    pub fn contact_identifier(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_person_get_contact_identifier(self.as_ptr())) }
    }

    /// Wraps the corresponding `CSPerson` setter.
    pub fn set_contact_identifier(&self, value: Option<&str>) -> Result<(), CoreSpotlightError> {
        let value = optional_cstring_from_str(value, "person contact identifier")?;
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_person_set_contact_identifier(
                self.as_ptr(),
                opt_cstring_ptr(value.as_ref()),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }

    /// Wraps the corresponding `CSPerson` getter.
    pub fn to_data(&self) -> Result<CSPersonData, CoreSpotlightError> {
        Ok(CSPersonData {
            display_name: self.display_name(),
            handles: self.handles()?,
            handle_identifier: self.handle_identifier().unwrap_or_default(),
            contact_identifier: self.contact_identifier(),
        })
    }
}

impl NSUserActivity {
    /// Wraps the `NSUserActivity` initializer.
    pub fn new(activity_type: impl AsRef<str>) -> Result<Self, CoreSpotlightError> {
        let activity_type = cstring_from_str(activity_type.as_ref(), "user activity type")?;
        let mut out_activity = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_activity_new(activity_type.as_ptr(), &mut out_activity, &mut out_error)
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        unsafe { Self::from_retained_ptr(out_activity, "user activity") }
    }

    /// Wraps the corresponding `NSUserActivity` getter.
    pub fn activity_type(&self) -> Option<String> {
        unsafe { take_string(ffi::cs_user_activity_get_activity_type(self.as_ptr())) }
    }

    /// Wraps the corresponding `NSUserActivity` getter.
    pub fn content_attribute_set(&self) -> Option<CSSearchableItemAttributeSet> {
        let attribute_set =
            unsafe { ffi::cs_user_activity_get_content_attribute_set(self.as_ptr()) };
        unsafe {
            CSSearchableItemAttributeSet::from_retained_ptr(
                attribute_set,
                "user activity content attribute set",
            )
            .ok()
        }
    }

    /// Wraps the corresponding `NSUserActivity` setter.
    pub fn set_content_attribute_set(
        &self,
        value: Option<&CSSearchableItemAttributeSet>,
    ) -> Result<(), CoreSpotlightError> {
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::cs_user_activity_set_content_attribute_set(
                self.as_ptr(),
                value.map_or(core::ptr::null_mut(), CSSearchableItemAttributeSet::as_ptr),
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        Ok(())
    }
}
