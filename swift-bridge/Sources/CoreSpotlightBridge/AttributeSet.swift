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
        let attributeSet = CSSearchableItemAttributeSet(contentType: csUTType(from: String(cString: itemContentType)))
        outAttributeSet?.pointee = csRetain(attributeSet)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_activity_new")
public func csUserActivityNew(
    _ activityType: UnsafePointer<CChar>?,
    _ outActivity: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let activityType else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user activity type")
        }
        outActivity?.pointee = csRetain(NSUserActivity(activityType: String(cString: activityType)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_user_activity_get_activity_type")
public func csUserActivityGetActivityType(_ activityPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let activityPtr else { return nil }
    let activity: NSUserActivity = csBorrow(activityPtr)
    return csCString(activity.activityType)
}

@_cdecl("cs_user_activity_get_content_attribute_set")
public func csUserActivityGetContentAttributeSet(_ activityPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let activityPtr else { return nil }
    let activity: NSUserActivity = csBorrow(activityPtr)
    return activity.contentAttributeSet.map(csRetain)
}

@_cdecl("cs_user_activity_set_content_attribute_set")
public func csUserActivitySetContentAttributeSet(
    _ activityPtr: UnsafeMutableRawPointer?,
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let activityPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing user activity")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let activity: NSUserActivity = csBorrow(activityPtr)
    activity.contentAttributeSet = attributeSetPtr.map { ptr in
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(ptr)
        return attributeSet
    }
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_string")
public func csAttributeSetGetString(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr, let fieldName else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return (csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? String).flatMap(csCString)
}

@_cdecl("cs_attribute_set_set_string")
public func csAttributeSetSetString(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set string field")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: value.map(String.init(cString:)))
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_string_array")
public func csAttributeSetGetStringArray(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set string-array field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? [String] ?? []
        outJSON?.pointee = csCString(try csEncodeJSON(values))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_string_array")
public func csAttributeSetSetStringArray(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set string-array field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = try csDecodeJSON(valuesJSON, as: [String].self)
        csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: values)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_number")
public func csAttributeSetGetNumber(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Double>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName else { return 0 }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    let value = csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? NSNumber
    return csGetNumber(value, outValue: outValue)
}

@_cdecl("cs_attribute_set_set_number")
public func csAttributeSetSetNumber(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ value: Double,
    _ hasValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set number field")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetNumber(attributeSet, fieldName: String(cString: fieldName), value: value, hasValue: hasValue)
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_url")
public func csAttributeSetGetURL(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let attributeSetPtr, let fieldName else { return nil }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csURLString(csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? URL)
}

@_cdecl("cs_attribute_set_set_url")
public func csAttributeSetSetURL(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set URL field")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetURL(attributeSet, fieldName: String(cString: fieldName), value: value)
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_data")
public func csAttributeSetGetData(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set data field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let value = csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? Data ?? Data()
        outJSON?.pointee = csCString(try csEncodeJSON(Array(value)))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_data")
public func csAttributeSetSetData(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set data field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let value = try csDecodeJSON(valuesJSON, as: [UInt8].self)
        csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: Data(value))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_date")
public func csAttributeSetGetDate(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Double>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName else { return 0 }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    return csGetDate(csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? Date, outValue: outValue)
}

@_cdecl("cs_attribute_set_set_date")
public func csAttributeSetSetDate(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ value: Double,
    _ hasValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set date field")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    csSetDate(attributeSet, fieldName: String(cString: fieldName), value: value, hasValue: hasValue)
    return CSR_OK
}

@_cdecl("cs_attribute_set_get_date_array")
public func csAttributeSetGetDateArray(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set date-array field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = (csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? [Date] ?? []).map(\.timeIntervalSince1970)
        outJSON?.pointee = csCString(try csEncodeJSON(values))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_date_array")
public func csAttributeSetSetDateArray(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set date-array field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = try csDecodeJSON(valuesJSON, as: [Double].self).map(Date.init(timeIntervalSince1970:))
        csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: values)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_person_array")
public func csAttributeSetGetPersonArray(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set person-array field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = (csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? [CSPerson] ?? []).map(csPersonPayload(from:))
        outJSON?.pointee = csCString(try csEncodeJSON(values))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_person_array")
public func csAttributeSetSetPersonArray(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set person-array field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = try csDecodeJSON(valuesJSON, as: [CSPersonPayload].self).map(csMakePerson(from:))
        csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: values)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_string_array_map")
public func csAttributeSetGetStringArrayMap(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set map field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = csAttributeValue(attributeSet, fieldName: String(cString: fieldName)) as? [String: [String]] ?? [:]
        outJSON?.pointee = csCString(try csEncodeJSON(values))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_string_array_map")
public func csAttributeSetSetStringArrayMap(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let fieldName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set map field")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let values = try csDecodeJSON(valuesJSON, as: [String: [String]].self)
        csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: values)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_set_localized_string")
public func csAttributeSetSetLocalizedString(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ fieldName: UnsafePointer<CChar>?,
    _ localizedStringPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let attributeSetPtr, let fieldName, let localizedStringPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing localized string field")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    let localizedString: CSLocalizedString = csBorrow(localizedStringPtr)
    csSetAttributeValue(attributeSet, fieldName: String(cString: fieldName), value: localizedString)
    return CSR_OK
}

@_cdecl("cs_attribute_set_move_from")
public func csAttributeSetMoveFrom(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ sourceAttributeSetPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let attributeSetPtr, let sourceAttributeSetPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing attribute set")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
    let sourceAttributeSet: CSSearchableItemAttributeSet = csBorrow(sourceAttributeSetPtr)
    let selector = NSSelectorFromString("moveFrom:")
    guard attributeSet.responds(to: selector) else {
        let error = csBridgeNSError(
            code: CSR_FAILURE,
            message: "CSSearchableItemAttributeSet.moveFrom: is unavailable on this SDK"
        )
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    _ = attributeSet.perform(selector, with: sourceAttributeSet)
    return CSR_OK
}

@_cdecl("cs_attribute_set_set_custom_value")
public func csAttributeSetSetCustomValue(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ keyPtr: UnsafeMutableRawPointer?,
    _ valueJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let keyPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing custom attribute key")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let key: CSCustomAttributeKey = csBorrow(keyPtr)
        let payload = try csDecodeJSON(valueJSON, as: CSCustomAttributePayload.self)
        attributeSet.setValue(try csCustomAttributeValue(from: payload), forCustomKey: key)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_attribute_set_get_custom_value")
public func csAttributeSetGetCustomValue(
    _ attributeSetPtr: UnsafeMutableRawPointer?,
    _ keyPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let attributeSetPtr, let keyPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing custom attribute key")
        }
        let attributeSet: CSSearchableItemAttributeSet = csBorrow(attributeSetPtr)
        let key: CSCustomAttributeKey = csBorrow(keyPtr)
        let payload = try csCustomAttributePayload(from: attributeSet.value(forCustomKey: key))
        outJSON?.pointee = csCString(try csEncodeJSON(payload))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_localized_string_new")
public func csLocalizedStringNew(
    _ localizedStringsJSON: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let localizedStrings = try csDecodeJSON(localizedStringsJSON, as: [String: String].self)
        let value = CSLocalizedString(localizedStrings: localizedStrings)
        outValue?.pointee = csRetain(value)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_localized_string_get_localized_string")
public func csLocalizedStringGetLocalizedString(_ valuePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let valuePtr else { return nil }
    let value: CSLocalizedString = csBorrow(valuePtr)
    return csCString(value.localizedString())
}

@_cdecl("cs_custom_attribute_key_new")
public func csCustomAttributeKeyNew(
    _ keyName: UnsafePointer<CChar>?,
    _ outKey: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let keyName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing custom attribute key name")
        }
        guard let key = CSCustomAttributeKey(keyName: String(cString: keyName)) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Invalid custom attribute key")
        }
        outKey?.pointee = csRetain(key)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_custom_attribute_key_new_with_options")
public func csCustomAttributeKeyNewWithOptions(
    _ keyName: UnsafePointer<CChar>?,
    _ searchable: Int32,
    _ searchableByDefault: Int32,
    _ unique: Int32,
    _ multiValued: Int32,
    _ outKey: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let keyName else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing custom attribute key name")
        }
        guard let key = CSCustomAttributeKey(
            keyName: String(cString: keyName),
            searchable: searchable != 0,
            searchableByDefault: searchableByDefault != 0,
            unique: unique != 0,
            multiValued: multiValued != 0
        ) else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Invalid custom attribute key")
        }
        outKey?.pointee = csRetain(key)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_custom_attribute_key_get_key_name")
public func csCustomAttributeKeyGetKeyName(_ keyPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let keyPtr else { return nil }
    let key: CSCustomAttributeKey = csBorrow(keyPtr)
    return csCString(key.keyName)
}

@_cdecl("cs_custom_attribute_key_is_searchable")
public func csCustomAttributeKeyIsSearchable(_ keyPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let keyPtr else { return 0 }
    let key: CSCustomAttributeKey = csBorrow(keyPtr)
    return key.isSearchable ? 1 : 0
}

@_cdecl("cs_custom_attribute_key_is_searchable_by_default")
public func csCustomAttributeKeyIsSearchableByDefault(_ keyPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let keyPtr else { return 0 }
    let key: CSCustomAttributeKey = csBorrow(keyPtr)
    return key.isSearchableByDefault ? 1 : 0
}

@_cdecl("cs_custom_attribute_key_is_unique")
public func csCustomAttributeKeyIsUnique(_ keyPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let keyPtr else { return 0 }
    let key: CSCustomAttributeKey = csBorrow(keyPtr)
    return key.isUnique ? 1 : 0
}

@_cdecl("cs_custom_attribute_key_is_multi_valued")
public func csCustomAttributeKeyIsMultiValued(_ keyPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let keyPtr else { return 0 }
    let key: CSCustomAttributeKey = csBorrow(keyPtr)
    return key.isMultiValued ? 1 : 0
}

@_cdecl("cs_person_new")
public func csPersonNew(
    _ displayName: UnsafePointer<CChar>?,
    _ handlesJSON: UnsafePointer<CChar>?,
    _ handleIdentifier: UnsafePointer<CChar>?,
    _ outPerson: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let handleIdentifier else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing person handle identifier")
        }
        let handles = try csDecodeJSON(handlesJSON, as: [String].self)
        let person = CSPerson(
            displayName: displayName.map(String.init(cString:)),
            handles: handles,
            handleIdentifier: String(cString: handleIdentifier)
        )
        outPerson?.pointee = csRetain(person)
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_person_get_display_name")
public func csPersonGetDisplayName(_ personPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let personPtr else { return nil }
    let person: CSPerson = csBorrow(personPtr)
    return person.displayName.flatMap(csCString)
}

@_cdecl("cs_person_get_handles")
public func csPersonGetHandles(
    _ personPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let personPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing person")
        }
        let person: CSPerson = csBorrow(personPtr)
        outJSON?.pointee = csCString(try csEncodeJSON(person.handles))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cs_person_get_handle_identifier")
public func csPersonGetHandleIdentifier(_ personPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let personPtr else { return nil }
    let person: CSPerson = csBorrow(personPtr)
    return csCString(person.handleIdentifier)
}

@_cdecl("cs_person_get_contact_identifier")
public func csPersonGetContactIdentifier(_ personPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let personPtr else { return nil }
    let person: CSPerson = csBorrow(personPtr)
    return person.contactIdentifier.flatMap(csCString)
}

@_cdecl("cs_person_set_contact_identifier")
public func csPersonSetContactIdentifier(
    _ personPtr: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let personPtr else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing person")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let person: CSPerson = csBorrow(personPtr)
    person.contactIdentifier = value.map(String.init(cString:))
    return CSR_OK
}
