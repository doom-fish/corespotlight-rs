# CoreSpotlight coverage (v0.2.1)

Legend: ✅ implemented and exercised by examples/tests, 🟡 partially implemented or has a runtime caveat, ⏭️ intentionally skipped.

## Public SDK surface

| SDK area | Status | Notes |
| --- | --- | --- |
| `CSSearchableIndex` | ✅ | Default/custom indexes, deletion APIs, batching, last-client-state fetch, external-provider fetch, delegate attachment, request-handler attachment |
| `CSSearchableIndex::endIndexBatch(expectedClientState:newClientState:)` | 🟡 | The current Swift overlay exposed to the bridge only provides `endBatch(withClientState:)`; passing a non-`None` expected state returns a bridge error instead of silently ignoring it |
| `CSSearchableItem` | ✅ | Construction, rank comparison, mutable identifiers, expiration dates, `isUpdate`, and `updateListenerOptions` |
| `CSSearchableItemAttributeSet` | ✅ | Typed string / array / number / URL / data / date / date-array / people / map accessors, common convenience fields, localized strings, people, and custom attribute values |
| `NSUserActivity (CSSearchableItemAttributeSet)` | ✅ | `contentAttributeSet` attachment and retrieval for Spotlight-backed activities |
| `CSLocalizedString` | ✅ | Creation and retrieval |
| `CSPerson` | ✅ | Creation and round-trip support through person-array attribute fields |
| `CSCustomAttributeKey` | 🟡 | Rust API is exposed, but Apple validates custom key names against the active bundle identifier at runtime; command-line demos may fail to create keys even with syntactically valid names |
| `CSSearchQuery` | ✅ | Construction, attribute-limited construction, execute, cancel, item counts, protection classes |
| `CSUserQuery` | 🟡 | Construction, execute, cancel, item/suggestion counts, protection classes; `user_engaged_with_item` and `user_engaged_with_suggestion` currently return a bridge error because the Swift overlay uses opaque `CSUserQuery.Item` / `CSUserQuery.Suggestion` wrappers not yet surfaced to Rust |
| `CSSearchQueryContext` | ✅ | Fetch attributes, filter queries, keyboard language, source options |
| `CSUserQueryContext` | ✅ | Current suggestion, ranked-results toggle, semantic-search toggle, max result/suggestion/ranked counts |
| `CSSuggestion` | ✅ | Localized attributed suggestion payload, kind, ordering, rank ordering |
| `CSSearchableIndexDelegate` | ✅ | Reindex, throttle, data/file-provider, searchable-items update callbacks, plus simulation helpers for tests |
| `CSIndexExtensionRequestHandler` | ✅ | Rust-backed subclass mirroring the delegate callback surface, plus simulation helpers |
| `CSImportExtension` | ✅ | Rust-backed subclass for `update(_:forFileAt:)`, plus simulation helpers |
| `DefaultIndexExtensionRequestHandler` | 🟡 | Test/demo helper implemented in the bridge; this is not an Apple SDK type |
| Version / domain / action / mailbox constants | ✅ | Core Spotlight version metadata, error domains, action/query/suggestion keys, and mailbox constants |

## Intentionally skipped

| Area | Status | Notes |
| --- | --- | --- |
| iOS-only / tvOS-unavailable APIs | ⏭️ | This crate targets macOS Core Spotlight bindings |
| Deprecated Swift 2/3 spellings | ⏭️ | The bridge uses current Swift overlay names |

## Validation

The following checks were used against v0.2.1 while expanding coverage:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for example in $(find examples -maxdepth 1 -name '*.rs' -exec basename {} .rs \; | sort); do
  cargo run --example "$example"
done
```
