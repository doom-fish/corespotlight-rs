import CoreSpotlight
import Foundation

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
            attributeSet: attributeSet
        )
        outItem?.pointee = csRetain(item)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_item_compare_by_rank")
public func csSearchableItemCompareByRank(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ otherPtr: UnsafeMutableRawPointer?
) -> Int32 {
    guard let itemPtr, let otherPtr else {
        return 0
    }
    let item: CSSearchableItem = csBorrow(itemPtr)
    let other: CSSearchableItem = csBorrow(otherPtr)
    switch item.compare(byRank: other) {
    case .orderedAscending:
        return -1
    case .orderedDescending:
        return 1
    case .orderedSame:
        return 0
    @unknown default:
        return 0
    }
}

@_cdecl("cs_searchable_item_get_unique_identifier")
public func csSearchableItemGetUniqueIdentifier(_ itemPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let itemPtr else { return nil }
    let item: CSSearchableItem = csBorrow(itemPtr)
    return csCString(item.uniqueIdentifier)
}

@_cdecl("cs_searchable_item_set_unique_identifier")
public func csSearchableItemSetUniqueIdentifier(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let itemPtr, let value else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item unique identifier")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let item: CSSearchableItem = csBorrow(itemPtr)
    item.uniqueIdentifier = String(cString: value)
    return CSR_OK
}

@_cdecl("cs_searchable_item_get_domain_identifier")
public func csSearchableItemGetDomainIdentifier(_ itemPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let itemPtr else { return nil }
    let item: CSSearchableItem = csBorrow(itemPtr)
    guard let domainIdentifier = item.domainIdentifier else { return nil }
    return csCString(domainIdentifier)
}

@_cdecl("cs_searchable_item_set_domain_identifier")
public func csSearchableItemSetDomainIdentifier(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let itemPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let item: CSSearchableItem = csBorrow(itemPtr)
    item.domainIdentifier = value.map(String.init(cString:))
    return CSR_OK
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

@_cdecl("cs_searchable_item_get_attribute_set")
public func csSearchableItemGetAttributeSet(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ outAttributeSet: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let itemPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item")
        }
        let item: CSSearchableItem = csBorrow(itemPtr)
        outAttributeSet?.pointee = csRetain(item.attributeSet)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_item_set_attribute_set")
public func csSearchableItemSetAttributeSet(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let itemPtr, let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let item: CSSearchableItem = csBorrow(itemPtr)
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    item.attributeSet = attributeSet
    return CSR_OK
}

@_cdecl("cs_searchable_item_get_is_update")
public func csSearchableItemGetIsUpdate(_ itemPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let itemPtr else { return 0 }
    let item: CSSearchableItem = csBorrow(itemPtr)
    return item.isUpdate ? 1 : 0
}

@_cdecl("cs_searchable_item_set_is_update")
public func csSearchableItemSetIsUpdate(_ itemPtr: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let itemPtr else { return }
    let item: CSSearchableItem = csBorrow(itemPtr)
    item.isUpdate = value != 0
}

@_cdecl("cs_searchable_item_get_update_listener_options")
public func csSearchableItemGetUpdateListenerOptions(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt64>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.4, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "updateListenerOptions requires macOS 15.4")
        }
        guard let itemPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item")
        }
        let item: CSSearchableItem = csBorrow(itemPtr)
        outValue?.pointee = UInt64(item.updateListenerOptions.rawValue)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_searchable_item_set_update_listener_options")
public func csSearchableItemSetUpdateListenerOptions(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ value: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.4, *) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "updateListenerOptions requires macOS 15.4")
        }
        guard let itemPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing searchable item")
        }
        let item: CSSearchableItem = csBorrow(itemPtr)
        item.updateListenerOptions = CSSearchableItem.UpdateListenerOptions(rawValue: UInt(value))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
