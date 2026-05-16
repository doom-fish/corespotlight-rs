# corespotlight coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 52
VERIFIED: 48
GAPS: 4
EXEMPT: 0
COVERAGE_PCT: 92.3%

Audit scope: top-level public CoreSpotlight declarations (classes, protocols, categories, exported constants, enums / option sets) plus the public `CoreSpotlightAPIVersion` macro.
Filtered out from totals: symbols explicitly unavailable on macOS, such as `CSActionIdentifier`, plus iOS-only members inside otherwise-macOS categories.
No top-level macOS-deprecated declarations were found in the counted symbol set, so `EXEMPT` is 0.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CoreSpotlightVersionNumber | constant | CoreSpotlight.h | `core_spotlight_version_number()` |
| CoreSpotlightAPIVersion | macro | CoreSpotlight.h | `core_spotlight_api_version()` |
| CSIndexErrorDomain | constant | CSSearchableIndex.h | `index_error_domain()` |
| CSIndexErrorCode | enum | CSSearchableIndex.h | `CSIndexErrorCode` |
| CSSearchableIndex | class | CSSearchableIndex.h | `CSSearchableIndex` |
| CSSearchableIndex (CSOptionalBatching) | category | CSSearchableIndex.h | `CSSearchableIndex::{begin_index_batch,end_index_batch_with_client_state,fetch_last_client_state}` |
| CSSearchableIndex (CSExternalProvider) | category | CSSearchableIndex.h | `CSSearchableIndex::fetch_data_for_bundle_identifier` |
| CSSearchableIndex (CSOptionalBatchingWithExpectedState) | category | CSSearchableIndex.h | `CSSearchableIndex::end_index_batch_with_expected_client_state` (publicly exposed, but the current bridge rejects non-`None` expected state values) |
| CSSearchableIndexDelegate | protocol | CSSearchableIndex.h | `CSSearchableIndexDelegate`, `CSSearchableIndexDelegateCallbacks` |
| CSSearchableItemActionType | constant | CSSearchableItem.h | `searchable_item_action_type()` |
| CSSearchableItemActivityIdentifier | constant | CSSearchableItem.h | `searchable_item_activity_identifier()` |
| CSQueryContinuationActionType | constant | CSSearchableItem.h | `query_continuation_action_type()` |
| CSSearchQueryString | constant | CSSearchableItem.h | `search_query_string_key()` |
| CSSearchableItemUpdateListenerOptions | option set | CSSearchableItem.h | `CSSearchableItemUpdateListenerOptions` |
| CSSearchableItem | class | CSSearchableItem.h | `CSSearchableItem::{new,compare_by_rank,attribute_set,...}` |
| CSSearchQueryErrorDomain | constant | CSSearchQuery.h | `search_query_error_domain()` |
| CSSearchQueryErrorCode | enum | CSSearchQuery.h | `CSSearchQueryErrorCode` |
| CSSearchQuerySourceOptions | option set | CSSearchQuery.h | `CSSearchQuerySourceOptions` |
| CSSearchQueryContext | class | CSSearchQuery.h | `CSSearchQueryContext` |
| CSSearchQuery | class | CSSearchQuery.h | `CSSearchQuery::{new,new_with_attributes,execute,cancel,...}` |
| CSUserInteraction | enum | CSUserQuery.h | `CSUserInteraction` |
| CSUserQueryContext | class | CSUserQuery.h | `CSUserQueryContext` |
| CSUserQuery | class | CSUserQuery.h | `CSUserQuery::{prepare,prepare_protection_classes,new,execute,cancel,...}` (the `user_engaged_with_*` helpers are exposed but currently return bridge errors on the command-line bridge) |
| CSSuggestionKind | enum | CSSuggestion.h | `CSSuggestionKind` |
| CSSuggestion | class | CSSuggestion.h | `CSSuggestion::{localized_attributed_suggestion,suggestion_kind,compare,compare_by_rank}` |
| CSSearchableItemAttributeSet | class | CSSearchableItemAttributeSet.h | `CSSearchableItemAttributeSet` |
| CSLocalizedString | class | CSSearchableItemAttributeSet.h | `CSLocalizedString::{new,localized_string}` |
| CSCustomAttributeKey | class | CSSearchableItemAttributeSet.h | `CSCustomAttributeKey::{new,new_with_options,...}` |
| CSSearchableItemAttributeSet (CSCustomAttributes) | category | CSSearchableItemAttributeSet.h | `CSSearchableItemAttributeSet::{set_custom_value,custom_value}` |
| CSSearchableItemAttributeSet (CSGeneral) | category | CSSearchableItemAttributeSet_General.h | generic field accessors plus `CSSearchableItemAttribute{String,StringArray,Number,URL,Data,Date}Field` and convenience helpers like `set_title` / `set_display_name` |
| CSSearchableItemAttributeSet (CSActionExtras) | category | CSSearchableItemAttributeSet_General.h | generic number-field accessors for `supportsPhoneCall` / `supportsNavigation` (iOS-only members filtered out) |
| CSSearchableItemAttributeSet (CSContainment) | category | CSSearchableItemAttributeSet_General.h | generic string/number-field accessors for the containment fields |
| CSSearchableItemAttributeSet (CSItemProvider) | category | CSSearchableItemAttributeSet_General.h | generic string-array-field accessors for `provider*TypeIdentifiers` |
| CSSearchableItemAttributeSet (CSDocuments) | category | CSSearchableItemAttributeSet_Documents.h | `CSSearchableItemAttributeSet::move_from` plus document field enums |
| CSSearchableItemAttributeSet (CSEvents) | category | CSSearchableItemAttributeSet_Events.h | generic date/date-array/number-field accessors for event fields |
| CSSearchableItemAttributeSet (CSMessaging) | category | CSSearchableItemAttributeSet_Messaging.h | generic string/string-array/person/map/read-only-field accessors for messaging fields |
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
| CSPerson | class | CSPerson.h | `CSPerson::{new,contact_identifier,to_data,...}` and `CSPersonData` |
| CSIndexExtensionRequestHandler | class | CSIndexExtensionRequestHandler.h | `CSIndexExtensionRequestHandler::{new,simulate_*}` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| CoreSpotlightVersionString | constant | CoreSpotlight.h | `core_spotlight_version_string()` exists, but the Swift bridge formats `CoreSpotlightVersionNumber` instead of reading `CoreSpotlightVersionString`, so this exported symbol is not actually wrapped. |
| CSSuggestionHighlightAttributeName | constant | CSSuggestion.h | Highlight ranges are decoded internally for `LocalizedSuggestion`, but the public constant / key itself is not exposed through the Rust API. |
| NSUserActivity (CSSearchableItemAttributeSet) | category | CSSearchableItemAttributeSet.h | No Rust wrapper for `NSUserActivity.contentAttributeSet`; the crate only exposes `CSSearchableItemAttributeSet` directly. |
| CSImportExtension | class | CSImportExtension.h | The import-extension request-handler API is not surfaced in `src/` or the Swift bridge. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
