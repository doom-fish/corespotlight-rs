use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = CSSearchableIndex::new("doom-fish.corespotlight.smoke")?;
    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    attributes.set_title(Some("doom-fish corespotlight smoke"))?;
    attributes.set_content_description(Some("Core Spotlight smoke item"))?;
    attributes.set_display_name(Some("doom-fish smoke"))?;
    attributes.set_keywords(["doom-fish", "corespotlight", "smoke"])?;

    let identifier = "doom-fish.corespotlight.item";
    let item = CSSearchableItem::new(Some(identifier), Some("doom-fish"), &attributes)?;
    index.index_searchable_items(&[item])?;
    index.delete_searchable_items_with_identifiers([identifier])?;

    println!("✅ corespotlight index + delete OK");
    Ok(())
}
