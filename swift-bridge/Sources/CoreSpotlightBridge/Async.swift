import CoreSpotlight
import Foundation

public typealias CSAsyncCallback = @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void

private func csAsyncFinish(_ error: Error?, _ cb: CSAsyncCallback, _ ctx: UnsafeMutableRawPointer) {
    guard let error = error as NSError? else {
        cb(nil, nil, ctx)
        return
    }
    let payload = CSErrorPayload(domain: error.domain, code: error.code, message: error.localizedDescription)
    let json = (try? csEncodeJSON(payload)) ?? "{\"domain\":\"CoreSpotlightBridge\",\"code\":-2,\"message\":\"Unknown Core Spotlight bridge error\"}"
    json.withCString { cb(nil, $0, ctx) }
}

private func csAsyncIndex(_ indexPtr: UnsafeMutableRawPointer?) throws -> CSSearchableIndex {
    guard let indexPtr else {
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable index")
    }
    return csBorrow(indexPtr)
}

@_cdecl("corespotlight_index_searchable_items_async")
public func coreSpotlightIndexSearchableItemsAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ itemsJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ cb: CSAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let index: CSSearchableIndex
    let items: [CSSearchableItem]
    do {
        index = try csAsyncIndex(indexPtr)
        items = try csItems(from: itemsJSON)
    } catch {
        csAsyncFinish(error, cb, ctx)
        return
    }
    index.indexSearchableItems(items) { error in
        csAsyncFinish(error, cb, ctx)
    }
}

@_cdecl("corespotlight_delete_searchable_items_with_identifiers_async")
public func coreSpotlightDeleteSearchableItemsWithIdentifiersAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ cb: CSAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let index: CSSearchableIndex
    let identifiers: [String]
    do {
        index = try csAsyncIndex(indexPtr)
        identifiers = try csDecodeJSON(identifiersJSON, as: [String].self)
    } catch {
        csAsyncFinish(error, cb, ctx)
        return
    }
    index.deleteSearchableItems(withIdentifiers: identifiers) { error in
        csAsyncFinish(error, cb, ctx)
    }
}

@_cdecl("corespotlight_delete_searchable_items_with_domain_identifiers_async")
public func coreSpotlightDeleteSearchableItemsWithDomainIdentifiersAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ domainIdentifiersJSON: UnsafePointer<CChar>?,
    _ timeoutSeconds: Int32,
    _ cb: CSAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let index: CSSearchableIndex
    let domainIdentifiers: [String]
    do {
        index = try csAsyncIndex(indexPtr)
        domainIdentifiers = try csDecodeJSON(domainIdentifiersJSON, as: [String].self)
    } catch {
        csAsyncFinish(error, cb, ctx)
        return
    }
    index.deleteSearchableItems(withDomainIdentifiers: domainIdentifiers) { error in
        csAsyncFinish(error, cb, ctx)
    }
}

@_cdecl("corespotlight_delete_all_searchable_items_async")
public func coreSpotlightDeleteAllSearchableItemsAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ cb: CSAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let index: CSSearchableIndex
    do {
        index = try csAsyncIndex(indexPtr)
    } catch {
        csAsyncFinish(error, cb, ctx)
        return
    }
    index.deleteAllSearchableItems { error in
        csAsyncFinish(error, cb, ctx)
    }
}

@_cdecl("corespotlight_fetch_last_client_state_async")
public func coreSpotlightFetchLastClientStateAsync(
    _ indexPtr: UnsafeMutableRawPointer?,
    _ cb: CSAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let index: CSSearchableIndex
    do {
        index = try csAsyncIndex(indexPtr)
    } catch {
        csAsyncFinish(error, cb, ctx)
        return
    }
    index.fetchLastClientState { clientState, error in
        if let error {
            csAsyncFinish(error, cb, ctx)
            return
        }
        do {
            let json = try csEncodeJSON(clientState.map(Array.init) ?? [UInt8]())
            json.withCString { cb(UnsafeRawPointer($0), nil, ctx) }
        } catch {
            csAsyncFinish(error, cb, ctx)
        }
    }
}
