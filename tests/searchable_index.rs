mod common;

use common::{sample_index, sample_item};
use corespotlight::prelude::*;

#[test]
fn searchable_index_indexes_batches_and_deletes_items() -> Result<(), Box<dyn std::error::Error>> {
    assert!(CSSearchableIndex::is_indexing_available());

    let index = sample_index("index")?;
    let (identifier, item) = sample_item("batched-item", "Batched guide")?;

    index.begin_index_batch()?;
    index.index_searchable_items(&[item])?;
    index.end_index_batch_with_client_state(b"client-state-v1")?;
    assert_eq!(index.fetch_last_client_state()?, b"client-state-v1");

    assert!(index
        .end_index_batch_with_expected_client_state(Some(b"expected-client-state"), b"client-state-v2")
        .is_err());

    index.delete_searchable_items_with_identifiers([identifier])?;
    index.delete_searchable_items_with_domain_identifiers(["doom-fish.tests"])?;
    index.delete_all_searchable_items()?;
    Ok(())
}
