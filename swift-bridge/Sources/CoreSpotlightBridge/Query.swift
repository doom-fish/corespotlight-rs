import CoreSpotlight
import Foundation

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
        var retainedItems: [UInt64] = []
        let semaphore = DispatchSemaphore(value: 0)
        var completionError: NSError?
        query.foundItemsHandler = { items in
            retainedItems.append(contentsOf: csRetainedItemPointers(items))
        }
        query.completionHandler = { error in
            completionError = error as NSError?
            semaphore.signal()
        }
        query.start()
        if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
            query.cancel()
            throw csBridgeNSError(code: CSR_TIMED_OUT, message: "Timed out waiting for CSSearchQuery")
        }
        if let completionError {
            throw completionError
        }
        let payload = CSSearchQueryExecutionPayload(
            itemPointers: retainedItems,
            foundItemCount: UInt64(query.foundItemCount),
            cancelled: query.isCancelled
        )
        outJSON?.pointee = csCString(try csEncodeJSON(payload))
        query.foundItemsHandler = nil
        query.completionHandler = nil
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
        var retainedItems: [UInt64] = []
        var retainedSuggestions: [UInt64] = []
        let semaphore = DispatchSemaphore(value: 0)
        var completionError: NSError?
        query.foundItemsHandler = { items in
            retainedItems.append(contentsOf: csRetainedItemPointers(items))
        }
        query.foundSuggestionsHandler = { suggestions in
            retainedSuggestions = csRetainedSuggestionPointers(suggestions)
        }
        query.completionHandler = { error in
            completionError = error as NSError?
            semaphore.signal()
        }
        query.start()
        if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
            query.cancel()
            throw csBridgeNSError(code: CSR_TIMED_OUT, message: "Timed out waiting for CSUserQuery")
        }
        if let completionError {
            throw completionError
        }
        let payload = CSUserQueryExecutionPayload(
            itemPointers: retainedItems,
            foundItemCount: UInt64(query.foundItemCount),
            suggestionPointers: retainedSuggestions,
            foundSuggestionCount: UInt64(query.foundSuggestionCount),
            cancelled: query.isCancelled
        )
        outJSON?.pointee = csCString(try csEncodeJSON(payload))
        query.foundItemsHandler = nil
        query.foundSuggestionsHandler = nil
        query.completionHandler = nil
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
    return UInt64(query.foundSuggestionCount)
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
