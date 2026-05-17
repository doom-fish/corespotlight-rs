# corespotlight-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 53
VERIFIED: 52
GAPS: 0
EXEMPT: 1
COVERAGE_PCT: 100.0

Audit methodology: enumerated all public declarations in CoreSpotlight.framework headers (52 macOS-available top-level symbols across constants, enums, option sets, classes, protocols, and 16 categories). Cross-referenced against the crate's Swift bridge (`@_cdecl` thunks) and Rust safe API (`pub` items in `src/`). One symbol (CSActionIdentifier) is iOS-only with `API_UNAVAILABLE(macos)` and properly exempted. All 52 macOS symbols are either directly wrapped in the Rust API or exposed through swift-bridge FFI thunks; no gaps found.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CoreSpotlightVersionNumber | constant | CoreSpotlight.h | `core_spotlight_version_number()` |
| CoreSpotlightVersionString | constant | CoreSpotlight.h | `core_spotlight_version_string()` |
| CoreSpotlightAPIVersion | macro | CoreSpotlight.h | `core_spotlight_api_version()` |
| CSIndexErrorDomain | constant | CSSearchableIndex.h | `index_error_domain()` |
| CSIndexErrorCode | enum | CSSearchableIndex.h | `CSIndexErrorCode` |
| CSSearchableIndex | class | CSSearchableIndex.h | `CSSearchableIndex` |
| CSSearchableIndex (CSOptionalBatching) | category | CSSearchableIndex.h | `CSSearchableIndex::{begin_index_batch,end_index_batch_with_client_state,fetch_last_client_state}` |
| CSSearchableIndex (CSExternalProvider) | category | CSSearchableIndex.h | `CSSearchableIndex::fetch_data_for_bundle_identifier` |
| CSSearchableIndex (CSOptionalBatchingWithExpectedState) | category | CSSearchableIndex.h | `CSSearchableIndex::end_index_batch_with_expected_client_state` |
| CSSearchableIndexDelegate | protocol | CSSearchableIndex.h | `CSSearchableIndexDelegate`, `CSSearchableIndexDelegateCallbacks` |
| CSSearchableItemActionType | constant | CSSearchableItem.h | `searchable_item_action_type()` |
| CSSearchableItemActivityIdentifier | constant | CSSearchableItem.h | `searchable_item_activity_identifier()` |
| CSQueryContinuationActionType | constant | CSSearchableItem.h | `query_continuation_action_type()` |
| CSSearchQueryString | constant | CSSearchableItem.h | `search_query_string_key()` |
| CSSuggestionHighlightAttributeName | constant | CSSuggestion.h | `suggestion_highlight_attribute_name()` |
| CSSearchableItemUpdateListenerOptions | option set | CSSearchableItem.h | `CSSearchableItemUpdateListenerOptions` |
| CSSearchableItem | class | CSSearchableItem.h | `CSSearchableItem` |
| CSSearchQueryErrorDomain | constant | CSSearchQuery.h | `search_query_error_domain()` |
| CSSearchQueryErrorCode | enum | CSSearchQuery.h | `CSSearchQueryErrorCode` |
| CSSearchQuerySourceOptions | option set | CSSearchQuery.h | `CSSearchQuerySourceOptions` |
| CSSearchQueryContext | class | CSSearchQuery.h | `CSSearchQueryContext` |
| CSSearchQuery | class | CSSearchQuery.h | `CSSearchQuery` |
| CSUserInteraction | enum | CSUserQuery.h | `CSUserInteraction` |
| CSUserQueryContext | class | CSUserQuery.h | `CSUserQueryContext` |
| CSUserQuery | class | CSUserQuery.h | `CSUserQuery` |
| CSSuggestionKind | enum | CSSuggestion.h | `CSSuggestionKind` |
| CSSuggestion | class | CSSuggestion.h | `CSSuggestion` |
| CSSearchableItemAttributeSet | class | CSSearchableItemAttributeSet.h | `CSSearchableItemAttributeSet` |
| CSLocalizedString | class | CSSearchableItemAttributeSet.h | `CSLocalizedString` |
| CSCustomAttributeKey | class | CSSearchableItemAttributeSet.h | `CSCustomAttributeKey` |
| CSSearchableItemAttributeSet (CSCustomAttributes) | category | CSSearchableItemAttributeSet.h | `CSSearchableItemAttributeSet::{set_custom_value,custom_value}` |
| NSUserActivity (CSSearchableItemAttributeSet) | category | CSSearchableItemAttributeSet.h | `NSUserActivity` helper via bridge |
| CSSearchableItemAttributeSet (CSGeneral) | category | CSSearchableItemAttributeSet_General.h | generic field accessors plus convenience helpers |
| CSSearchableItemAttributeSet (CSActionExtras) | category | CSSearchableItemAttributeSet_General.h | generic number-field accessors |
| CSSearchableItemAttributeSet (CSContainment) | category | CSSearchableItemAttributeSet_General.h | generic string/number-field accessors |
| CSSearchableItemAttributeSet (CSItemProvider) | category | CSSearchableItemAttributeSet_General.h | generic string-array-field accessors |
| CSSearchableItemAttributeSet (CSDocuments) | category | CSSearchableItemAttributeSet_Documents.h | document field accessors plus `move_from` |
| CSSearchableItemAttributeSet (CSEvents) | category | CSSearchableItemAttributeSet_Events.h | generic date/number-field accessors |
| CSSearchableItemAttributeSet (CSMessaging) | category | CSSearchableItemAttributeSet_Messaging.h | generic string/person/map-field accessors |
| CSSearchableItemAttributeSet (CSMedia) | category | CSSearchableItemAttributeSet_Media.h | generic media field accessors |
| CSSearchableItemAttributeSet (CSMusic) | category | CSSearchableItemAttributeSet_Media.h | generic music field accessors |
| CSSearchableItemAttributeSet (CSImages) | category | CSSearchableItemAttributeSet_Images.h | generic image field accessors |
| CSSearchableItemAttributeSet (CSPlaces) | category | CSSearchableItemAttributeSet_Places.h | generic place field accessors |
| CSMailboxInbox | constant | CSSearchableItemAttributeSet_Messaging.h | `mailbox_inbox()` |
| CSMailboxDrafts | constant | CSSearchableItemAttributeSet_Messaging.h | `mailbox_drafts()` |
| CSMailboxSent | constant | CSSearchableItemAttributeSet_Messaging.h | `mailbox_sent()` |
| CSMailboxJunk | constant | CSSearchableItemAttributeSet_Messaging.h | `mailbox_junk()` |
| CSMailboxTrash | constant | CSSearchableItemAttributeSet_Messaging.h | `mailbox_trash()` |
| CSMailboxArchive | constant | CSSearchableItemAttributeSet_Messaging.h | `mailbox_archive()` |
| CSPerson | class | CSPerson.h | `CSPerson`, `CSPersonData` |
| CSImportExtension | class | CSImportExtension.h | `CSImportExtension` |
| CSIndexExtensionRequestHandler | class | CSIndexExtensionRequestHandler.h | `CSIndexExtensionRequestHandler` |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| CSActionIdentifier | constant | CSSearchableItem.h | iOS-only, unavailable on macOS | `API_AVAILABLE(ios(15.0)) API_UNAVAILABLE(macos, tvos)` |
