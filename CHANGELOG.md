# Changelog

All notable changes to `corespotlight` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- The async index and delete APIs no longer read the index, the items or the
  JSON arguments after the call returns. The Swift bridge decoded them in a
  `Task` that ran later, so dropping the items before awaiting the future was
  a use-after-free from safe code. The bridge now retains and copies its
  inputs before returning, and the JSON C strings are no longer leaked on
  every call.

### Fixed

- `AsyncCSSearchableIndex::fetch_last_client_state` never returned the stored
  state: the bridge passed a boxed Swift object where Rust expected a C
  string, and Rust then freed it with `free`.
- Async errors keep the `NSError` domain and code.
- Batch misuse no longer aborts the process. A second `begin_index_batch`,
  ending a batch that isn't open, and batching the default index return
  errors instead of raising Objective-C exceptions.
- `end_index_batch_with_expected_client_state` calls
  `endIndexBatch(expectedClientState:newClientState:)` on macOS 15 and later
  instead of rejecting every expected state.
- `CSSearchQuery::execute` and `CSUserQuery::execute` release their results
  and clear their handlers on a timeout or error, no longer leak earlier
  suggestion deliveries, and return an error instead of aborting when a
  query is executed a second time.
- Reindex requests are acknowledged only after the Rust callback's work; see
  Changed.
- The default index extension request handler's counters and last
  identifiers are synchronized.
- Bridge errors with code 0 are no longer reported as success, and codes
  outside the `Int32` range no longer trap.
- Dates before 1970 read from attribute sets and item expiration dates no
  longer panic.
- The delegate and import-extension trampolines use the shared panic helpers
  and contain panicking context destructors.

### Changed

- **Breaking:** the reindex callbacks passed to
  `CSSearchableIndexDelegateCallbacks::new` receive a
  `CSReindexAcknowledgement` as their last argument. Call `acknowledge` or
  drop it once the reindex is done; dropping it while panicking doesn't
  acknowledge. `simulate_reindex_all` and `simulate_reindex_identifiers` wait
  for the acknowledgement and return an error when the callback finished
  without acknowledging.
- **Breaking:** the raw `ffi::CsDelegateReindexAll` and
  `ffi::CsDelegateReindexIdentifiers` callback types take an acknowledgement
  pointer.
- The `doom-fish-utils` requirement is `>=0.4.1, <0.5`.
- `rust-version` is 1.82 (was 1.76); `unsafe extern` blocks already needed it.

### Added

- `CSSearchQuery::escape_value` for values interpolated into query strings,
  and warnings in the docs of the query-string APIs.
- `CSReindexAcknowledgement` and `ffi::cs_reindex_acknowledgement_finish`.

## [0.3.8] - 2026-06-06

- The delegate and import-extension FFI trampolines catch panics from user
  callbacks instead of unwinding into Swift. Removed the empty bridge header.

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
