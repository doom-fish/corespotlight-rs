#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cs_string_free(s: *mut c_char);
    pub fn cs_retain_object(ptr: *mut c_void) -> *mut c_void;
    pub fn cs_release_object(ptr: *mut c_void);

    pub fn cs_searchable_index_default_searchable_index(
        out_index: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_new(
        name: *const c_char,
        out_index: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_index_items(
        index: *mut c_void,
        items_json: *const c_char,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delete_identifiers(
        index: *mut c_void,
        identifiers_json: *const c_char,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delete_domain_identifiers(
        index: *mut c_void,
        identifiers_json: *const c_char,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delete_all(
        index: *mut c_void,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_searchable_item_new(
        unique_identifier: *const c_char,
        domain_identifier: *const c_char,
        attribute_set: *mut c_void,
        out_item: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_set_expiration_date(
        item: *mut c_void,
        unix_seconds: f64,
        has_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_get_expiration_date(
        item: *mut c_void,
        out_unix_seconds: *mut f64,
    ) -> i32;

    pub fn cs_attribute_set_new(
        item_content_type: *const c_char,
        out_attribute_set: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_item_content_type(attribute_set: *mut c_void) -> *mut c_char;
    pub fn cs_attribute_set_set_item_content_type(
        attribute_set: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_title(attribute_set: *mut c_void) -> *mut c_char;
    pub fn cs_attribute_set_set_title(
        attribute_set: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_content_description(attribute_set: *mut c_void) -> *mut c_char;
    pub fn cs_attribute_set_set_content_description(
        attribute_set: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_display_name(attribute_set: *mut c_void) -> *mut c_char;
    pub fn cs_attribute_set_set_display_name(
        attribute_set: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_keywords(
        attribute_set: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_keywords(
        attribute_set: *mut c_void,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_thumbnail_data(
        attribute_set: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_thumbnail_data(
        attribute_set: *mut c_void,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_thumbnail_url(attribute_set: *mut c_void) -> *mut c_char;
    pub fn cs_attribute_set_set_thumbnail_url(
        attribute_set: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_content_url(attribute_set: *mut c_void) -> *mut c_char;
    pub fn cs_attribute_set_set_content_url(
        attribute_set: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_latitude(
        attribute_set: *mut c_void,
        out_value: *mut f64,
    ) -> i32;
    pub fn cs_attribute_set_set_latitude(
        attribute_set: *mut c_void,
        value: f64,
        has_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_longitude(
        attribute_set: *mut c_void,
        out_value: *mut f64,
    ) -> i32;
    pub fn cs_attribute_set_set_longitude(
        attribute_set: *mut c_void,
        value: f64,
        has_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_rating(
        attribute_set: *mut c_void,
        out_value: *mut f64,
    ) -> i32;
    pub fn cs_attribute_set_set_rating(
        attribute_set: *mut c_void,
        value: f64,
        has_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FAILURE: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
}
