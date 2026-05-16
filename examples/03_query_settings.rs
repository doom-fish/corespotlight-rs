use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "CoreSpotlight {} (API {})",
        core_spotlight_version_string()?,
        core_spotlight_api_version()
    );
    println!("index error domain: {}", index_error_domain()?);
    println!("search query error domain: {}", search_query_error_domain()?);
    println!("mailboxes: {}, {}, {}", mailbox_inbox()?, mailbox_sent()?, mailbox_archive()?);

    let search_context = CSSearchQueryContext::new()?;
    search_context.set_fetch_attributes(["title", "displayName"])?;
    search_context.set_filter_queries(["title == \"Foundation Models Guide\""])?;
    search_context.set_keyboard_language(Some("en-US"))?;
    search_context.set_source_options(CSSearchQuerySourceOptions::ALLOW_MAIL)?;

    let search_query = CSSearchQuery::new("title == \"Foundation Models Guide\"", Some(&search_context))?;
    search_query.set_protection_classes(["NSFileProtectionCompleteUntilFirstUserAuthentication"])?;
    println!("search query protection classes: {:?}", search_query.protection_classes()?);

    let user_context = CSUserQueryContext::with_current_suggestion(None)?;
    user_context.set_enable_ranked_results(true);
    user_context.set_disable_semantic_search(true)?;
    user_context.set_max_result_count(5);
    user_context.set_max_suggestion_count(3);

    let user_query = CSUserQuery::new(Some("foundation models"), Some(&user_context))?;
    user_query.set_protection_classes(["NSFileProtectionCompleteUntilFirstUserAuthentication"])?;
    println!("user query protection classes: {:?}", user_query.protection_classes()?);
    Ok(())
}
