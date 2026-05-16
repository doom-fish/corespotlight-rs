mod common;

use common::sample_item;
use corespotlight::prelude::*;

#[test]
fn settings_constants_and_query_contexts_are_available() -> Result<(), Box<dyn std::error::Error>> {
    assert!(core_spotlight_version_number() > 0.0);
    assert!(core_spotlight_api_version() > 0);
    assert!(!core_spotlight_version_string()?.is_empty());
    assert!(!index_error_domain()?.is_empty());
    assert!(!search_query_error_domain()?.is_empty());
    assert!(!searchable_item_action_type()?.is_empty());
    assert!(!searchable_item_activity_identifier()?.is_empty());
    assert!(!query_continuation_action_type()?.is_empty());
    assert!(!search_query_string_key()?.is_empty());
    assert!(!mailbox_inbox()?.is_empty());
    assert!(!mailbox_drafts()?.is_empty());
    assert!(!mailbox_sent()?.is_empty());
    assert!(!mailbox_junk()?.is_empty());
    assert!(!mailbox_trash()?.is_empty());
    assert!(!mailbox_archive()?.is_empty());

    let search_context = CSSearchQueryContext::new()?;
    search_context.set_fetch_attributes(["title", "displayName"])?;
    assert_eq!(
        search_context.fetch_attributes()?,
        vec![String::from("title"), String::from("displayName")]
    );
    search_context.set_filter_queries(["title == \"Guide\""])?;
    assert_eq!(
        search_context.filter_queries()?,
        vec![String::from("title == \"Guide\"")]
    );
    search_context.set_keyboard_language(Some("en-US"))?;
    assert_eq!(search_context.keyboard_language().as_deref(), Some("en-US"));
    search_context.set_source_options(CSSearchQuerySourceOptions::ALLOW_MAIL)?;
    assert!(
        search_context
            .source_options()
            .contains(CSSearchQuerySourceOptions::ALLOW_MAIL)
    );

    let search_query = CSSearchQuery::new("title == \"Guide\"", Some(&search_context))?;
    search_query.set_protection_classes(["NSFileProtectionCompleteUntilFirstUserAuthentication"])?;
    assert_eq!(
        search_query.protection_classes()?,
        vec![String::from("NSFileProtectionCompleteUntilFirstUserAuthentication")]
    );
    search_query.cancel();
    assert!(search_query.is_cancelled());

    let search_query_with_attributes = CSSearchQuery::new_with_attributes(
        "title == \"Guide\"",
        ["title", "displayName"],
    )?;
    search_query_with_attributes.cancel();
    assert!(search_query_with_attributes.is_cancelled());

    let user_context = CSUserQueryContext::with_current_suggestion(None)?;
    user_context.set_enable_ranked_results(true);
    assert!(user_context.enable_ranked_results());
    user_context.set_disable_semantic_search(true)?;
    assert!(user_context.disable_semantic_search()?);
    user_context.set_max_result_count(5);
    user_context.set_max_suggestion_count(3);
    user_context.set_max_ranked_result_count(2)?;
    assert_eq!(user_context.max_result_count(), 5);
    assert_eq!(user_context.max_suggestion_count(), 3);
    assert_eq!(user_context.max_ranked_result_count()?, 2);
    assert_eq!(CSUserInteraction::DEFAULT.raw_value(), 0);
    assert_eq!(CSUserInteraction::Focus.raw_value(), 1);

    let user_query = CSUserQuery::new(Some("Guide"), Some(&user_context))?;
    user_query.set_protection_classes(["NSFileProtectionCompleteUntilFirstUserAuthentication"])?;
    assert_eq!(
        user_query.protection_classes()?,
        vec![String::from("NSFileProtectionCompleteUntilFirstUserAuthentication")]
    );
    user_query.cancel();
    assert!(user_query.is_cancelled());

    let (_, item) = sample_item("user-query-item", "Guide")?;
    assert!(user_query
        .user_engaged_with_item(&item, std::slice::from_ref(&item), CSUserInteraction::Select)
        .is_err());
    Ok(())
}
