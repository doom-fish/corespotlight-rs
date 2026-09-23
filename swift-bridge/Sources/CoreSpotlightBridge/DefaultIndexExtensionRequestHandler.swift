import CoreSpotlight
import Foundation

final class CSDefaultIndexExtensionRequestHandler: CSIndexExtensionRequestHandler {
    private let lock = NSLock()
    fileprivate var reindexAllCount: UInt64 = 0
    fileprivate var reindexIdentifiersCount: UInt64 = 0
    fileprivate var didThrottleCount: UInt64 = 0
    fileprivate var didFinishThrottleCount: UInt64 = 0
    fileprivate var lastIdentifiers: [String] = []

    fileprivate func locked<T>(_ body: () -> T) -> T {
        lock.lock()
        defer { lock.unlock() }
        return body()
    }

    override func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexAllSearchableItemsWithAcknowledgementHandler acknowledgementHandler: @escaping () -> Void) {
        locked { reindexAllCount += 1 }
        acknowledgementHandler()
    }

    override func searchableIndex(_ searchableIndex: CSSearchableIndex, reindexSearchableItemsWithIdentifiers identifiers: [String], acknowledgementHandler: @escaping () -> Void) {
        locked {
            reindexIdentifiersCount += 1
            lastIdentifiers = identifiers
        }
        acknowledgementHandler()
    }

    override func searchableIndexDidThrottle(_ searchableIndex: CSSearchableIndex) {
        locked { didThrottleCount += 1 }
    }

    override func searchableIndexDidFinishThrottle(_ searchableIndex: CSSearchableIndex) {
        locked { didFinishThrottleCount += 1 }
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
    guard let handler = csDefaultHandler(handlerPtr) else { return 0 }
    return handler.locked { handler.reindexAllCount }
}

@_cdecl("cs_default_index_extension_request_handler_get_reindex_identifiers_count")
public func csDefaultIndexExtensionRequestHandlerGetReindexIdentifiersCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handler = csDefaultHandler(handlerPtr) else { return 0 }
    return handler.locked { handler.reindexIdentifiersCount }
}

@_cdecl("cs_default_index_extension_request_handler_get_did_throttle_count")
public func csDefaultIndexExtensionRequestHandlerGetDidThrottleCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handler = csDefaultHandler(handlerPtr) else { return 0 }
    return handler.locked { handler.didThrottleCount }
}

@_cdecl("cs_default_index_extension_request_handler_get_did_finish_throttle_count")
public func csDefaultIndexExtensionRequestHandlerGetDidFinishThrottleCount(_ handlerPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handler = csDefaultHandler(handlerPtr) else { return 0 }
    return handler.locked { handler.didFinishThrottleCount }
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
        outJSON?.pointee = csCString(try csEncodeJSON(handler.locked { handler.lastIdentifiers }))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
