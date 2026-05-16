import CoreSpotlight
import Foundation

private func csItems(from json: UnsafePointer<CChar>?) throws -> [CSSearchableItem] {
    let rawPointers = try csDecodeJSON(json, as: [UInt64].self)
    return rawPointers.compactMap { rawPointer in
        guard let pointer = UnsafeMutableRawPointer(bitPattern: UInt(rawPointer)) else {
            return nil
        }
        let item: CSSearchableItem = csBorrow(pointer)
        return item
    }
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

@_cdecl("cs_searchable_item_new")
public func csSearchableItemNew(
    _ uniqueIdentifier: UnsafePointer<CChar>?,
    _ domainIdentifier: UnsafePointer<CChar>?,
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ outItem: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item attribute set")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let item = CSSearchableItem(
            uniqueIdentifier: uniqueIdentifier.map(String.init(cString:)),
            domainIdentifier: domainIdentifier.map(String.init(cString:)),
            attributeSet: attributeSet)
        outItem?.pointee = csRetain(item)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_item_set_expiration_date")
public func csSearchableItemSetExpirationDate(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ unixSeconds: Double,
    _ hasValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let itemPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let item: CSSearchableItem = csBorrow(itemPtr)
    item.expirationDate = hasValue == 0 ? nil : Date(timeIntervalSince1970: unixSeconds)
    return CSR_OK
}

@_cdecl("cs_searchable_item_get_expiration_date")
public func csSearchableItemGetExpirationDate(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ outUnixSeconds: UnsafeMutablePointer<Double>?
) -> Int32 {
    guard let itemPtr else {
        return 0
    }
    let item: CSSearchableItem = csBorrow(itemPtr)
    guard let expirationDate = item.expirationDate else {
        return 0
    }
    outUnixSeconds?.pointee = expirationDate.timeIntervalSince1970
    return 1
}
