import CoreSpotlight
import CoreSpotlightObjCBridge
import Foundation

private final class CSQueryRun {
    private let lock = NSLock()
    private let semaphore = DispatchSemaphore(value: 0)
    private var open = true
    private var items: [CSSearchableItem] = []
    private var suggestions: [CSSuggestion] = []
    private var completionError: NSError?

    func append(_ newItems: [CSSearchableItem]) {
        lock.lock()
        defer { lock.unlock() }
        if open {
            items.append(contentsOf: newItems)
        }
    }

    func replaceSuggestions(_ newSuggestions: [CSSuggestion]) {
        lock.lock()
        defer { lock.unlock() }
        if open {
            suggestions = newSuggestions
        }
    }

    func complete(_ error: Error?) {
        lock.lock()
        if open {
            completionError = error as NSError?
        }
        lock.unlock()
        semaphore.signal()
    }

    func wait(timeoutSeconds: Int32) -> Bool {
        semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) != .timedOut
    }

    func close() -> (items: [CSSearchableItem], suggestions: [CSSuggestion], error: NSError?) {
        lock.lock()
        defer { lock.unlock() }
        open = false
        let result = (items, suggestions, completionError)
        items = []
        suggestions = []
        completionError = nil
        return result
    }
}

private func csRunQuery(_ query: CSSearchQuery, run: CSQueryRun, label: String, timeoutSeconds: Int32) throws -> (items: [CSSearchableItem], suggestions: [CSSuggestion]) {
    var exceptionError: NSError?
    let started = CSXTryStartQuery(query, &exceptionError)
    let finished = started && run.wait(timeoutSeconds: timeoutSeconds)
    if started && !finished {
        query.cancel()
    }
    query.foundItemsHandler = nil
    query.completionHandler = nil
    if let userQuery = query as? CSUserQuery {
        userQuery.foundSuggestionsHandler = nil
    }
    let result = run.close()
    guard started else {
        throw exceptionError ?? csBridgeNSError(code: CSR_FAILURE, message: "Failed to start \(label)")
    }
    guard finished else {
        throw csBridgeNSError(code: CSR_TIMED_OUT, message: "Timed out waiting for \(label)")
    }
    if let error = result.error {
        throw error
    }
    return (result.items, result.suggestions)
}

private func csProtectionClasses(from json: UnsafePointer<CChar>?) throws -> [FileProtectionType] {
    try csDecodeJSON(json, as: [String].self).map(FileProtectionType.init(rawValue:))
}

private func csProtectionClassesJSON(_ value: [FileProtectionType]) throws -> String {
    try csEncodeJSON(value.map(\.rawValue))
}

@_cdecl("cs_search_query_new")
public func csSearchQueryNew(
    _ queryString: UnsafePointer<CChar>?,
    _ queryContextPtr: UnsafeMutableRawPointer?,
    _ outQuery: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let queryString else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query string")
        }
        let queryContext = queryContextPtr.map { ptr -> CSSearchQueryContext in csBorrow(ptr) }
        let query = CSSearchQuery(queryString: String(cString: queryString), queryContext: queryContext)
        outQuery?.pointee = csRetain(query)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_new_with_attributes")
public func csSearchQueryNewWithAttributes(
    _ queryString: UnsafePointer<CChar>?,
    _ attributesJSON: UnsafePointer<CChar>?,
    _ outQuery: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let queryString else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query string")
        }
        let attributes = try csDecodeJSON(attributesJSON, as: [String].self)
        let query = CSSearchQuery(queryString: String(cString: queryString), attributes: attributes)
        outQuery?.pointee = csRetain(query)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_execute")
public func csSearchQueryExecute(
    _ queryPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let queryPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query")
        }
        let query: CSSearchQuery = csBorrow(queryPtr)
        let run = CSQueryRun()
        query.foundItemsHandler = { [run] items in
            run.append(items)
        }
        query.completionHandler = { [run] error in
            run.complete(error)
        }
        let result = try csRunQuery(query, run: run, label: "CSSearchQuery", timeoutSeconds: timeoutSeconds)
        let foundItemCount = UInt64(query.foundItemCount)
        let cancelled = query.isCancelled
        let itemPointers = csRetainedItemPointers(result.items)
        let json: String
        do {
            json = try csEncodeJSON(CSSearchQueryExecutionPayload(
                itemPointers: itemPointers,
                foundItemCount: foundItemCount,
                cancelled: cancelled
            ))
        } catch {
            csReleaseRetainedPointers(itemPointers)
            throw error
        }
        outJSON?.pointee = csCString(json)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_cancel")
public func csSearchQueryCancel(_ queryPtr: UnsafeMutableRawPointer?) {
    guard let queryPtr else { return }
    let query: CSSearchQuery = csBorrow(queryPtr)
    query.cancel()
}

@_cdecl("cs_search_query_is_cancelled")
public func csSearchQueryIsCancelled(_ queryPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let queryPtr else { return 0 }
    let query: CSSearchQuery = csBorrow(queryPtr)
    return query.isCancelled ? 1 : 0
}

@_cdecl("cs_search_query_found_item_count")
public func csSearchQueryFoundItemCount(_ queryPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let queryPtr else { return 0 }
    let query: CSSearchQuery = csBorrow(queryPtr)
    return UInt64(query.foundItemCount)
}

@_cdecl("cs_search_query_get_protection_classes")
public func csSearchQueryGetProtectionClasses(
    _ queryPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let queryPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query")
        }
        let query: CSSearchQuery = csBorrow(queryPtr)
        outJSON?.pointee = csCString(try csProtectionClassesJSON(query.protectionClasses))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_search_query_set_protection_classes")
public func csSearchQuerySetProtectionClasses(
    _ queryPtr: UnsafeMutableRawPointer?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let queryPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing search query")
        }
        let query: CSSearchQuery = csBorrow(queryPtr)
        query.protectionClasses = try csProtectionClasses(from: valuesJSON)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_prepare")
public func csUserQueryPrepare(_ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "CSUserQuery.prepare requires macOS 15.0")
        }
        CSUserQuery.prepare()
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_prepare_protection_classes")
public func csUserQueryPrepareProtectionClasses(
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "CSUserQuery.prepareProtectionClasses requires macOS 15.0")
        }
        CSUserQuery.prepareProtectionClasses(try csProtectionClasses(from: valuesJSON))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_new")
public func csUserQueryNew(
    _ userQueryString: UnsafePointer<CChar>?,
    _ userQueryContextPtr: UnsafeMutableRawPointer?,
    _ outQuery: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let userQueryContext = userQueryContextPtr.map { ptr -> CSUserQueryContext in csBorrow(ptr) }
        let query = CSUserQuery(userQueryString: userQueryString.map(String.init(cString:)), userQueryContext: userQueryContext)
        outQuery?.pointee = csRetain(query)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_execute")
public func csUserQueryExecute(
    _ queryPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let queryPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query")
        }
        let query: CSUserQuery = csBorrow(queryPtr)
        let run = CSQueryRun()
        query.foundItemsHandler = { [run] items in
            run.append(items)
        }
        query.foundSuggestionsHandler = { [run] suggestions in
            run.replaceSuggestions(suggestions)
        }
        query.completionHandler = { [run] error in
            run.complete(error)
        }
        let result = try csRunQuery(query, run: run, label: "CSUserQuery", timeoutSeconds: timeoutSeconds)
        let foundItemCount = UInt64(query.foundItemCount)
        let foundSuggestionCount = UInt64(clamping: query.foundSuggestionCount)
        let cancelled = query.isCancelled
        let itemPointers = csRetainedItemPointers(result.items)
        let suggestionPointers = csRetainedSuggestionPointers(result.suggestions)
        let json: String
        do {
            json = try csEncodeJSON(CSUserQueryExecutionPayload(
                itemPointers: itemPointers,
                foundItemCount: foundItemCount,
                suggestionPointers: suggestionPointers,
                foundSuggestionCount: foundSuggestionCount,
                cancelled: cancelled
            ))
        } catch {
            csReleaseRetainedPointers(itemPointers + suggestionPointers)
            throw error
        }
        outJSON?.pointee = csCString(json)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_found_suggestion_count")
public func csUserQueryFoundSuggestionCount(_ queryPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let queryPtr else { return 0 }
    let query: CSUserQuery = csBorrow(queryPtr)
    return UInt64(clamping: query.foundSuggestionCount)
}

@_cdecl("cs_user_query_user_engaged_with_item")
public func csUserQueryUserEngagedWithItem(
    _ queryPtr: UnsafeMutableRawPointer?,
    _ itemPtr: UnsafeMutableRawPointer?,
    _ visibleItemsJSON: UnsafePointer<CChar>?,
    _ interaction: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "CSUserQuery.user engagement requires macOS 15.0")
        }
        guard queryPtr != nil, itemPtr != nil else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query item engagement arguments")
        }
        throw csBridgeNSError(code: CSR_FAILURE, message: "CSUserQuery item engagement is not bridged by corespotlight-rs v0.2.0")
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_query_user_engaged_with_suggestion")
public func csUserQueryUserEngagedWithSuggestion(
    _ queryPtr: UnsafeMutableRawPointer?,
    _ suggestionPtr: UnsafeMutableRawPointer?,
    _ visibleSuggestionsJSON: UnsafePointer<CChar>?,
    _ interaction: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "CSUserQuery.user engagement requires macOS 15.0")
        }
        guard queryPtr != nil, suggestionPtr != nil else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user query suggestion engagement arguments")
        }
        throw csBridgeNSError(code: CSR_FAILURE, message: "CSUserQuery suggestion engagement is not bridged by corespotlight-rs v0.2.0")
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
