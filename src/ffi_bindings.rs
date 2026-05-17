use core::ffi::{c_char, c_void};

pub type CsDelegateReleaseContext = unsafe extern "C" fn(context: *mut c_void);
pub type CsDelegateReindexAll = unsafe extern "C" fn(context: *mut c_void, index: *mut c_void);
pub type CsDelegateReindexIdentifiers =
    unsafe extern "C" fn(context: *mut c_void, index: *mut c_void, identifiers_json: *const c_char);
pub type CsDelegateNotification = unsafe extern "C" fn(context: *mut c_void, index: *mut c_void);
pub type CsDelegateDataForItem = unsafe extern "C" fn(
    context: *mut c_void,
    index: *mut c_void,
    item_identifier: *const c_char,
    type_identifier: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32;
pub type CsDelegateFileUrlForItem = unsafe extern "C" fn(
    context: *mut c_void,
    index: *mut c_void,
    item_identifier: *const c_char,
    type_identifier: *const c_char,
    in_place: i32,
    out_url: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32;
pub type CsDelegateSearchableItemsForIdentifiers = unsafe extern "C" fn(
    context: *mut c_void,
    identifiers_json: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32;
pub type CsDelegateSearchableItemsDidUpdate =
    unsafe extern "C" fn(context: *mut c_void, items_json: *const c_char);
pub type CsImportExtensionUpdate = unsafe extern "C" fn(
    context: *mut c_void,
    attributes: *mut c_void,
    content_url: *const c_char,
    out_error: *mut *mut c_char,
) -> i32;

extern "C" {
    pub fn cs_string_free(s: *mut c_char);
    pub fn cs_retain_object(ptr: *mut c_void) -> *mut c_void;
    pub fn cs_release_object(ptr: *mut c_void);

    pub fn cs_searchable_index_is_indexing_available() -> i32;
    pub fn cs_searchable_index_default_searchable_index(
        out_index: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_new(
        name: *const c_char,
        out_index: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_new_with_protection_class(
        name: *const c_char,
        protection_class: *const c_char,
        out_index: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_set_delegate(
        index: *mut c_void,
        delegate: *mut c_void,
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
    pub fn cs_searchable_index_begin_batch(index: *mut c_void, out_error: *mut *mut c_char) -> i32;
    pub fn cs_searchable_index_end_batch_with_client_state(
        index: *mut c_void,
        client_state_json: *const c_char,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_end_batch_with_expected_client_state(
        index: *mut c_void,
        expected_client_state_json: *const c_char,
        new_client_state_json: *const c_char,
        timeout_seconds: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_fetch_last_client_state(
        index: *mut c_void,
        timeout_seconds: i32,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_fetch_data_for_bundle_identifier(
        index: *mut c_void,
        bundle_identifier: *const c_char,
        item_identifier: *const c_char,
        content_type: *const c_char,
        timeout_seconds: i32,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_searchable_item_new(
        unique_identifier: *const c_char,
        domain_identifier: *const c_char,
        attribute_set: *mut c_void,
        out_item: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_compare_by_rank(item: *mut c_void, other: *mut c_void) -> i32;
    pub fn cs_searchable_item_get_unique_identifier(item: *mut c_void) -> *mut c_char;
    pub fn cs_searchable_item_set_unique_identifier(
        item: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_get_domain_identifier(item: *mut c_void) -> *mut c_char;
    pub fn cs_searchable_item_set_domain_identifier(
        item: *mut c_void,
        value: *const c_char,
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
    pub fn cs_searchable_item_get_attribute_set(
        item: *mut c_void,
        out_attribute_set: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_set_attribute_set(
        item: *mut c_void,
        attribute_set: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_get_is_update(item: *mut c_void) -> i32;
    pub fn cs_searchable_item_set_is_update(item: *mut c_void, value: i32);
    pub fn cs_searchable_item_get_update_listener_options(
        item: *mut c_void,
        out_value: *mut u64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_item_set_update_listener_options(
        item: *mut c_void,
        value: u64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_attribute_set_new(
        item_content_type: *const c_char,
        out_attribute_set: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_string(
        attribute_set: *mut c_void,
        field_name: *const c_char,
    ) -> *mut c_char;
    pub fn cs_attribute_set_set_string(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_string_array(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_string_array(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_number(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_value: *mut f64,
    ) -> i32;
    pub fn cs_attribute_set_set_number(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        value: f64,
        has_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_url(
        attribute_set: *mut c_void,
        field_name: *const c_char,
    ) -> *mut c_char;
    pub fn cs_attribute_set_set_url(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_data(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_data(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_date(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_value: *mut f64,
    ) -> i32;
    pub fn cs_attribute_set_set_date(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        value: f64,
        has_value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_date_array(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_date_array(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_person_array(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_person_array(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_string_array_map(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_string_array_map(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_localized_string(
        attribute_set: *mut c_void,
        field_name: *const c_char,
        localized_string: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_move_from(
        attribute_set: *mut c_void,
        source_attribute_set: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_set_custom_value(
        attribute_set: *mut c_void,
        key: *mut c_void,
        value_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_attribute_set_get_custom_value(
        attribute_set: *mut c_void,
        key: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_user_activity_new(
        activity_type: *const c_char,
        out_activity: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_activity_get_activity_type(activity: *mut c_void) -> *mut c_char;
    pub fn cs_user_activity_get_content_attribute_set(activity: *mut c_void) -> *mut c_void;
    pub fn cs_user_activity_set_content_attribute_set(
        activity: *mut c_void,
        attribute_set: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_localized_string_new(
        localized_strings_json: *const c_char,
        out_value: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_localized_string_get_localized_string(value: *mut c_void) -> *mut c_char;

    pub fn cs_custom_attribute_key_new(
        key_name: *const c_char,
        out_key: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_custom_attribute_key_new_with_options(
        key_name: *const c_char,
        searchable: i32,
        searchable_by_default: i32,
        unique: i32,
        multi_valued: i32,
        out_key: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_custom_attribute_key_get_key_name(key: *mut c_void) -> *mut c_char;
    pub fn cs_custom_attribute_key_is_searchable(key: *mut c_void) -> i32;
    pub fn cs_custom_attribute_key_is_searchable_by_default(key: *mut c_void) -> i32;
    pub fn cs_custom_attribute_key_is_unique(key: *mut c_void) -> i32;
    pub fn cs_custom_attribute_key_is_multi_valued(key: *mut c_void) -> i32;

    pub fn cs_person_new(
        display_name: *const c_char,
        handles_json: *const c_char,
        handle_identifier: *const c_char,
        out_person: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_person_get_display_name(person: *mut c_void) -> *mut c_char;
    pub fn cs_person_get_handles(
        person: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_person_get_handle_identifier(person: *mut c_void) -> *mut c_char;
    pub fn cs_person_get_contact_identifier(person: *mut c_void) -> *mut c_char;
    pub fn cs_person_set_contact_identifier(
        person: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_core_spotlight_version_number() -> f64;
    pub fn cs_core_spotlight_version_string() -> *mut c_char;
    pub fn cs_core_spotlight_api_version() -> i32;
    pub fn cs_index_error_domain() -> *mut c_char;
    pub fn cs_search_query_error_domain() -> *mut c_char;
    pub fn cs_searchable_item_action_type() -> *mut c_char;
    pub fn cs_searchable_item_activity_identifier() -> *mut c_char;
    pub fn cs_query_continuation_action_type() -> *mut c_char;
    pub fn cs_search_query_string_key() -> *mut c_char;
    pub fn cs_suggestion_highlight_attribute_name() -> *mut c_char;
    pub fn cs_mailbox_inbox() -> *mut c_char;
    pub fn cs_mailbox_drafts() -> *mut c_char;
    pub fn cs_mailbox_sent() -> *mut c_char;
    pub fn cs_mailbox_junk() -> *mut c_char;
    pub fn cs_mailbox_trash() -> *mut c_char;
    pub fn cs_mailbox_archive() -> *mut c_char;

    pub fn cs_search_query_context_new(
        out_context: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_context_get_fetch_attributes(
        context: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_context_set_fetch_attributes(
        context: *mut c_void,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_context_get_filter_queries(
        context: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_context_set_filter_queries(
        context: *mut c_void,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_context_get_keyboard_language(context: *mut c_void) -> *mut c_char;
    pub fn cs_search_query_context_set_keyboard_language(
        context: *mut c_void,
        value: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_context_get_source_options(context: *mut c_void) -> u64;
    pub fn cs_search_query_context_set_source_options(
        context: *mut c_void,
        value: u64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_user_query_context_new(
        out_context: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_context_with_current_suggestion(
        suggestion: *mut c_void,
        out_context: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_context_get_enable_ranked_results(context: *mut c_void) -> i32;
    pub fn cs_user_query_context_set_enable_ranked_results(context: *mut c_void, value: i32);
    pub fn cs_user_query_context_get_disable_semantic_search(
        context: *mut c_void,
        out_value: *mut i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_context_set_disable_semantic_search(
        context: *mut c_void,
        value: i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_context_get_max_result_count(context: *mut c_void) -> i64;
    pub fn cs_user_query_context_set_max_result_count(context: *mut c_void, value: i64);
    pub fn cs_user_query_context_get_max_suggestion_count(context: *mut c_void) -> i64;
    pub fn cs_user_query_context_set_max_suggestion_count(context: *mut c_void, value: i64);
    pub fn cs_user_query_context_get_max_ranked_result_count(
        context: *mut c_void,
        out_value: *mut i64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_context_set_max_ranked_result_count(
        context: *mut c_void,
        value: i64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_suggestion_get_localized_attributed_suggestion(
        suggestion: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_suggestion_get_kind(suggestion: *mut c_void) -> i64;
    pub fn cs_suggestion_compare(suggestion: *mut c_void, other: *mut c_void) -> i32;
    pub fn cs_suggestion_compare_by_rank(suggestion: *mut c_void, other: *mut c_void) -> i32;

    pub fn cs_search_query_new(
        query_string: *const c_char,
        query_context: *mut c_void,
        out_query: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_new_with_attributes(
        query_string: *const c_char,
        attributes_json: *const c_char,
        out_query: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_execute(
        query: *mut c_void,
        timeout_seconds: i32,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_cancel(query: *mut c_void);
    pub fn cs_search_query_is_cancelled(query: *mut c_void) -> i32;
    pub fn cs_search_query_found_item_count(query: *mut c_void) -> u64;
    pub fn cs_search_query_get_protection_classes(
        query: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_search_query_set_protection_classes(
        query: *mut c_void,
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_user_query_prepare(out_error: *mut *mut c_char) -> i32;
    pub fn cs_user_query_prepare_protection_classes(
        values_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_new(
        user_query_string: *const c_char,
        user_query_context: *mut c_void,
        out_query: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_execute(
        query: *mut c_void,
        timeout_seconds: i32,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_found_suggestion_count(query: *mut c_void) -> u64;
    pub fn cs_user_query_user_engaged_with_item(
        query: *mut c_void,
        item: *mut c_void,
        visible_items_json: *const c_char,
        interaction: i64,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_user_query_user_engaged_with_suggestion(
        query: *mut c_void,
        suggestion: *mut c_void,
        visible_suggestions_json: *const c_char,
        interaction: i64,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_searchable_index_delegate_new(
        context: *mut c_void,
        release_context: Option<CsDelegateReleaseContext>,
        reindex_all: Option<CsDelegateReindexAll>,
        reindex_identifiers: Option<CsDelegateReindexIdentifiers>,
        did_throttle: Option<CsDelegateNotification>,
        did_finish_throttle: Option<CsDelegateNotification>,
        data_for_item: Option<CsDelegateDataForItem>,
        file_url_for_item: Option<CsDelegateFileUrlForItem>,
        searchable_items_for_identifiers: Option<CsDelegateSearchableItemsForIdentifiers>,
        searchable_items_did_update: Option<CsDelegateSearchableItemsDidUpdate>,
        out_delegate: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_reindex_all(
        delegate: *mut c_void,
        index: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_reindex_identifiers(
        delegate: *mut c_void,
        index: *mut c_void,
        identifiers_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_did_throttle(
        delegate: *mut c_void,
        index: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_did_finish_throttle(
        delegate: *mut c_void,
        index: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_data_request(
        delegate: *mut c_void,
        index: *mut c_void,
        item_identifier: *const c_char,
        type_identifier: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_file_url_request(
        delegate: *mut c_void,
        index: *mut c_void,
        item_identifier: *const c_char,
        type_identifier: *const c_char,
        in_place: i32,
        out_url: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_searchable_items_for_identifiers(
        delegate: *mut c_void,
        identifiers_json: *const c_char,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_searchable_index_delegate_simulate_searchable_items_did_update(
        delegate: *mut c_void,
        items_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_index_extension_request_handler_new(
        context: *mut c_void,
        release_context: Option<CsDelegateReleaseContext>,
        reindex_all: Option<CsDelegateReindexAll>,
        reindex_identifiers: Option<CsDelegateReindexIdentifiers>,
        did_throttle: Option<CsDelegateNotification>,
        did_finish_throttle: Option<CsDelegateNotification>,
        data_for_item: Option<CsDelegateDataForItem>,
        file_url_for_item: Option<CsDelegateFileUrlForItem>,
        searchable_items_for_identifiers: Option<CsDelegateSearchableItemsForIdentifiers>,
        searchable_items_did_update: Option<CsDelegateSearchableItemsDidUpdate>,
        out_handler: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_import_extension_new(
        context: *mut c_void,
        release_context: Option<CsDelegateReleaseContext>,
        update: Option<CsImportExtensionUpdate>,
        out_extension: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_import_extension_simulate_update(
        extension: *mut c_void,
        attributes: *mut c_void,
        content_url: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn cs_default_index_extension_request_handler_new(
        out_handler: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cs_default_index_extension_request_handler_get_reindex_all_count(
        handler: *mut c_void,
    ) -> u64;
    pub fn cs_default_index_extension_request_handler_get_reindex_identifiers_count(
        handler: *mut c_void,
    ) -> u64;
    pub fn cs_default_index_extension_request_handler_get_did_throttle_count(
        handler: *mut c_void,
    ) -> u64;
    pub fn cs_default_index_extension_request_handler_get_did_finish_throttle_count(
        handler: *mut c_void,
    ) -> u64;
    pub fn cs_default_index_extension_request_handler_get_last_identifiers(
        handler: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    // Async APIs (feature-gated in Rust)
    pub fn corespotlight_index_searchable_items_async(
        index: *mut c_void,
        items_json: *const c_char,
        timeout_seconds: i32,
        cb: extern "C" fn(*const c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
    pub fn corespotlight_delete_searchable_items_with_identifiers_async(
        index: *mut c_void,
        identifiers_json: *const c_char,
        timeout_seconds: i32,
        cb: extern "C" fn(*const c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
    pub fn corespotlight_delete_searchable_items_with_domain_identifiers_async(
        index: *mut c_void,
        domain_identifiers_json: *const c_char,
        timeout_seconds: i32,
        cb: extern "C" fn(*const c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
    pub fn corespotlight_delete_all_searchable_items_async(
        index: *mut c_void,
        cb: extern "C" fn(*const c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
    pub fn corespotlight_fetch_last_client_state_async(
        index: *mut c_void,
        cb: extern "C" fn(*const c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
}
