import CoreSpotlight
import Foundation

struct CSHighlightedRangePayload: Codable {
    var start: Int
    var length: Int
}

struct CSLocalizedSuggestionPayload: Codable {
    var string: String
    var highlightedRanges: [CSHighlightedRangePayload]
}

private func csSuggestionPayload(_ suggestion: CSSuggestion) -> CSLocalizedSuggestionPayload {
    let attributed = (suggestion.value(forKey: "localizedAttributedSuggestion") as? NSAttributedString) ?? NSAttributedString(string: "")
    var highlightedRanges: [CSHighlightedRangePayload] = []
    attributed.enumerateAttribute(.suggestionHighlight, in: NSRange(location: 0, length: attributed.length), options: []) { value, range, _ in
        guard value != nil else {
            return
        }
        highlightedRanges.append(CSHighlightedRangePayload(start: range.location, length: range.length))
    }
    return CSLocalizedSuggestionPayload(string: attributed.string, highlightedRanges: highlightedRanges)
}

@_cdecl("cs_core_spotlight_version_number")
public func csCoreSpotlightVersionNumber() -> Double {
    CoreSpotlightVersionNumber
}

@_cdecl("cs_core_spotlight_version_string")
public func csCoreSpotlightVersionString() -> UnsafeMutablePointer<CChar>? {
    csCString(String(describing: CoreSpotlightVersionNumber))
}

@_cdecl("cs_core_spotlight_api_version")
public func csCoreSpotlightAPIVersion() -> Int32 {
    CoreSpotlightAPIVersion
}

@_cdecl("cs_index_error_domain")
public func csIndexErrorDomain() -> UnsafeMutablePointer<CChar>? {
    csCString(CSIndexErrorDomain)
}

@_cdecl("cs_search_query_error_domain")
public func csSearchQueryErrorDomain() -> UnsafeMutablePointer<CChar>? {
    csCString(CSSearchQueryErrorDomain)
}

@_cdecl("cs_searchable_item_action_type")
public func csSearchableItemActionTypeKey() -> UnsafeMutablePointer<CChar>? {
    csCString(CSSearchableItemActionType)
}

@_cdecl("cs_searchable_item_activity_identifier")
public func csSearchableItemActivityIdentifierKey() -> UnsafeMutablePointer<CChar>? {
    csCString(CSSearchableItemActivityIdentifier)
}

@_cdecl("cs_query_continuation_action_type")
public func csQueryContinuationActionTypeKey() -> UnsafeMutablePointer<CChar>? {
    csCString(CSQueryContinuationActionType)
}

@_cdecl("cs_search_query_string_key")
public func csSearchQueryStringKey() -> UnsafeMutablePointer<CChar>? {
    csCString(CSSearchQueryString)
}

@_cdecl("cs_mailbox_inbox")
public func csMailboxInboxKey() -> UnsafeMutablePointer<CChar>? { csCString(CSMailboxInbox) }

@_cdecl("cs_mailbox_drafts")
public func csMailboxDraftsKey() -> UnsafeMutablePointer<CChar>? { csCString(CSMailboxDrafts) }

@_cdecl("cs_mailbox_sent")
public func csMailboxSentKey() -> UnsafeMutablePointer<CChar>? { csCString(CSMailboxSent) }

@_cdecl("cs_mailbox_junk")
public func csMailboxJunkKey() -> UnsafeMutablePointer<CChar>? { csCString(CSMailboxJunk) }

@_cdecl("cs_mailbox_trash")
public func csMailboxTrashKey() -> UnsafeMutablePointer<CChar>? { csCString(CSMailboxTrash) }

@_cdecl("cs_mailbox_archive")
public func csMailboxArchiveKey() -> UnsafeMutablePointer<CChar>? { csCString(CSMailboxArchive) }

@_cdecl("cs_search_query_context_new")
public func csSearchQueryContextNew(
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outContext?.pointee = csRetain(CSSearchQueryContext())
    return CSR_OK
}

@_cdecl("cs_search_query_context_get_fetch_attributes")
public func csSearchQueryContextGetFetchAttributes(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query context")
        }
        let context: CSSearchQueryContext = csBorrow(contextPtr)
        outJSON?.pointee = csCString(try csEncodeJSON(context.fetchAttributes))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_context_set_fetch_attributes")
public func csSearchQueryContextSetFetchAttributes(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query context")
        }
        let context: CSSearchQueryContext = csBorrow(contextPtr)
        context.fetchAttributes = try csDecodeJSON(valuesJSON, as: [String].self)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_context_get_filter_queries")
public func csSearchQueryContextGetFilterQueries(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query context")
        }
        let context: CSSearchQueryContext = csBorrow(contextPtr)
        outJSON?.pointee = csCString(try csEncodeJSON(context.filterQueries))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_context_set_filter_queries")
public func csSearchQueryContextSetFilterQueries(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query context")
        }
        let context: CSSearchQueryContext = csBorrow(contextPtr)
        context.filterQueries = try csDecodeJSON(valuesJSON, as: [String].self)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_context_get_keyboard_language")
public func csSearchQueryContextGetKeyboardLanguage(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let contextPtr else { return nil }
    let context: CSSearchQueryContext = csBorrow(contextPtr)
    return context.keyboardLanguage.flatMap(csCString)
}

@_cdecl("cs_search_query_context_set_keyboard_language")
public func csSearchQueryContextSetKeyboardLanguage(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let contextPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query context")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let context: CSSearchQueryContext = csBorrow(contextPtr)
    context.keyboardLanguage = value.map(String.init(cString:))
    return CSR_OK
}

@_cdecl("cs_search_query_context_get_source_options")
public func csSearchQueryContextGetSourceOptions(_ contextPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let contextPtr else { return 0 }
    let context: CSSearchQueryContext = csBorrow(contextPtr)
    return UInt64(context.sourceOptions.rawValue)
}

@_cdecl("cs_search_query_context_set_source_options")
public func csSearchQueryContextSetSourceOptions(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ value: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let contextPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query context")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let context: CSSearchQueryContext = csBorrow(contextPtr)
    context.sourceOptions = CSSearchQueryContext.SourceOptions(rawValue: UInt(value))
    return CSR_OK
}

@_cdecl("cs_user_query_context_new")
public func csUserQueryContextNew(
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outContext?.pointee = csRetain(CSUserQueryContext())
    return CSR_OK
}

@_cdecl("cs_user_query_context_with_current_suggestion")
public func csUserQueryContextWithCurrentSuggestion(
    _ suggestionPtr: UnsafeMutableRawPointer?,
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let suggestion = suggestionPtr.map { ptr -> CSSuggestion in
        csBorrow(ptr)
    }
    outContext?.pointee = csRetain(CSUserQueryContext(currentSuggestion: suggestion))
    return CSR_OK
}

@_cdecl("cs_user_query_context_get_enable_ranked_results")
public func csUserQueryContextGetEnableRankedResults(_ contextPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let contextPtr else { return 0 }
    let context: CSUserQueryContext = csBorrow(contextPtr)
    return context.enableRankedResults ? 1 : 0
}

@_cdecl("cs_user_query_context_set_enable_ranked_results")
public func csUserQueryContextSetEnableRankedResults(_ contextPtr: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let contextPtr else { return }
    let context: CSUserQueryContext = csBorrow(contextPtr)
    context.enableRankedResults = value != 0
}

@_cdecl("cs_user_query_context_get_disable_semantic_search")
public func csUserQueryContextGetDisableSemanticSearch(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "disableSemanticSearch requires macOS 15.0")
        }
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query context")
        }
        let context: CSUserQueryContext = csBorrow(contextPtr)
        outValue?.pointee = context.disableSemanticSearch ? 1 : 0
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_context_set_disable_semantic_search")
public func csUserQueryContextSetDisableSemanticSearch(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ value: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "disableSemanticSearch requires macOS 15.0")
        }
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query context")
        }
        let context: CSUserQueryContext = csBorrow(contextPtr)
        context.disableSemanticSearch = value != 0
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_context_get_max_result_count")
public func csUserQueryContextGetMaxResultCount(_ contextPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let contextPtr else { return 0 }
    let context: CSUserQueryContext = csBorrow(contextPtr)
    return Int64(context.maxResultCount)
}

@_cdecl("cs_user_query_context_set_max_result_count")
public func csUserQueryContextSetMaxResultCount(_ contextPtr: UnsafeMutableRawPointer?, _ value: Int64) {
    guard let contextPtr else { return }
    let context: CSUserQueryContext = csBorrow(contextPtr)
    context.maxResultCount = Int(value)
}

@_cdecl("cs_user_query_context_get_max_suggestion_count")
public func csUserQueryContextGetMaxSuggestionCount(_ contextPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let contextPtr else { return 0 }
    let context: CSUserQueryContext = csBorrow(contextPtr)
    return Int64(context.maxSuggestionCount)
}

@_cdecl("cs_user_query_context_set_max_suggestion_count")
public func csUserQueryContextSetMaxSuggestionCount(_ contextPtr: UnsafeMutableRawPointer?, _ value: Int64) {
    guard let contextPtr else { return }
    let context: CSUserQueryContext = csBorrow(contextPtr)
    context.maxSuggestionCount = Int(value)
}

@_cdecl("cs_user_query_context_get_max_ranked_result_count")
public func csUserQueryContextGetMaxRankedResultCount(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int64>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "maxRankedResultCount requires macOS 15.0")
        }
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query context")
        }
        let context: CSUserQueryContext = csBorrow(contextPtr)
        outValue?.pointee = Int64(context.maxRankedResultCount)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_context_set_max_ranked_result_count")
public func csUserQueryContextSetMaxRankedResultCount(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ value: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "maxRankedResultCount requires macOS 15.0")
        }
        guard let contextPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query context")
        }
        let context: CSUserQueryContext = csBorrow(contextPtr)
        context.maxRankedResultCount = Int(value)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_suggestion_get_localized_attributed_suggestion")
public func csSuggestionGetLocalizedAttributedSuggestion(
    _ suggestionPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let suggestionPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing suggestion")
        }
        let suggestion: CSSuggestion = csBorrow(suggestionPtr)
        outJSON?.pointee = csCString(try csEncodeJSON(csSuggestionPayload(suggestion)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_suggestion_get_kind")
public func csSuggestionGetKind(_ suggestionPtr: UnsafeMutableRawPointer?) -> Int64 {
    guard let suggestionPtr else { return 0 }
    let suggestion: CSSuggestion = csBorrow(suggestionPtr)
    return Int64(suggestion.suggestionKind.rawValue)
}

@_cdecl("cs_suggestion_compare")
public func csSuggestionCompare(_ suggestionPtr: UnsafeMutableRawPointer?, _ otherPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let suggestionPtr, let otherPtr else { return 0 }
    let suggestion: CSSuggestion = csBorrow(suggestionPtr)
    let other: CSSuggestion = csBorrow(otherPtr)
    switch suggestion.compare(other) {
    case .orderedAscending:
        return -1
    case .orderedDescending:
        return 1
    case .orderedSame:
        return 0
    @unknown default:
        return 0
    }
}

@_cdecl("cs_suggestion_compare_by_rank")
public func csSuggestionCompareByRank(_ suggestionPtr: UnsafeMutableRawPointer?, _ otherPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let suggestionPtr, let otherPtr else { return 0 }
    let suggestion: CSSuggestion = csBorrow(suggestionPtr)
    let other: CSSuggestion = csBorrow(otherPtr)
    switch suggestion.compare(byRank: other) {
    case .orderedAscending:
        return -1
    case .orderedDescending:
        return 1
    case .orderedSame:
        return 0
    @unknown default:
        return 0
    }
}
