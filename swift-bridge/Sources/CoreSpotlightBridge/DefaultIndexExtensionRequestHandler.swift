import CoreSpotlight
import Foundation

final class CSDefaultIndexExtensionRequestHandler: CSIndexExtensionRequestHandler {
    private(set) var reindexAllCount: UInt64 = 0
    private(set) var reindexIdentifiersCount: UInt64 = 0
    private(set) var didThrottleCount: UInt64 = 0
    private(set) var didFinishThrottleCount: UInt64 = 0
    private(set) var lastIdentifiers: [String] = []

    override func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexAllSearchableItemsWithAcknowledgementHandler acknowledgementHandler: @escaping () -> Void) {
        reindexAllCount += 1
        acknowledgementHandler()
    }

    override func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexSearchableItemsWithIdentifiers identifiers: [String], acknowledgementHandler: @escaping () -> Void) {
        reindexIdentifiersCount += 1
        lastIdentifiers = identifiers
        acknowledgementHandler()
    }

    override func searchableIndexDidThrottle(_ searchableIndex: CSSearchableIndex) {
        didThrottleCount += 1
    }

    override func searchableIndexDidFinishThrottle(_ searchableIndex: CSSearchableIndex) {
        didFinishThrottleCount += 1
    }
}

@_cdecl("cs_default_index_extension_request_handler_new")
public func csDefaultIndexExtensionRequestHandlerNew(
    _ outHandler: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outHandler?.pointee = csRetain(CSDefaultIndexExtensionRequestHandler())
    return CSR_OK
}

private func csDefaultHandler(_ handlerPtr: UnsafeMutableRawPointer?) -> CSDefaultIndexExtensionRequestHandler? {
    guard let handlerPtr else {
        return nil
    }
    return csBorrow(handlerPtr)
}

@_cdecl("cs_default_index_extension_request_handler_get_reindex_all_count")
public func csDefaultIndexExtensionRequestHandlerGetReindexAllCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    csDefaultHandler(handlerPtr)?.reindexAllCount ?? 0
}

@_cdecl("cs_default_index_extension_request_handler_get_reindex_identifiers_count")
public func csDefaultIndexExtensionRequestHandlerGetReindexIdentifiersCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    csDefaultHandler(handlerPtr)?.reindexIdentifiersCount ?? 0
}

@_cdecl("cs_default_index_extension_request_handler_get_did_throttle_count")
public func csDefaultIndexExtensionRequestHandlerGetDidThrottleCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    csDefaultHandler(handlerPtr)?.didThrottleCount ?? 0
}

@_cdecl("cs_default_index_extension_request_handler_get_did_finish_throttle_count")
public func csDefaultIndexExtensionRequestHandlerGetDidFinishThrottleCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    csDefaultHandler(handlerPtr)?.didFinishThrottleCount ?? 0
}

@_cdecl("cs_default_index_extension_request_handler_get_last_identifiers")
public func csDefaultIndexExtensionRequestHandlerGetLastIdentifiers(
    _ handlerPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let handler = csDefaultHandler(handlerPtr) else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing default index extension request handler")
        }
        outJSON?.pointee = csCString(try csEncodeJSON(handler.lastIdentifiers))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
