# corespotlight

Safe Rust bindings for Apple's [Core Spotlight](https://developer.apple.com/documentation/corespotlight) framework on macOS.

See [COVERAGE.md](COVERAGE.md) for the SDK matrix and known caveats.

## Requirements

- macOS 13 or later; the Swift bridge's deployment target is macOS 13.
- `end_index_batch_with_expected_client_state`, `CSUserQuery::prepare`, `CSUserQuery::prepare_protection_classes`, and the semantic-search and ranked-result settings of `CSUserQueryContext` need macOS 15. `CSSearchableItem` update-listener options and the `searchable_items_for_identifiers` / `searchable_items_did_update` delegate callbacks need macOS 15.4. On older systems these return an error.

## Installation

```toml
[dependencies]
corespotlight = "0.4"
```

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

## Async API

When the `async` feature is enabled, the [`async_api`](src/async_api.rs) module provides executor-agnostic futures for all completion-handler operations:

```rust,ignore
use corespotlight::async_api::AsyncCSSearchableIndex;
use corespotlight::CSSearchableIndex;

let index = CSSearchableIndex::default_searchable_index()?;
let items = vec![]; // Create your items
AsyncCSSearchableIndex::index_searchable_items(&index, &items).await?;
```

Available async operations:
- `AsyncCSSearchableIndex::index_searchable_items` — Index items asynchronously
- `AsyncCSSearchableIndex::delete_searchable_items_with_identifiers` — Delete items by identifier
- `AsyncCSSearchableIndex::delete_searchable_items_with_domain_identifiers` — Delete items by domain
- `AsyncCSSearchableIndex::delete_all_searchable_items` — Delete all items
- `AsyncCSSearchableIndex::fetch_last_client_state` — Fetch last client state (macOS 13+)

Enable with `cargo build --features async` or add to `Cargo.toml`:
```toml
corespotlight = { version = "0.4", features = ["async"] }
```

The futures don't borrow the index, items or identifiers: the bridge retains and copies them before the call returns, so the futures are `'static`.

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

## Query strings

`CSSearchQuery::new`, `CSSearchQuery::new_with_attributes` and `CSSearchQueryContext::set_filter_queries` take Core Spotlight query syntax, not plain text. Don't paste untrusted text into a query: a `"` ends the string literal and `*` is a wildcard, so the text can change what the query matches. Escape each value and put it inside double quotes:

```rust,ignore
let query = CSSearchQuery::new(
    format!("title == \"{}\"", CSSearchQuery::escape_value(user_text)),
    None,
)?;
```

`CSUserQuery::new` takes the user's natural-language search text and needs no escaping.

## Notes

- A `CSSearchQuery` or `CSUserQuery` runs once. Calling `execute` again on the same query returns an error; create a new query instead.
- Batching works only on indexes created with `CSSearchableIndex::new`. Beginning a batch on the default index, beginning a second batch, or ending a batch that isn't open returns an error instead of raising an Objective-C exception.
- Reindex callbacks receive a `CSReindexAcknowledgement`. Core Spotlight is told the reindex is done when you call `acknowledge` or drop the value, so you can move it to the thread or task that does the work. Dropping it while panicking doesn't acknowledge. The `simulate_reindex_*` helpers wait for the acknowledgement, up to 30 seconds.
- `CSCustomAttributeKey` is exposed, but Apple validates custom key names against the current bundle identifier at runtime; command-line examples may not always be able to create them.
- `CSUserQuery::user_engaged_with_item` and `CSUserQuery::user_engaged_with_suggestion` always return an error. Core Spotlight crashes when it's given an item or suggestion that the query didn't return, and the bridge can't check that.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
