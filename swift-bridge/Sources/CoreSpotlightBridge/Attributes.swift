import CoreSpotlight
import Foundation

@_cdecl("cs_attribute_set_new")
public func csAttributeSetNew(
    _ itemContentType: UnsafePointer<CChar>?,
    _ outAttributeSet: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let itemContentType else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing item content type")
        }
        let attributeSet = CSSearchableItemAttributeSet(itemContentType: String(cString: itemContentType))
        outAttributeSet?.pointee = csRetain(attributeSet)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

private func csURLString(_ url: URL?) -> UnsafeMutablePointer<CChar>? {
    guard let url else {
        return nil
    }
    return csCString(url.absoluteString)
}

private func csSetURL(_ attributeSet: CSSearchableItemAttributeSet, value: UnsafePointer<CChar>?, keyPath: ReferenceWritableKeyPath<CSSearchableItemAttributeSet, URL?>) {
    if let value {
        attributeSet[keyPath: keyPath] = URL(string: String(cString: value))
    } else {
        attributeSet[keyPath: keyPath] = nil
    }
}

@_cdecl("cs_attribute_set_get_item_content_type")
public func csAttributeSetGetItemContentType(_ attributeSetPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    guard let value = attributeSet.contentType else { return nil }
    return csCString(value)
}

@_cdecl("cs_attribute_set_set_item_content_type")
public func csAttributeSetSetItemContentType(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    attributeSet.contentType = value.map { String(cString: $0) }
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_title")
public func csAttributeSetGetTitle(_ attributeSetPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    guard let value = attributeSet.title else { return nil }
    return csCString(value)
}

@_cdecl("cs_attribute_set_set_title")
public func csAttributeSetSetTitle(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    attributeSet.title = value.map { String(cString: $0) }
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_content_description")
public func csAttributeSetGetContentDescription(_ attributeSetPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    guard let value = attributeSet.contentDescription else { return nil }
    return csCString(value)
}

@_cdecl("cs_attribute_set_set_content_description")
public func csAttributeSetSetContentDescription(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    attributeSet.contentDescription = value.map { String(cString: $0) }
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_display_name")
public func csAttributeSetGetDisplayName(_ attributeSetPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    guard let value = attributeSet.displayName else { return nil }
    return csCString(value)
}

@_cdecl("cs_attribute_set_set_display_name")
public func csAttributeSetSetDisplayName(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    attributeSet.displayName = value.map { String(cString: $0) }
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_keywords")
public func csAttributeSetGetKeywords(_ attributeSetPtr: UnsafeMutableRawPointer?, _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    do {
        guard let attributeSetPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let keywords = attributeSet.keywords ?? []
        outJSON?.pointee = csCString(try csEncodeJSON(keywords))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_keywords")
public func csAttributeSetSetKeywords(_ attributeSetPtr: UnsafeMutableRawPointer?, _ valuesJSON: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    do {
        guard let attributeSetPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        attributeSet.keywords = try csDecodeJSON(valuesJSON, as: [String].self)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_thumbnail_data")
public func csAttributeSetGetThumbnailData(_ attributeSetPtr: UnsafeMutableRawPointer?, _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    do {
        guard let attributeSetPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let bytes = attributeSet.thumbnailData.map(Array.init) ?? []
        outJSON?.pointee = csCString(try csEncodeJSON(bytes))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_thumbnail_data")
public func csAttributeSetSetThumbnailData(_ attributeSetPtr: UnsafeMutableRawPointer?, _ valuesJSON: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    do {
        guard let attributeSetPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let bytes = try csDecodeJSON(valuesJSON, as: [UInt8].self)
        attributeSet.thumbnailData = Data(bytes)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_thumbnail_url")
public func csAttributeSetGetThumbnailURL(_ attributeSetPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csURLString(attributeSet.thumbnailURL)
}

@_cdecl("cs_attribute_set_set_thumbnail_url")
public func csAttributeSetSetThumbnailURL(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetURL(attributeSet, value: value, keyPath: \.thumbnailURL)
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_content_url")
public func csAttributeSetGetContentURL(_ attributeSetPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csURLString(attributeSet.contentURL)
}

@_cdecl("cs_attribute_set_set_content_url")
public func csAttributeSetSetContentURL(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetURL(attributeSet, value: value, keyPath: \.contentURL)
    return CSR_OK
}

private func csGetNumber(_ value: NSNumber?, outValue: UnsafeMutablePointer<Double>?) -> Int32 {
    guard let value else { return 0 }
    outValue?.pointee = value.doubleValue
    return 1
}

private func csSetNumber(_ attributeSet: CSSearchableItemAttributeSet, keyPath: ReferenceWritableKeyPath<CSSearchableItemAttributeSet, NSNumber?>, value: Double, hasValue: Int32) {
    attributeSet[keyPath: keyPath] = hasValue == 0 ? nil : NSNumber(value: value)
}

@_cdecl("cs_attribute_set_get_latitude")
public func csAttributeSetGetLatitude(_ attributeSetPtr: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Int32 {
    guard let attributeSetPtr else { return 0 }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csGetNumber(attributeSet.latitude, outValue: outValue)
}

@_cdecl("cs_attribute_set_set_latitude")
public func csAttributeSetSetLatitude(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: Double, _ hasValue: Int32, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetNumber(attributeSet, keyPath: \.latitude, value: value, hasValue: hasValue)
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_longitude")
public func csAttributeSetGetLongitude(_ attributeSetPtr: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Int32 {
    guard let attributeSetPtr else { return 0 }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csGetNumber(attributeSet.longitude, outValue: outValue)
}

@_cdecl("cs_attribute_set_set_longitude")
public func csAttributeSetSetLongitude(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: Double, _ hasValue: Int32, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetNumber(attributeSet, keyPath: \.longitude, value: value, hasValue: hasValue)
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_rating")
public func csAttributeSetGetRating(_ attributeSetPtr: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Int32 {
    guard let attributeSetPtr else { return 0 }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csGetNumber(attributeSet.rating, outValue: outValue)
}

@_cdecl("cs_attribute_set_set_rating")
public func csAttributeSetSetRating(_ attributeSetPtr: UnsafeMutableRawPointer?, _ value: Double, _ hasValue: Int32, _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    guard let attributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetNumber(attributeSet, keyPath: \.rating, value: value, hasValue: hasValue)
    return CSR_OK
}
