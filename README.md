# corespotlight

Safe Rust bindings for Apple's [Core Spotlight](https://developer.apple.com/documentation/corespotlight) framework on macOS.

> **Status:** v0.1.0 covers searchable indexes, searchable items, attribute sets, and the most common metadata fields needed to index and delete app content from Spotlight.

## Quick start

```rust,no_run
use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = CSSearchableIndex::new("doom-fish.demo")?;
    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    attributes.set_title(Some("doom-fish corespotlight"))?;
    attributes.set_display_name(Some("doom-fish"))?;
    attributes.set_keywords(["doom-fish", "spotlight"])?;

    let item = CSSearchableItem::new(
        Some("doom-fish.corespotlight.demo"),
        Some("doom-fish"),
        &attributes,
    )?;

    index.index_searchable_items(&[item.clone()])?;
    index.delete_searchable_items_with_identifiers(["doom-fish.corespotlight.demo"])?;
    Ok(())
}
```

## Highlights

- `CSSearchableIndex::default_searchable_index` and `CSSearchableIndex::new`
- `index_searchable_items`, `delete_searchable_items_with_identifiers`, `delete_searchable_items_with_domain_identifiers`, and `delete_all_searchable_items`
- `CSSearchableItem` construction plus expiration-date accessors
- `CSSearchableItemAttributeSet` setters/getters for content type, title, content description, display name, keywords, thumbnails, content URLs, coordinates, and rating
- Completion-handler APIs bridged to synchronous Rust with a `DispatchSemaphore`

## Smoke example

Run the Spotlight smoke test with:

```bash
cargo run --all-features --example 01_index_smoke
```

It indexes one item titled `doom-fish corespotlight smoke`, deletes it by identifier, and prints `✅ corespotlight index + delete OK`.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
