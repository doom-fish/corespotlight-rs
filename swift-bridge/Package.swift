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
            name: "CoreSpotlightBridge",
            path: "Sources/CoreSpotlightBridge",
            publicHeadersPath: "include")
    ]
)
