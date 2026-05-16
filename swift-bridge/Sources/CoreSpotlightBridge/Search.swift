import CoreSpotlight
import Foundation

struct CSSearchQueryExecutionPayload: Codable {
    var itemPointers: [UInt64]
    var foundItemCount: UInt64
    var cancelled: Bool
}

struct CSUserQueryExecutionPayload: Codable {
    var itemPointers: [UInt64]
    var foundItemCount: UInt64
    var suggestionPointers: [UInt64]
    var foundSuggestionCount: UInt64
    var cancelled: Bool
}

func csItems(from json: UnsafePointer<CChar>?) throws -> [CSSearchableItem] {
    let rawPointers = try csDecodeJSON(json, as: [UInt64].self)
    return rawPointers.compactMap { rawPointer in
        guard let pointer = UnsafeMutableRawPointer(bitPattern: UInt(rawPointer)) else {
            return nil
        }
        let item: CSSearchableItem = csBorrow(pointer)
        return item
    }
}

func csRetainedItemPointers(_ items: [CSSearchableItem]) -> [UInt64] {
    items.map { UInt64(UInt(bitPattern: csRetain($0))) }
}

func csSuggestions(from json: UnsafePointer<CChar>?) throws -> [CSSuggestion] {
    let rawPointers = try csDecodeJSON(json, as: [UInt64].self)
    return rawPointers.compactMap { rawPointer in
        guard let pointer = UnsafeMutableRawPointer(bitPattern: UInt(rawPointer)) else {
            return nil
        }
        let suggestion: CSSuggestion = csBorrow(pointer)
        return suggestion
    }
}

func csRetainedSuggestionPointers(_ suggestions: [CSSuggestion]) -> [UInt64] {
    suggestions.map { UInt64(UInt(bitPattern: csRetain($0))) }
}
