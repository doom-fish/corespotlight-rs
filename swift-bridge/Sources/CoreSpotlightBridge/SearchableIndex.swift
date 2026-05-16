import CoreSpotlight
import Foundation

private func csClientState(from json: UnsafePointer<CChar>?) throws -> Data {
    Data(try csDecodeJSON(json, as: [UInt8].self))
}

private func csClientStateJSON(_ data: Data?) throws -> String {
    try csEncodeJSON(data.map(Array.init) ?? [])
}

@_cdecl("cs_searchable_index_is_indexing_available")
public func csSearchableIndexIsIndexingAvailable() -> Int32 {
    CSSearchableIndex.isIndexingAvailable() ? 1 : 0
}

@_cdecl("cs_searchable_index_default_searchable_index")
public func csSearchableIndexDefaultSearchableIndex(
    _ outIndex: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outIndex?.pointee = csRetain(CSSearchableIndex.default())
    return CSR_OK
}

@_cdecl("cs_searchable_index_new")
public func csSearchableIndexNew(
    _ name: UnsafePointer<CChar>?,
    _ outIndex: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let name else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index name")
        }
        outIndex?.pointee = csRetain(CSSearchableIndex(name: String(cString: name)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_new_with_protection_class")
public func csSearchableIndexNewWithProtectionClass(
    _ name: UnsafePointer<CChar>?,
    _ protectionClass: UnsafePointer<CChar>?,
    _ outIndex: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let name else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index name")
        }
        let index = CSSearchableIndex(name: String(cString: name), protectionClass: csProtectionClass(from: protectionClass))
        outIndex?.pointee = csRetain(index)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_set_delegate")
public func csSearchableIndexSetDelegate(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        if let delegatePtr {
            let object = csBorrowAny(delegatePtr)
            guard let delegate = object as? CSSearchableIndexDelegate else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Object does not conform to CSSearchableIndexDelegate")
            }
            index.indexDelegate = delegate
        } else {
            index.indexDelegate = nil
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_index_items")
public func csSearchableIndexIndexItems(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ itemsJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let items = try csItems(from: itemsJSON)
        try csAwaitCompletion(label: "indexSearchableItems", timeoutSeconds: timeoutSeconds) { completion in
            index.indexSearchableItems(items) { error in
                completion(error as NSError?)
            }
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delete_identifiers")
public func csSearchableIndexDeleteIdentifiers(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let identifiers = try csDecodeJSON(identifiersJSON, as: [String].self)
        try csAwaitCompletion(label: "deleteSearchableItemsWithIdentifiers", timeoutSeconds: timeoutSeconds) { completion in
            index.deleteSearchableItems(withIdentifiers: identifiers) { error in
                completion(error as NSError?)
            }
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delete_domain_identifiers")
public func csSearchableIndexDeleteDomainIdentifiers(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let identifiers = try csDecodeJSON(identifiersJSON, as: [String].self)
        try csAwaitCompletion(label: "deleteSearchableItemsWithDomainIdentifiers", timeoutSeconds: timeoutSeconds) { completion in
            index.deleteSearchableItems(withDomainIdentifiers: identifiers) { error in
                completion(error as NSError?)
            }
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delete_all")
public func csSearchableIndexDeleteAll(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        try csAwaitCompletion(label: "deleteAllSearchableItems", timeoutSeconds: timeoutSeconds) { completion in
            index.deleteAllSearchableItems { error in
                completion(error as NSError?)
            }
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_begin_batch")
public func csSearchableIndexBeginBatch(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let indexPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let index: CSSearchableIndex = csBorrow(indexPtr)
    index.beginBatch()
    return CSR_OK
}

@_cdecl("cs_searchable_index_end_batch_with_client_state")
public func csSearchableIndexEndBatchWithClientState(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ clientStateJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let clientState = try csClientState(from: clientStateJSON)
        try csAwaitCompletion(label: "endIndexBatchWithClientState", timeoutSeconds: timeoutSeconds) { completion in
            index.endBatch(withClientState: clientState, completionHandler: { error in
                completion(error as NSError?)
            })
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_end_batch_with_expected_client_state")
public func csSearchableIndexEndBatchWithExpectedClientState(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ expectedClientStateJSON: UnsafePointer<CChar>?,
    _ newClientStateJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "endIndexBatch(expectedClientState:newClientState:) requires macOS 15.0")
        }
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        guard let newClientStateJSON else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing new client state")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let expectedClientState = try expectedClientStateJSON.map { try csClientState(from: $0) }
        guard expectedClientState == nil else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "endIndexBatch(expectedClientState:newClientState:) is not exposed by the current Swift overlay")
        }
        let newClientState = try csClientState(from: newClientStateJSON)
        try csAwaitCompletion(label: "endIndexBatchWithExpectedClientState", timeoutSeconds: timeoutSeconds) { completion in
            index.endBatch(withClientState: newClientState, completionHandler: { error in
                completion(error as NSError?)
            })
        }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_fetch_last_client_state")
public func csSearchableIndexFetchLastClientState(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ timeoutSeconds: Int32,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let semaphore = DispatchSemaphore(value: 0)
        var fetchedState: Data?
        var completionError: NSError?
        index.fetchLastClientState { clientState, error in
            fetchedState = clientState
            completionError = error as NSError?
            semaphore.signal()
        }
        if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
            throw csBridgeNSError(code: CSR_TIMED_OUT, message: "Timed out waiting for fetchLastClientState")
        }
        if let completionError {
            throw completionError
        }
        outJSON?.pointee = csCString(try csClientStateJSON(fetchedState))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_fetch_data_for_bundle_identifier")
public func csSearchableIndexFetchDataForBundleIdentifier(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ bundleIdentifier: UnsafePointer<CChar>?,
    _ itemIdentifier: UnsafePointer<CChar>?,
    _ contentType: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        guard let bundleIdentifier, let itemIdentifier, let contentType else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing external provider arguments")
        }
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let semaphore = DispatchSemaphore(value: 0)
        var fetchedData = Data()
        var completionError: NSError?
        index.fetchData(
            forBundleIdentifier: String(cString: bundleIdentifier),
            itemIdentifier: String(cString: itemIdentifier),
            contentType: csUTType(from: String(cString: contentType))
        ) { data, error in
            fetchedData = data ?? Data()
            completionError = error as NSError?
            semaphore.signal()
        }
        if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
            throw csBridgeNSError(code: CSR_TIMED_OUT, message: "Timed out waiting for fetchDataForBundleIdentifier")
        }
        if let completionError {
            throw completionError
        }
        outJSON?.pointee = csCString(try csEncodeJSON(Array(fetchedData)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
