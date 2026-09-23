// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CoreSpotlightBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "CoreSpotlightBridge",
            type: .static,
            targets: ["CoreSpotlightBridge"])
    ],
    targets: [
        .target(
            name: "CoreSpotlightObjCBridge",
            path: "Sources/CoreSpotlightObjCBridge",
            publicHeadersPath: "include"),
        .target(
            name: "CoreSpotlightBridge",
            dependencies: ["CoreSpotlightObjCBridge"],
            path: "Sources/CoreSpotlightBridge")
    ]
)
