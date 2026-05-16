import CoreSpotlight
import Foundation

let CSR_OK: Int32 = 0
let CSR_INVALID_ARGUMENT: Int32 = -1
let CSR_FAILURE: Int32 = -2
let CSR_TIMED_OUT: Int32 = -3
let CSR_BRIDGE_ERROR_DOMAIN = "CoreSpotlightBridge"

struct CSErrorPayload: Codable {
    var domain: String
    var code: Int
    var message: String
}

@_cdecl("cs_string_free")
public func csStringFree(_ string: UnsafeMutablePointer<CChar>?) {
    free(string)
}

@_cdecl("cs_retain_object")
public func csRetainObject(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr else {
        return nil
    }
    let object = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(object).toOpaque()
}

@_cdecl("cs_release_object")
public func csReleaseObject(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
func csCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func csRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func csBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func csBridgeNSError(code: Int32, message: String) -> NSError {
    NSError(domain: CSR_BRIDGE_ERROR_DOMAIN, code: Int(code), userInfo: [NSLocalizedDescriptionKey: message])
}

func csWriteError(_ error: NSError, to outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) {
    guard let outError else {
        return
    }
    let payload = CSErrorPayload(domain: error.domain, code: error.code, message: error.localizedDescription)
    let json = (try? csEncodeJSON(payload)) ?? "{\"domain\":\"CoreSpotlightBridge\",\"code\":-2,\"message\":\"Unknown Core Spotlight bridge error\"}"
    outError.pointee = csCString(json)
}

func csEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw csBridgeNSError(code: CSR_FAILURE, message: "Failed to encode JSON as UTF-8")
    }
    return string
}

func csDecodeJSON<T: Decodable>(_ cString: UnsafePointer<CChar>?, as _: T.Type) throws -> T {
    guard let cString else {
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing JSON payload")
    }
    let data = Data(String(cString: cString).utf8)
    do {
        return try JSONDecoder().decode(T.self, from: data)
    } catch {
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Invalid JSON payload: \(error.localizedDescription)")
    }
}

func csAwaitCompletion(
    label: String,
    timeoutSeconds: Int32 = 30,
    work: (@escaping (NSError?) -> Void) -> Void
) throws {
    let semaphore = DispatchSemaphore(value: 0)
    var completionError: NSError?
    work { error in
        completionError = error
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
        throw csBridgeNSError(code: CSR_TIMED_OUT, message: "Timed out waiting for \(label)")
    }
    if let completionError {
        throw completionError
    }
}
