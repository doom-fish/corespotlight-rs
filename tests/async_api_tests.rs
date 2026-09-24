#![cfg(feature = "async")]

mod common;

use common::LiveIndex;
use corespotlight::async_api::AsyncCSSearchableIndex;

#[test]
fn test_delete_all_searchable_items() -> Result<(), Box<dyn std::error::Error>> {
    let Some(live) = LiveIndex::acquire("test_delete_all_searchable_items") else {
        return Ok(());
    };
    pollster::block_on(async {
        let index = live.index()?;
        AsyncCSSearchableIndex::delete_all_searchable_items(&index).await?;
        Ok(())
    })
}

#[test]
fn test_delete_searchable_items_with_identifiers() -> Result<(), Box<dyn std::error::Error>> {
    let Some(live) = LiveIndex::acquire("test_delete_searchable_items_with_identifiers") else {
        return Ok(());
    };
    pollster::block_on(async {
        let index = live.index()?;
        AsyncCSSearchableIndex::delete_searchable_items_with_identifiers(
            &index,
            vec![
                format!("{}-1", live.domain()),
                format!("{}-2", live.domain()),
            ],
        )
        .await?;
        Ok(())
    })
}

#[test]
fn test_delete_searchable_items_with_domain_identifiers() -> Result<(), Box<dyn std::error::Error>>
{
    let Some(live) = LiveIndex::acquire("test_delete_searchable_items_with_domain_identifiers")
    else {
        return Ok(());
    };
    pollster::block_on(async {
        let index = live.index()?;
        AsyncCSSearchableIndex::delete_searchable_items_with_domain_identifiers(
            &index,
            vec![live.domain()],
        )
        .await?;
        Ok(())
    })
}

#[test]
fn index_future_does_not_borrow_its_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let Some(live) = LiveIndex::acquire("index_future_does_not_borrow_its_inputs") else {
        return Ok(());
    };
    let index = live.index()?;
    let (identifier, item) = live.item("Async outlives")?;
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
    let Some(live) = LiveIndex::acquire("fetch_last_client_state_returns_the_stored_state") else {
        return Ok(());
    };
    let index = live.index()?;
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
