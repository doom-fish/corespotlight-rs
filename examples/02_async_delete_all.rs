use corespotlight::async_api::AsyncCSSearchableIndex;
use corespotlight::CSSearchableIndex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        // Create the default searchable index
        let index = CSSearchableIndex::default_searchable_index()?;

        // Delete all items to start fresh
        println!("Deleting all items...");
        AsyncCSSearchableIndex::delete_all_searchable_items(&index).await?;
        println!("✓ All items deleted");

        Ok(())
    })
}
