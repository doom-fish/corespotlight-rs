# CoreSpotlight coverage (v0.4.0)

Legend: ✅ implemented and exercised by examples/tests, 🟡 partially implemented or has a runtime caveat, ⏭️ intentionally skipped.

## Public SDK surface

| SDK area | Status | Notes |
| --- | --- | --- |
| `CSSearchableIndex` | ✅ | Default/custom indexes, deletion APIs, batching, last-client-state fetch, external-provider fetch, delegate attachment, request-handler attachment. Batch misuse (a second `begin`, `end` without an open batch, batching the default index) returns an error instead of raising an Objective-C exception |
| `CSSearchableIndex::endIndexBatch(expectedClientState:newClientState:)` | ✅ | `end_index_batch_with_expected_client_state` on macOS 15 and later; a mismatch returns `CSIndexErrorCode::MismatchedClientState`. Earlier systems get an error |
| `CSSearchableItem` | ✅ | Construction, rank comparison, mutable identifiers, expiration dates, `isUpdate`, and `updateListenerOptions` |
| `CSSearchableItemAttributeSet` | ✅ | Typed string / array / number / URL / data / date / date-array / people / map accessors, common convenience fields, localized strings, people, and custom attribute values |
| `NSUserActivity (CSSearchableItemAttributeSet)` | ✅ | `contentAttributeSet` attachment and retrieval for Spotlight-backed activities |
| `CSLocalizedString` | ✅ | Creation and retrieval |
| `CSPerson` | ✅ | Creation and round-trip support through person-array attribute fields |
| `CSCustomAttributeKey` | 🟡 | Rust API is exposed, but Apple validates custom key names against the active bundle identifier at runtime; command-line demos may fail to create keys even with syntactically valid names |
| `CSSearchQuery` | ✅ | Construction, attribute-limited construction, execute (once per query), cancel, item counts, protection classes, and `escape_value` for values interpolated into query strings |
| `CSUserQuery` | 🟡 | Construction, execute (once per query), cancel, item/suggestion counts, protection classes; `user_engaged_with_item` and `user_engaged_with_suggestion` always return an error, because Core Spotlight crashes when it's given an item or suggestion the query didn't return and the bridge can't check that |
| `CSSearchQueryContext` | ✅ | Fetch attributes, filter queries, keyboard language, source options |
| `CSUserQueryContext` | ✅ | Current suggestion, ranked-results toggle, semantic-search toggle, max result/suggestion/ranked counts |
| `CSSuggestion` | ✅ | Localized attributed suggestion payload, kind, ordering, rank ordering |
| `CSSearchableIndexDelegate` | ✅ | Reindex (acknowledged through `CSReindexAcknowledgement`), throttle, data/file-provider, searchable-items update callbacks, plus simulation helpers for tests |
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

The following checks were run against v0.4.0:

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
for example in $(find examples -maxdepth 1 -name '*.rs' -exec basename {} .rs \; | sort); do
  cargo run --all-features --example "$example"
done
```
