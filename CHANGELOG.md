# Changelog

## [0.3.7] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.6] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.5] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.4] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## [0.3.3] - 2026-05-18

- Added rustdoc comments across the public Rust API surface outside `src/ffi/`, bringing the doc pass to full coverage for that audited surface.
- Made `CSSearchableItemAttributeSet::move_from` degrade gracefully on SDKs where the legacy selector is unavailable instead of failing the Swift bridge build.
- Marked the async delete-all example as requiring the `async` feature so default `cargo test` runs cleanly.

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## 0.3.1 - 2026-05-17

- Quality pass: added panic safety wrapping to all FFI callbacks in async API to prevent panics from unwinding across the C ABI boundary.
- Added detailed SAFETY comments to all unsafe blocks in async API explaining the safety invariants and assumptions.
- Added thread-safety documentation to async future types explaining Send/Sync guarantees.

## 0.3.0 - 2026-05-17

- Added `async` feature gate with async API module (`async_api`).
- Implemented async wrappers for all completion-handler APIs:
  - `AsyncCSSearchableIndex::index_searchable_items` — async indexing
  - `AsyncCSSearchableIndex::delete_searchable_items_with_identifiers` — async deletion by ID
  - `AsyncCSSearchableIndex::delete_searchable_items_with_domain_identifiers` — async deletion by domain
  - `AsyncCSSearchableIndex::delete_all_searchable_items` — async deletion of all items
  - `AsyncCSSearchableIndex::fetch_last_client_state` — async state fetch (macOS 13+)
- All async APIs are executor-agnostic and work with any async runtime.

## 0.2.1 - 2026-05-16

- Fixed `core_spotlight_version_string()` to read the exported `CoreSpotlightVersionString` symbol.
- Added `suggestion_highlight_attribute_name()`, `NSUserActivity`, and `CSImportExtension` coverage with runnable example/test support.
- Closed the remaining `COVERAGE_AUDIT.md` gaps, bringing the audited top-level Core Spotlight surface to 100% coverage.

## 0.2.0 - 2026-05-16

- Expanded `CSSearchableIndex` with batching, client-state helpers, external-provider fetch, and delegate / request-handler attachment.
- Expanded `CSSearchableItem` with rank comparison, mutable identifiers, update flags, and update-listener options.
- Added broad `CSSearchableItemAttributeSet` coverage through typed field enums, localized strings, people, and custom attribute values.
- Added `CSSearchQuery`, `CSUserQuery`, `CSSearchQueryContext`, `CSUserQueryContext`, `CSSuggestion`, and version / domain / mailbox constants.
- Added `CSSearchableIndexDelegate`, `CSIndexExtensionRequestHandler`, and the test-helper `DefaultIndexExtensionRequestHandler`.
- Added numbered examples, integration tests, and `COVERAGE.md`.

## 0.1.0 - 2026-05-16

- Initial release.
- Added searchable indexes, searchable items, and searchable item attribute sets.
- Added a Spotlight smoke example covering index + delete.
