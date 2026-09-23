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

    index.begin_index_batch()?;
    match index.end_index_batch_with_expected_client_state(
        Some(b"expected-client-state"),
        b"client-state-v2",
    ) {
        Err(error) if error.message.contains("requires macOS 15.0") => {
            index.end_index_batch_with_client_state(b"client-state-v1")?;
        }
        Err(error) => {
            assert_eq!(error.code, CSIndexErrorCode::MismatchedClientState as i64);
            assert_eq!(index.fetch_last_client_state()?, b"client-state-v1");
            index.begin_index_batch()?;
            index.end_index_batch_with_expected_client_state(
                Some(b"client-state-v1"),
                b"client-state-v2",
            )?;
            assert_eq!(index.fetch_last_client_state()?, b"client-state-v2");
            index.begin_index_batch()?;
            index.end_index_batch_with_expected_client_state(None, b"client-state-v3")?;
            assert_eq!(index.fetch_last_client_state()?, b"client-state-v3");
        }
        Ok(()) => panic!("a mismatched expected client state must be rejected"),
    }

    index.delete_searchable_items_with_identifiers([identifier])?;
    index.delete_searchable_items_with_domain_identifiers(["doom-fish.tests"])?;
    index.delete_all_searchable_items()?;
    Ok(())
}

#[test]
fn batch_misuse_returns_errors_instead_of_aborting() -> Result<(), Box<dyn std::error::Error>> {
    let index = sample_index("batch-misuse")?;

    let error = index
        .end_index_batch_with_client_state(b"no-open-batch")
        .unwrap_err();
    assert_eq!(error.domain, CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN);
    assert!(error.message.contains("There is no batch open"), "{error}");

    assert!(index
        .end_index_batch_with_expected_client_state(None, b"no-open-batch")
        .is_err());

    index.begin_index_batch()?;
    let error = index.begin_index_batch().unwrap_err();
    assert!(error.message.contains("There is already an open batch"), "{error}");
    index.end_index_batch_with_client_state(b"closed")?;
    assert_eq!(index.fetch_last_client_state()?, b"closed");

    let default_index = CSSearchableIndex::default_searchable_index()?;
    let error = default_index.begin_index_batch().unwrap_err();
    assert!(error.message.contains("Batching is not supported"), "{error}");
    Ok(())
}
