// swift-tools-version: 5.9
// `AgentsMeshCore` — Swift façade over the Rust-powered XCFramework.
//
// The binary target points at the xcframework produced by
// `clients/core/scripts/build-ios-xcframework.sh`. In CI/release we swap
// this to a `.url(...)` pointing at a GitHub release asset.
import PackageDescription

let package = Package(
    name: "AgentsMeshCore",
    platforms: [.iOS(.v16)],
    products: [
        .library(name: "AgentsMeshCore", targets: ["AgentsMeshCore"]),
    ],
    targets: [
        // Auto-generated Swift glue (produced by uniffi-bindgen-swift) lives at
        // ../../../core/ios-framework/Generated/AgentsMeshCore.swift. SPM
        // doesn't let us add files from outside the package, so we symlink or
        // copy them into Sources/AgentsMeshCore/Generated/ as part of the
        // XCFramework build script. See Makefile for the link step.
        .target(
            name: "AgentsMeshCore",
            dependencies: ["AgentsMeshCoreFFI"],
            path: "Sources/AgentsMeshCore"
        ),
        .binaryTarget(
            name: "AgentsMeshCoreFFI",
            // Relative to package root; symlink created by Makefile.
            path: "Sources/AgentsMeshCoreFFI/AgentsMeshCore.xcframework"
        ),
        .testTarget(
            name: "AgentsMeshCoreTests",
            dependencies: ["AgentsMeshCore"],
            path: "Tests/AgentsMeshCoreTests"
        ),
    ]
)
