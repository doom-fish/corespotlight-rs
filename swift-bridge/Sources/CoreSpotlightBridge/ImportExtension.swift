import CoreSpotlight
import Foundation

public typealias CSImportExtensionUpdateCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

final class CSRustImportExtensionBox {
    let context: UnsafeMutableRawPointer?
    let releaseContext: CSDelegateReleaseContextCallback?
    let update: CSImportExtensionUpdateCallback?

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: CSDelegateReleaseContextCallback?,
        update: CSImportExtensionUpdateCallback?
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.update = update
    }

    deinit {
        releaseContext?(context)
    }
}

final class CSRustImportExtension: CSImportExtension {
    let box: CSRustImportExtensionBox

    init(box: CSRustImportExtensionBox) {
        self.box = box
    }

    override func update(_ attributes: CSSearchableItemAttributeSet, forFileAt contentURL: URL) throws {
        guard let callback = box.update else {
            throw csBridgeNSError(code: CSR_FAILURE, message: "Import extension update callback not configured")
        }
        var callbackError: UnsafeMutablePointer<CChar>?
        let status = contentURL.absoluteString.withCString {
            callback(box.context, csRetain(attributes), $0, &callbackError)
        }
        guard status == CSR_OK else {
            throw csNSError(fromJSONCString: callbackError)
        }
    }
}

private func csImportContentURL(from value: UnsafePointer<CChar>?) throws -> URL {
    guard let value else {
        throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing import extension content URL")
    }
    let rawValue = String(cString: value)
    if let contentURL = URL(string: rawValue), contentURL.scheme != nil {
        guard contentURL.isFileURL else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Import extension content URL must be a file URL")
        }
        return contentURL
    }
    return URL(fileURLWithPath: rawValue)
}

@_cdecl("cs_import_extension_new")
public func csImportExtensionNew(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: CSDelegateReleaseContextCallback?,
    _ update: CSImportExtensionUpdateCallback?,
    _ outExtension: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard update != nil else {
        let error = csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Import extension requires an update callback")
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
    let importExtension = CSRustImportExtension(box: CSRustImportExtensionBox(
        context: context,
        releaseContext: releaseContext,
        update: update
    ))
    outExtension?.pointee = csRetain(importExtension)
    return CSR_OK
}

@_cdecl("cs_import_extension_simulate_update")
public func csImportExtensionSimulateUpdate(
    _ extensionPtr: UnsafeMutableRawPointer?,
    _ attributesPtr: UnsafeMutableRawPointer?,
    _ contentURL: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let extensionPtr, let attributesPtr else {
            throw csBridgeNSError(code: CSR_INVALID_ARGUMENT, message: "Missing import extension simulation arguments")
        }
        let importExtension: CSImportExtension = csBorrow(extensionPtr)
        let attributes: CSSearchableItemAttributeSet = csBorrow(attributesPtr)
        try importExtension.update(attributes, forFileAt: csImportContentURL(from: contentURL))
        return CSR_OK
    } catch let error as NSError {
        csWriteError(error, to: outError)
        return Int32(error.code)
    }
}
