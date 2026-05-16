#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

use corespotlight::prelude::*;

pub fn unique_label(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("doom-fish.corespotlight.tests.{prefix}.{nanos}")
}

pub fn sample_index(prefix: &str) -> Result<CSSearchableIndex, CoreSpotlightError> {
    CSSearchableIndex::new(unique_label(prefix))
}

pub fn sample_attributes(
    title: &str,
) -> Result<CSSearchableItemAttributeSet, CoreSpotlightError> {
    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    attributes.set_title(Some(title))?;
    attributes.set_display_name(Some(title))?;
    attributes.set_content_description(Some("Core Spotlight Rust test item"))?;
    attributes.set_keywords(["corespotlight-rs", "tests"])?;
    Ok(attributes)
}

pub fn sample_item(
    prefix: &str,
    title: &str,
) -> Result<(String, CSSearchableItem), CoreSpotlightError> {
    let identifier = unique_label(prefix);
    let attributes = sample_attributes(title)?;
    let item = CSSearchableItem::new(Some(&identifier), Some("doom-fish.tests"), &attributes)?;
    Ok((identifier, item))
}
