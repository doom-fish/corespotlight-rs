import CoreSpotlight
import Foundation

// ============================================================================
// AsyncIndexSearchableItems
// ============================================================================

@_cdecl("corespotlight_index_searchable_items_async")
public func coreSpotlightIndexSearchableItemsAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ itemsJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            guard let indexPtr else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
            }
            guard let itemsJSON else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing items JSON")
            }
            
            let index: CSSearchableIndex = csBorrow(indexPtr)
            let itemPointers = try csDecodeJSON(itemsJSON, as: [UInt64].self)
            let items = itemPointers.compactMap { ptr -> CSSearchableItem? in
                let rawPtr = UnsafeMutableRawPointer(bitPattern: UInt(ptr))
                guard let rawPtr else { return nil }
                return csBorrow(rawPtr)
            }
            
            try await index.indexSearchableItems(items)
            cb(nil, nil, ctx)
        } catch let error as NSError {
            error.localizedDescription.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}

// ============================================================================
// AsyncDeleteSearchableItems (by identifier)
// ============================================================================

@_cdecl("corespotlight_delete_searchable_items_with_identifiers_async")
public func coreSpotlightDeleteSearchableItemsWithIdentifiersAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            guard let indexPtr else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
            }
            guard let identifiersJSON else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing identifiers JSON")
            }
            
            let index: CSSearchableIndex = csBorrow(indexPtr)
            let identifiers = try csDecodeJSON(identifiersJSON, as: [String].self)
            
            try await index.deleteSearchableItems(withIdentifiers: identifiers)
            cb(nil, nil, ctx)
        } catch let error as NSError {
            error.localizedDescription.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}

// ============================================================================
// AsyncDeleteSearchableItems (by domain identifier)
// ============================================================================

@_cdecl("corespotlight_delete_searchable_items_with_domain_identifiers_async")
public func coreSpotlightDeleteSearchableItemsWithDomainIdentifiersAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ domainIdentifiersJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            guard let indexPtr else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
            }
            guard let domainIdentifiersJSON else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing domain identifiers JSON")
            }
            
            let index: CSSearchableIndex = csBorrow(indexPtr)
            let domainIdentifiers = try csDecodeJSON(domainIdentifiersJSON, as: [String].self)
            
            try await index.deleteSearchableItems(withDomainIdentifiers: domainIdentifiers)
            cb(nil, nil, ctx)
        } catch let error as NSError {
            error.localizedDescription.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}

// ============================================================================
// AsyncDeleteAllSearchableItems
// ============================================================================

@_cdecl("corespotlight_delete_all_searchable_items_async")
public func coreSpotlightDeleteAllSearchableItemsAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            guard let indexPtr else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
            }
            
            let index: CSSearchableIndex = csBorrow(indexPtr)
            try await index.deleteAllSearchableItems()
            cb(nil, nil, ctx)
        } catch let error as NSError {
            error.localizedDescription.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}

// ============================================================================
// AsyncFetchLastClientState (macOS 13+)
// ============================================================================

@_cdecl("corespotlight_fetch_last_client_state_async")
public func coreSpotlightFetchLastClientStateAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            guard let indexPtr else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
            }
            
            let index: CSSearchableIndex = csBorrow(indexPtr)
            
            if #available(macOS 13.0, *) {
                let clientState = try await index.fetchLastClientState()
                let dataArray = [UInt8](clientState)
                let json = try csEncodeJSON(dataArray)
                let jsonCStr = csCString(json)
                cb(Unmanaged.passRetained(jsonCStr as AnyObject).toOpaque(), nil, ctx)
            } else {
                throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "fetchLastClientState requires macOS 13.0+")
            }
        } catch let error as NSError {
            error.localizedDescription.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}
