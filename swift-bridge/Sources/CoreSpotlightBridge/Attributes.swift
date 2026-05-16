import CoreSpotlight
import Foundation

struct CSPersonPayload: Codable {
    var displayName: String?
    var handles: [String]
    var handleIdentifier: String
    var contactIdentifier: String?
}

enum CSCustomAttributePayload: Codable {
    case string(String)
    case number(Double)
    case boolean(Bool)
    case bytes([UInt8])
    case date(Double)
    case person(CSPersonPayload)
    case array([CSCustomAttributePayload])
    case null

    private enum CodingKeys: String, CodingKey {
        case kind
        case value
    }

    private enum Kind: String, Codable {
        case string
        case number
        case boolean
        case bytes
        case date
        case person
        case array
        case null
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        switch try container.decode(Kind.self, forKey: .kind) {
        case .string:
            self = .string(try container.decode(String.self, forKey: .value))
        case .number:
            self = .number(try container.decode(Double.self, forKey: .value))
        case .boolean:
            self = .boolean(try container.decode(Bool.self, forKey: .value))
        case .bytes:
            self = .bytes(try container.decode([UInt8].self, forKey: .value))
        case .date:
            self = .date(try container.decode(Double.self, forKey: .value))
        case .person:
            self = .person(try container.decode(CSPersonPayload.self, forKey: .value))
        case .array:
            self = .array(try container.decode([CSCustomAttributePayload].self, forKey: .value))
        case .null:
            self = .null
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        switch self {
        case let .string(value):
            try container.encode(Kind.string, forKey: .kind)
            try container.encode(value, forKey: .value)
        case let .number(value):
            try container.encode(Kind.number, forKey: .kind)
            try container.encode(value, forKey: .value)
        case let .boolean(value):
            try container.encode(Kind.boolean, forKey: .kind)
            try container.encode(value, forKey: .value)
        case let .bytes(value):
            try container.encode(Kind.bytes, forKey: .kind)
            try container.encode(value, forKey: .value)
        case let .date(value):
            try container.encode(Kind.date, forKey: .kind)
            try container.encode(value, forKey: .value)
        case let .person(value):
            try container.encode(Kind.person, forKey: .kind)
            try container.encode(value, forKey: .value)
        case let .array(value):
            try container.encode(Kind.array, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .null:
            try container.encode(Kind.null, forKey: .kind)
        }
    }
}

func csAttributeValue(_ attributeSet: CSSearchableItemAttributeSet, fieldName: String) -> Any? {
    attributeSet.value(forKey: fieldName)
}

func csSetAttributeValue(_ attributeSet: CSSearchableItemAttributeSet, fieldName: String, value: Any?) {
    attributeSet.setValue(value, forKey: fieldName)
}

func csURLString(_ url: URL?) -> UnsafeMutablePointer<CChar>? {
    guard let url else {
        return nil
    }
    return csCString(url.absoluteString)
}

func csSetURL(_ attributeSet: CSSearchableItemAttributeSet, fieldName: String, value: UnsafePointer<CChar>?) {
    if let value {
        csSetAttributeValue(attributeSet, fieldName: fieldName, value: URL(string: String(cString: value)))
    } else {
        csSetAttributeValue(attributeSet, fieldName: fieldName, value: nil)
    }
}

func csGetNumber(_ value: NSNumber?, outValue: UnsafeMutablePointer<Double>?) -> Int32 {
    guard let value else { return 0 }
    outValue?.pointee = value.doubleValue
    return 1
}

func csSetNumber(_ attributeSet: CSSearchableItemAttributeSet, fieldName: String, value: Double, hasValue: Int32) {
    csSetAttributeValue(attributeSet, fieldName: fieldName, value: hasValue == 0 ? nil : NSNumber(value: value))
}

func csGetDate(_ value: Date?, outValue: UnsafeMutablePointer<Double>?) -> Int32 {
    guard let value else { return 0 }
    outValue?.pointee = value.timeIntervalSince1970
    return 1
}

func csSetDate(_ attributeSet: CSSearchableItemAttributeSet, fieldName: String, value: Double, hasValue: Int32) {
    csSetAttributeValue(attributeSet, fieldName: fieldName, value: hasValue == 0 ? nil : Date(timeIntervalSince1970: value))
}

func csPersonPayload(from person: CSPerson) -> CSPersonPayload {
    CSPersonPayload(
        displayName: person.displayName,
        handles: person.handles,
        handleIdentifier: person.handleIdentifier,
        contactIdentifier: person.contactIdentifier
    )
}

func csMakePerson(from payload: CSPersonPayload) -> CSPerson {
    let person = CSPerson(displayName: payload.displayName, handles: payload.handles, handleIdentifier: payload.handleIdentifier)
    person.contactIdentifier = payload.contactIdentifier
    return person
}

func csCustomAttributePayload(from value: Any?) throws -> CSCustomAttributePayload {
    switch value {
    case nil:
        return .null
    case is NSNull:
        return .null
    case let string as String:
        return .string(string)
    case let number as NSNumber:
        if CFGetTypeID(number) == CFBooleanGetTypeID() {
            return .boolean(number.boolValue)
        }
        return .number(number.doubleValue)
    case let data as Data:
        return .bytes(Array(data))
    case let date as Date:
        return .date(date.timeIntervalSince1970)
    case let person as CSPerson:
        return .person(csPersonPayload(from: person))
    case let values as [Any]:
        return .array(try values.map(csCustomAttributePayload(from:)))
    default:
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Unsupported custom attribute value type: \(type(of: value as Any))")
    }
}

func csCustomAttributeValue(from payload: CSCustomAttributePayload) throws -> NSSecureCoding? {
    switch payload {
    case let .string(value):
        return value as NSString
    case let .number(value):
        return NSNumber(value: value)
    case let .boolean(value):
        return NSNumber(value: value)
    case let .bytes(value):
        return Data(value) as NSData
    case let .date(value):
        return Date(timeIntervalSince1970: value) as NSDate
    case let .person(value):
        return csMakePerson(from: value)
    case let .array(values):
        return try values.map { try csCustomAttributeValue(from: $0) as Any } as NSArray
    case .null:
        return nil
    }
}
