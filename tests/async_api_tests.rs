#![cfg(feature = "async")]

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
