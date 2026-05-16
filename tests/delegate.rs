#![allow(clippy::too_many_lines)]

mod common;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use common::{sample_index, sample_item};
use corespotlight::prelude::*;

#[test]
fn delegate_simulation_invokes_registered_callbacks() -> Result<(), Box<dyn std::error::Error>> {
    let index = sample_index("delegate")?;

    let reindex_all_count = Arc::new(AtomicUsize::new(0));
    let reindex_identifiers = Arc::new(Mutex::new(Vec::<String>::new()));
    let did_throttle_count = Arc::new(AtomicUsize::new(0));
    let did_finish_throttle_count = Arc::new(AtomicUsize::new(0));
    let did_update_count = Arc::new(AtomicUsize::new(0));

    let delegate = CSSearchableIndexDelegate::new(
        CSSearchableIndexDelegateCallbacks::new(
            {
                let reindex_all_count = Arc::clone(&reindex_all_count);
                move |_| {
                    reindex_all_count.fetch_add(1, Ordering::SeqCst);
                }
            },
            {
                let reindex_identifiers = Arc::clone(&reindex_identifiers);
                move |_, identifiers| {
                    reindex_identifiers.lock().unwrap().extend(identifiers);
                }
            },
        )
        .did_throttle({
            let did_throttle_count = Arc::clone(&did_throttle_count);
            move |_| {
                did_throttle_count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .did_finish_throttle({
            let did_finish_throttle_count = Arc::clone(&did_finish_throttle_count);
            move |_| {
                did_finish_throttle_count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .data_for_item(|_, item_identifier, type_identifier| {
            Ok(Some(format!("{item_identifier}:{type_identifier}").into_bytes()))
        })
        .file_url_for_item(|_, item_identifier, type_identifier, in_place| {
            Ok(Some(format!(
                "https://example.com/{item_identifier}/{type_identifier}?in_place={in_place}"
            )))
        })
        .searchable_items_for_identifiers(|identifiers| {
            identifiers
                .into_iter()
                .map(|identifier| {
                    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
                    attributes.set_title(Some(&identifier))?;
                    CSSearchableItem::new(Some(&identifier), Some("delegate.tests"), &attributes)
                })
                .collect()
        })
        .searchable_items_did_update({
            let did_update_count = Arc::clone(&did_update_count);
            move |items| {
                did_update_count.fetch_add(items.len(), Ordering::SeqCst);
            }
        }),
    )?;

    index.set_delegate(Some(&delegate))?;

    let initial_reindex_all = reindex_all_count.load(Ordering::SeqCst);
    let initial_reindex_identifiers_len = reindex_identifiers.lock().unwrap().len();
    let initial_did_throttle = did_throttle_count.load(Ordering::SeqCst);
    let initial_did_finish_throttle = did_finish_throttle_count.load(Ordering::SeqCst);
    let initial_did_update = did_update_count.load(Ordering::SeqCst);

    delegate.simulate_reindex_all(&index)?;
    delegate.simulate_reindex_identifiers(&index, ["one", "two"])?;
    delegate.simulate_did_throttle(&index)?;
    delegate.simulate_did_finish_throttle(&index)?;

    assert_eq!(
        delegate.simulate_data_request(&index, "item-1", "public.plain-text")?,
        b"item-1:public.plain-text".to_vec()
    );
    assert_eq!(
        delegate
            .simulate_file_url_request(&index, "item-1", "public.plain-text", true)?
            .as_deref(),
        Some("https://example.com/item-1/public.plain-text?in_place=true")
    );

    let items = delegate.simulate_searchable_items_for_identifiers(["one", "two"])?;
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].unique_identifier().as_deref(), Some("one"));
    delegate.simulate_searchable_items_did_update(&items)?;

    assert!(reindex_all_count.load(Ordering::SeqCst) > initial_reindex_all);
    assert_eq!(
        &reindex_identifiers.lock().unwrap()[initial_reindex_identifiers_len..],
        [String::from("one"), String::from("two")]
    );
    assert!(did_throttle_count.load(Ordering::SeqCst) > initial_did_throttle);
    assert!(
        did_finish_throttle_count.load(Ordering::SeqCst) > initial_did_finish_throttle
    );
    assert!(did_update_count.load(Ordering::SeqCst) >= initial_did_update + 2);

    let (_, sample_item) = sample_item("delegate-updated", "Updated item")?;
    delegate.simulate_searchable_items_did_update(&[sample_item])?;
    assert!(did_update_count.load(Ordering::SeqCst) >= initial_did_update + 3);
    Ok(())
}
