#![cfg(feature = "async")]

mod common;

use common::{sample_index, sample_item};
use corespotlight::async_api::AsyncCSSearchableIndex;
use corespotlight::CSSearchableIndex;

#[test]
fn test_delete_all_searchable_items() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let index = CSSearchableIndex::default_searchable_index()?;
        AsyncCSSearchableIndex::delete_all_searchable_items(&index).await?;
        Ok(())
    })
}

#[test]
fn test_delete_searchable_items_with_identifiers() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let index = CSSearchableIndex::default_searchable_index()?;
        AsyncCSSearchableIndex::delete_searchable_items_with_identifiers(
            &index,
            vec!["test-id-1", "test-id-2"],
        )
        .await?;
        Ok(())
    })
}

#[test]
fn test_delete_searchable_items_with_domain_identifiers() -> Result<(), Box<dyn std::error::Error>>
{
    pollster::block_on(async {
        let index = CSSearchableIndex::default_searchable_index()?;
        AsyncCSSearchableIndex::delete_searchable_items_with_domain_identifiers(
            &index,
            vec!["test-domain-1"],
        )
        .await?;
        Ok(())
    })
}

#[test]
fn index_future_does_not_borrow_its_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let index = sample_index("async-outlives")?;
    let (identifier, item) = sample_item("async-outlives-item", "Async outlives")?;
    let items = vec![item];
    let future = AsyncCSSearchableIndex::index_searchable_items(&index, &items);
    let delete_future =
        AsyncCSSearchableIndex::delete_searchable_items_with_identifiers(&index, vec![identifier]);
    drop(items);
    drop(index);
    pollster::block_on(future)?;
    pollster::block_on(delete_future)?;
    Ok(())
}

#[test]
fn fetch_last_client_state_returns_the_stored_state() -> Result<(), Box<dyn std::error::Error>> {
    let index = sample_index("async-client-state")?;
    assert_eq!(
        pollster::block_on(AsyncCSSearchableIndex::fetch_last_client_state(&index))?,
        Vec::<u8>::new()
    );
    index.begin_index_batch()?;
    index.end_index_batch_with_client_state(b"async-client-state")?;
    let future = AsyncCSSearchableIndex::fetch_last_client_state(&index);
    drop(index);
    assert_eq!(pollster::block_on(future)?, b"async-client-state");
    Ok(())
}
