mod common;

use std::thread;
use std::time::{Duration, Instant};

use common::LiveIndex;
use corespotlight::prelude::*;

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
    let Some(live) = LiveIndex::acquire("executing_a_query_returns_indexed_items_and_runs_once")
    else {
        return Ok(());
    };
    let index = live.index()?;
    let tag = live.domain();
    index_titles(&index, tag, &["alpha", "beta"])?;
    assert_eq!(wait_for_titles(tag, "title == \"*\"", 2)?, ["alpha", "beta"]);

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

#[test]
fn escaped_values_match_literally() -> Result<(), Box<dyn std::error::Error>> {
    let Some(live) = LiveIndex::acquire("escaped_values_match_literally") else {
        return Ok(());
    };
    let index = live.index()?;
    let tag = live.domain();
    let titles = ["a\"b", "a\\b", "a*b", "axb", "a?b", "a'b"];
    index_titles(&index, tag, &titles)?;
    assert_eq!(wait_for_titles(tag, "title == \"*\"", titles.len())?.len(), titles.len());

    for title in titles {
        let clause = format!("title == \"{}\"", CSSearchQuery::escape_value(title));
        assert_eq!(titles_matching(tag, &clause)?, [title], "{clause}");
    }
    assert_eq!(titles_matching(tag, "title == \"a*b\"")?.len(), titles.len());

    index.delete_searchable_items_with_domain_identifiers([tag])?;
    Ok(())
}
