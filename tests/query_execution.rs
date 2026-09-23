mod common;

use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use common::sample_index;
use corespotlight::prelude::*;

fn unique_tag(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{prefix}{nanos}")
}

fn index_titles(
    index: &CSSearchableIndex,
    tag: &str,
    titles: &[&str],
) -> Result<(), CoreSpotlightError> {
    let items = titles
        .iter()
        .enumerate()
        .map(|(position, title)| {
            let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
            attributes.set_title(Some(title))?;
            attributes.set_keywords([tag])?;
            CSSearchableItem::new(Some(&format!("{tag}-{position}")), Some(tag), &attributes)
        })
        .collect::<Result<Vec<_>, _>>()?;
    index.index_searchable_items(&items)
}

fn titles_matching(tag: &str, clause: &str) -> Result<Vec<String>, CoreSpotlightError> {
    let context = CSSearchQueryContext::new()?;
    context.set_fetch_attributes(["title"])?;
    let query = CSSearchQuery::new(format!("keywords == \"{tag}\" && ({clause})"), Some(&context))?;
    let result = query.execute(Duration::from_secs(10))?;
    let mut titles = result
        .items
        .iter()
        .filter_map(|item| item.attribute_set().ok()?.title())
        .collect::<Vec<_>>();
    titles.sort();
    Ok(titles)
}

fn wait_for_titles(
    tag: &str,
    clause: &str,
    expected: usize,
) -> Result<Vec<String>, CoreSpotlightError> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let titles = titles_matching(tag, clause)?;
        if titles.len() >= expected || Instant::now() >= deadline {
            return Ok(titles);
        }
        thread::sleep(Duration::from_millis(250));
    }
}

#[test]
fn executing_a_query_returns_indexed_items_and_runs_once() -> Result<(), Box<dyn std::error::Error>>
{
    let index = sample_index("query-execution")?;
    let tag = unique_tag("corespotlightexecution");
    index_titles(&index, &tag, &["alpha", "beta"])?;
    assert_eq!(wait_for_titles(&tag, "title == \"*\"", 2)?, ["alpha", "beta"]);

    let query = CSSearchQuery::new(format!("keywords == \"{tag}\""), None)?;
    let result = query.execute(Duration::from_secs(10))?;
    assert_eq!(result.items.len(), 2);
    assert!(!result.cancelled);
    let error = query.execute(Duration::from_secs(10)).unwrap_err();
    assert!(error.message.contains("already started"), "{error}");

    let cancelled = CSSearchQuery::new(format!("keywords == \"{tag}\""), None)?;
    cancelled.cancel();
    let error = cancelled.execute(Duration::from_secs(10)).unwrap_err();
    assert_eq!(error.code, CSSearchQueryErrorCode::Cancelled as i64);

    index.delete_searchable_items_with_domain_identifiers([tag])?;
    Ok(())
}
