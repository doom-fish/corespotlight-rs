# Changelog

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
