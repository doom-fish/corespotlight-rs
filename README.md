# corespotlight

Safe Rust bindings for Apple's [Core Spotlight](https://developer.apple.com/documentation/corespotlight) framework on macOS.

> **Status:** v0.2.1 closes the remaining audit gaps with the real Core Spotlight version string, the suggestion-highlight key, `NSUserActivity.contentAttributeSet`, and `CSImportExtension` support. See [COVERAGE.md](COVERAGE.md) for the current SDK matrix and known caveats.

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

- `CSSearchableIndex` creation, deletion, batching, client-state fetch, external-provider fetch, and delegate / request-handler attachment
- `CSSearchableItem` rank comparison, mutable identifiers, expiration dates, update flags, and update-listener options
- `CSSearchableItemAttributeSet` generic typed field enums for strings, arrays, numbers, URLs, data, dates, people, and maps, plus convenience helpers for common fields and `NSUserActivity.contentAttributeSet`
- `CSSearchQuery`, `CSUserQuery`, `CSSearchQueryContext`, `CSUserQueryContext`, `CSSuggestion`, Core Spotlight version metadata, error domains, suggestion/action keys, and mailbox constants
- Simulatable `CSSearchableIndexDelegate`, `CSIndexExtensionRequestHandler`, `CSImportExtension`, and `DefaultIndexExtensionRequestHandler` flows for integration tests and examples

## Examples

```bash
cargo run --example 01_index_smoke
cargo run --example 02_attribute_fields
cargo run --example 03_query_settings
cargo run --example 04_delegate_simulation
cargo run --example 05_index_extension_request_handler
cargo run --example 06_default_index_extension_request_handler
cargo run --example 07_user_activity_import_extension
```

## Notes

- `CSCustomAttributeKey` is exposed, but Apple validates custom key names against the current bundle identifier at runtime; command-line examples may not always be able to create them.
- `CSUserQuery::user_engaged_with_item` and `CSUserQuery::user_engaged_with_suggestion` currently return a bridge error on the command-line bridge because Apple’s Swift overlay uses opaque wrapper types that are not yet surfaced to Rust.
- `CSSearchableIndex::end_index_batch_with_expected_client_state` currently rejects non-`None` expected state values on the current SDK bridge; the limitation is documented in [COVERAGE.md](COVERAGE.md).

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
