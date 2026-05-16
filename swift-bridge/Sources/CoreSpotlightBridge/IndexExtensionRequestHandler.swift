import CoreSpotlight
import Foundation

final class CSRustIndexExtensionRequestHandler: CSIndexExtensionRequestHandler {
    let box: CSRustSearchableIndexDelegateBox

    init(box: CSRustSearchableIndexDelegateBox) {
        self.box = box
    }

    override func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexAllSearchableItemsWithAcknowledgementHandler acknowledgementHandler: @escaping () -> Void) {
        box.reindexAll?(box.context, csRetain(searchableIndex))
        acknowledgementHandler()
    }

    override func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexSearchableItemsWithIdentifiers identifiers: [String], acknowledgementHandler: @escaping () -> Void) {
        guard let callback = box.reindexIdentifiers, let identifiersJSON = try? csEncodeJSON(identifiers) else {
            acknowledgementHandler()
            return
        }
        identifiersJSON.withCString { callback(box.context, csRetain(searchableIndex), $0) }
        acknowledgementHandler()
    }

    override func searchableIndexDidThrottle(_ searchableIndex: CSSearchableIndex) {
        box.didThrottle?(box.context, csRetain(searchableIndex))
    }

    override func searchableIndexDidFinishThrottle(_ searchableIndex: CSSearchableIndex) {
        box.didFinishThrottle?(box.context, csRetain(searchableIndex))
    }

    override func data(for searchableIndex: CSSearchableIndex, itemIdentifier: String, typeIdentifier: String) throws -> Data {
        guard let callback = box.dataForItem else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Index extension data callback not configured")
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
            throw csBridgeNSError(code: CSR_FAILURE, message: "Index extension data callback returned no payload")
        }
        defer { csStringFree(outJSON) }
        let bytes = try csDecodeJSON(outJSON, as: [UInt8].self)
        return Data(bytes)
    }

    override func fileURL(for searchableIndex: CSSearchableIndex, itemIdentifier: String, typeIdentifier: String, inPlace: Bool) throws -> URL {
        guard let callback = box.fileURLForItem else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Index extension file URL callback not configured")
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
            throw csBridgeNSError(code: CSR_FAILURE, message: "Index extension file URL callback returned no URL")
        }
        defer { csStringFree(outURL) }
        guard let url = URL(string: String(cString: outURL)) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Index extension file URL callback returned an invalid URL")
        }
        return url
    }

    @available(macOS 15.4, *)
    override func searchableItems(forIdentifiers identifiers: [String], searchableItemsHandler: @escaping ([CSSearchableItem]) -> Void) {
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
    override func searchableItemsDidUpdate(_ items: [CSSearchableItem]) {
        guard let callback = box.searchableItemsDidUpdate, let itemsJSON = try? csEncodeJSON(csRetainedItemPointers(items)) else {
            return
        }
        itemsJSON.withCString { callback(box.context, $0) }
    }
}

@_cdecl("cs_index_extension_request_handler_new")
public func csIndexExtensionRequestHandlerNew(
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
    _ outHandler: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard reindexAll != nil, reindexIdentifiers != nil else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Index extension request handler requires reindex callbacks")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let handler = CSRustIndexExtensionRequestHandler(box: CSRustSearchableIndexDelegateBox(
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
    outHandler?.pointee = csRetain(handler)
    return CSR_OK
}
