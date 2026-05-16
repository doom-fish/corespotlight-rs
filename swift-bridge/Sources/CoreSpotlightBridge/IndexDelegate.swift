import CoreSpotlight
import Foundation

public typealias CSDelegateReleaseContextCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias CSDelegateReindexAllCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
public typealias CSDelegateReindexIdentifiersCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void
public typealias CSDelegateNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
public typealias CSDelegateDataForItemCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32
public typealias CSDelegateFileURLForItemCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
    Int32,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32
public typealias CSDelegateSearchableItemsForIdentifiersCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32
public typealias CSDelegateSearchableItemsDidUpdateCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

final class CSRustSearchableIndexDelegateBox {
    let context: UnsafeMutableRawPointer?
    let releaseContext: CSDelegateReleaseContextCallback?
    let reindexAll: CSDelegateReindexAllCallback?
    let reindexIdentifiers: CSDelegateReindexIdentifiersCallback?
    let didThrottle: CSDelegateNotificationCallback?
    let didFinishThrottle: CSDelegateNotificationCallback?
    let dataForItem: CSDelegateDataForItemCallback?
    let fileURLForItem: CSDelegateFileURLForItemCallback?
    let searchableItemsForIdentifiers: CSDelegateSearchableItemsForIdentifiersCallback?
    let searchableItemsDidUpdate: CSDelegateSearchableItemsDidUpdateCallback?

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: CSDelegateReleaseContextCallback?,
        reindexAll: CSDelegateReindexAllCallback?,
        reindexIdentifiers: CSDelegateReindexIdentifiersCallback?,
        didThrottle: CSDelegateNotificationCallback?,
        didFinishThrottle: CSDelegateNotificationCallback?,
        dataForItem: CSDelegateDataForItemCallback?,
        fileURLForItem: CSDelegateFileURLForItemCallback?,
        searchableItemsForIdentifiers: CSDelegateSearchableItemsForIdentifiersCallback?,
        searchableItemsDidUpdate: CSDelegateSearchableItemsDidUpdateCallback?
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.reindexAll = reindexAll
        self.reindexIdentifiers = reindexIdentifiers
        self.didThrottle = didThrottle
        self.didFinishThrottle = didFinishThrottle
        self.dataForItem = dataForItem
        self.fileURLForItem = fileURLForItem
        self.searchableItemsForIdentifiers = searchableItemsForIdentifiers
        self.searchableItemsDidUpdate = searchableItemsDidUpdate
    }

    deinit {
        releaseContext?(context)
    }
}

final class CSRustSearchableIndexDelegate: NSObject, CSSearchableIndexDelegate {
    let box: CSRustSearchableIndexDelegateBox

    init(box: CSRustSearchableIndexDelegateBox) {
        self.box = box
    }

    func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexAllSearchableItemsWithAcknowledgementHandler acknowledgementHandler: @escaping () -> Void) {
        box.reindexAll?(box.context, csRetain(searchableIndex))
        acknowledgementHandler()
    }

    func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexSearchableItemsWithIdentifiers identifiers: [String], acknowledgementHandler: @escaping () -> Void) {
        if let callback = box.reindexIdentifiers, let identifiersJSON = try? csEncodeJSON(identifiers) {
            identifiersJSON.withCString { callback(box.context, csRetain(searchableIndex), $0) }
        }
        acknowledgementHandler()
    }

    func searchableIndexDidThrottle(_ searchableIndex: CSSearchableIndex) {
        box.didThrottle?(box.context, csRetain(searchableIndex))
    }

    func searchableIndexDidFinishThrottle(_ searchableIndex: CSSearchableIndex) {
        box.didFinishThrottle?(box.context, csRetain(searchableIndex))
    }

    func data(for searchableIndex: CSSearchableIndex, itemIdentifier: String, typeIdentifier: String) throws -> Data {
        guard let callback = box.dataForItem else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Delegate data callback not configured")
        }
        var outJSON: UnsafeMutablePointer<CChar>?
        var callbackError: UnsafeMutablePointer<CChar>?
        let status = itemIdentifier.withCString { itemIdentifierCString in
            typeIdentifier.withCString { typeIdentifierCString in
                callback(box.context, csRetain(searchableIndex), itemIdentifierCString, typeIdentifierCString, &outJSON, &callbackError)
            }
        }
        guard status == CSR_OK else {
            throw csNSError(fromJSONCString: callbackError)
        }
        guard let outJSON else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Delegate data callback returned no payload")
        }
        defer { csStringFree(outJSON) }
        let bytes = try csDecodeJSON(outJSON, as: [UInt8].self)
        return Data(bytes)
    }

    func fileURL(for searchableIndex: CSSearchableIndex, itemIdentifier: String, typeIdentifier: String, inPlace: Bool) throws -> URL {
        guard let callback = box.fileURLForItem else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Delegate file URL callback not configured")
        }
        var outURL: UnsafeMutablePointer<CChar>?
        var callbackError: UnsafeMutablePointer<CChar>?
        let status = itemIdentifier.withCString { itemIdentifierCString in
            typeIdentifier.withCString { typeIdentifierCString in
                callback(box.context, csRetain(searchableIndex), itemIdentifierCString, typeIdentifierCString, inPlace ? 1 : 0, &outURL, &callbackError)
            }
        }
        guard status == CSR_OK else {
            throw csNSError(fromJSONCString: callbackError)
        }
        guard let outURL else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Delegate file URL callback returned no URL")
        }
        defer { csStringFree(outURL) }
        guard let url = URL(string: String(cString: outURL)) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Delegate file URL callback returned an invalid URL")
        }
        return url
    }

    @available(macOS 15.4, *)
    func searchableItems(forIdentifiers identifiers: [String], searchableItemsHandler: @escaping ([CSSearchableItem]) -> Void) {
        guard let callback = box.searchableItemsForIdentifiers, let identifiersJSON = try? csEncodeJSON(identifiers) else {
            searchableItemsHandler([])
            return
        }
        var outJSON: UnsafeMutablePointer<CChar>?
        var callbackError: UnsafeMutablePointer<CChar>?
        let status = identifiersJSON.withCString { callback(box.context, $0, &outJSON, &callbackError) }
        guard status == CSR_OK, let outJSON else {
            searchableItemsHandler([])
            return
        }
        defer { csStringFree(outJSON) }
        let items = (try? csItems(from: outJSON)) ?? []
        searchableItemsHandler(items)
    }

    @available(macOS 15.4, *)
    func searchableItemsDidUpdate(_ items: [CSSearchableItem]) {
        guard let callback = box.searchableItemsDidUpdate, let itemsJSON = try? csEncodeJSON(csRetainedItemPointers(items)) else {
            return
        }
        itemsJSON.withCString { callback(box.context, $0) }
    }
}

func csDelegateObject(_ objectPtr: UnsafeMutableRawPointer?) throws -> CSSearchableIndexDelegate {
    guard let objectPtr else {
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index delegate")
    }
    let object = csBorrowAny(objectPtr)
    guard let delegate = object as? CSSearchableIndexDelegate else {
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Object does not conform to CSSearchableIndexDelegate")
    }
    return delegate
}

@_cdecl("cs_searchable_index_delegate_new")
public func csSearchableIndexDelegateNew(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: CSDelegateReleaseContextCallback?,
    _ reindexAll: CSDelegateReindexAllCallback?,
    _ reindexIdentifiers: CSDelegateReindexIdentifiersCallback?,
    _ didThrottle: CSDelegateNotificationCallback?,
    _ didFinishThrottle: CSDelegateNotificationCallback?,
    _ dataForItem: CSDelegateDataForItemCallback?,
    _ fileURLForItem: CSDelegateFileURLForItemCallback?,
    _ searchableItemsForIdentifiers: CSDelegateSearchableItemsForIdentifiersCallback?,
    _ searchableItemsDidUpdate: CSDelegateSearchableItemsDidUpdateCallback?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard reindexAll != nil, reindexIdentifiers != nil else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Searchable index delegate requires reindex callbacks")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let delegate = CSRustSearchableIndexDelegate(box: CSRustSearchableIndexDelegateBox(
        context: context,
        releaseContext: releaseContext,
        reindexAll: reindexAll,
        reindexIdentifiers: reindexIdentifiers,
        didThrottle: didThrottle,
        didFinishThrottle: didFinishThrottle,
        dataForItem: dataForItem,
        fileURLForItem: fileURLForItem,
        searchableItemsForIdentifiers: searchableItemsForIdentifiers,
        searchableItemsDidUpdate: searchableItemsDidUpdate
    ))
    outDelegate?.pointee = csRetain(delegate)
    return CSR_OK
}

@_cdecl("cs_searchable_index_delegate_simulate_reindex_all")
public func csSearchableIndexDelegateSimulateReindexAll(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ indexPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let index: CSSearchableIndex = csBorrow(indexPtr)
        delegate.searchableIndex(index, reindexAllSearchableItemsWithAcknowledgementHandler: {})
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_reindex_identifiers")
public func csSearchableIndexDelegateSimulateReindexIdentifiers(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ indexPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let identifiers = try csDecodeJSON(identifiersJSON, as: [String].self)
        delegate.searchableIndex(index, reindexSearchableItemsWithIdentifiers: identifiers, acknowledgementHandler: {})
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_did_throttle")
public func csSearchableIndexDelegateSimulateDidThrottle(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ indexPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let index: CSSearchableIndex = csBorrow(indexPtr)
        delegate.searchableIndexDidThrottle?(index)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_did_finish_throttle")
public func csSearchableIndexDelegateSimulateDidFinishThrottle(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ indexPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let index: CSSearchableIndex = csBorrow(indexPtr)
        delegate.searchableIndexDidFinishThrottle?(index)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_data_request")
public func csSearchableIndexDelegateSimulateDataRequest(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ indexPtr: UnsafeMutableRawPointer?,
    _ itemIdentifier: UnsafePointer<CChar>?,
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr, let itemIdentifier, let typeIdentifier else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index data request arguments")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let itemIdentifierString = String(cString: itemIdentifier)
        let typeIdentifierString = String(cString: typeIdentifier)
        let data: Data
        if let rustDelegate = delegate as? CSRustSearchableIndexDelegate {
            data = try rustDelegate.data(for: index, itemIdentifier: itemIdentifierString, typeIdentifier: typeIdentifierString)
        } else if let requestHandler = delegate as? CSRustIndexExtensionRequestHandler {
            data = try requestHandler.data(for: index, itemIdentifier: itemIdentifierString, typeIdentifier: typeIdentifierString)
        } else {
            data = Data()
        }
        outJSON?.pointee = csCString(try csEncodeJSON(Array(data)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_file_url_request")
public func csSearchableIndexDelegateSimulateFileURLRequest(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ indexPtr: UnsafeMutableRawPointer?,
    _ itemIdentifier: UnsafePointer<CChar>?,
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ inPlace: Int32,
    _ outURL: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let indexPtr, let itemIdentifier, let typeIdentifier else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index file URL request arguments")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let index: CSSearchableIndex = csBorrow(indexPtr)
        let itemIdentifierString = String(cString: itemIdentifier)
        let typeIdentifierString = String(cString: typeIdentifier)
        let url: URL?
        if let rustDelegate = delegate as? CSRustSearchableIndexDelegate {
            url = try rustDelegate.fileURL(for: index, itemIdentifier: itemIdentifierString, typeIdentifier: typeIdentifierString, inPlace: inPlace != 0)
        } else if let requestHandler = delegate as? CSRustIndexExtensionRequestHandler {
            url = try requestHandler.fileURL(for: index, itemIdentifier: itemIdentifierString, typeIdentifier: typeIdentifierString, inPlace: inPlace != 0)
        } else {
            url = nil
        }
        outURL?.pointee = url.flatMap { csCString($0.absoluteString) }
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_searchable_items_for_identifiers")
public func csSearchableIndexDelegateSimulateSearchableItemsForIdentifiers(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.4, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "searchableItemsForIdentifiers requires macOS 15.4")
        }
        let delegate = try csDelegateObject(delegatePtr)
        let identifiers = try csDecodeJSON(identifiersJSON, as: [String].self)
        let semaphore = DispatchSemaphore(value: 0)
        var fetchedItems: [CSSearchableItem] = []
        delegate.searchableItems?(forIdentifiers: identifiers) { items in
            fetchedItems = items
            semaphore.signal()
        }
        _ = semaphore.wait(timeout: .now() + .seconds(1))
        outJSON?.pointee = csCString(try csEncodeJSON(csRetainedItemPointers(fetchedItems)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_index_delegate_simulate_searchable_items_did_update")
public func csSearchableIndexDelegateSimulateSearchableItemsDidUpdate(
    _ delegatePtr: UnsafeMutableRawPointer?,
    _ itemsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.4, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "searchableItemsDidUpdate requires macOS 15.4")
        }
        let delegate = try csDelegateObject(delegatePtr)
        delegate.searchableItemsDidUpdate?(try csItems(from: itemsJSON))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
