mod common;

use common::{sample_index, sample_item};
use corespotlight::prelude::*;

#[test]
fn default_request_handler_tracks_simulated_events() -> Result<(), Box<dyn std::error::Error>> {
    let index = sample_index("default-handler")?;
    let handler = DefaultIndexExtensionRequestHandler::new()?;
    index.set_default_extension_request_handler(Some(&handler))?;

    let initial_reindex_all = handler.reindex_all_count();
    let initial_reindex_identifiers = handler.reindex_identifiers_count();
    let initial_did_throttle = handler.did_throttle_count();
    let initial_did_finish_throttle = handler.did_finish_throttle_count();

    handler.simulate_reindex_all(&index)?;
    handler.simulate_reindex_identifiers(&index, ["alpha", "beta"])?;
    handler.simulate_did_throttle(&index)?;
    handler.simulate_did_finish_throttle(&index)?;

    let (_, item) = sample_item("updated", "Updated item")?;
    handler.simulate_searchable_items_did_update(&[item])?;

    assert!(handler.reindex_all_count() > initial_reindex_all);
    assert!(handler.reindex_identifiers_count() > initial_reindex_identifiers);
    assert!(handler.did_throttle_count() > initial_did_throttle);
    assert!(handler.did_finish_throttle_count() > initial_did_finish_throttle);
    assert_eq!(
        handler.last_identifiers()?,
        vec![String::from("alpha"), String::from("beta")]
    );
    Ok(())
}
