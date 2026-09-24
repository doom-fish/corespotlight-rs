#![allow(dead_code)]

use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use corespotlight::prelude::*;

static LIVE_INDEX: Mutex<()> = Mutex::new(());

fn nanos_since_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

pub fn unique_label(prefix: &str) -> String {
    format!(
        "doom-fish.corespotlight.tests.{prefix}.{}",
        nanos_since_epoch()
    )
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

pub fn live_tests_enabled(test: &str) -> bool {
    let enabled = std::env::var("CORESPOTLIGHT_LIVE_TESTS").as_deref() == Ok("1");
    if !enabled {
        eprintln!("{test}: skipped; set CORESPOTLIGHT_LIVE_TESTS=1 to use the Spotlight index");
    }
    enabled
}

pub struct LiveIndex {
    name: String,
    domain: String,
    _exclusive: MutexGuard<'static, ()>,
}

impl LiveIndex {
    pub fn acquire(test: &str) -> Option<Self> {
        if !live_tests_enabled(test) {
            return None;
        }
        let exclusive = LIVE_INDEX.lock().unwrap_or_else(PoisonError::into_inner);
        Some(Self {
            name: unique_label(test),
            domain: format!("corespotlighttests{}", nanos_since_epoch()),
            _exclusive: exclusive,
        })
    }

    pub fn index(&self) -> Result<CSSearchableIndex, CoreSpotlightError> {
        CSSearchableIndex::new(&self.name)
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn item(&self, title: &str) -> Result<(String, CSSearchableItem), CoreSpotlightError> {
        let identifier = format!("{}-{title}", self.domain);
        let attributes = sample_attributes(title)?;
        let item = CSSearchableItem::new(Some(&identifier), Some(&self.domain), &attributes)?;
        Ok((identifier, item))
    }
}

impl Drop for LiveIndex {
    fn drop(&mut self) {
        let removed = self.index().and_then(|index| {
            index.delete_searchable_items_with_domain_identifiers([self.domain.as_str()])
        });
        if let Err(error) = removed {
            eprintln!(
                "could not remove the Spotlight test items in domain {}: {error}",
                self.domain
            );
        }
    }
}
