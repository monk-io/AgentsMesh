// swift-tools-version: 5.9
// `AgentsMeshFeatures` — TCA reducers, SwiftUI views, and SwiftTerm wrappers.
// Depends on AgentsMeshCore for the Rust-backed service layer.
import PackageDescription

let package = Package(
    name: "AgentsMeshFeatures",
    platforms: [.iOS(.v16)],
    products: [
        .library(name: "AppFeature", targets: ["AppFeature"]),
        .library(name: "AuthFeature", targets: ["AuthFeature"]),
        .library(name: "WorkspaceFeature", targets: ["WorkspaceFeature"]),
        .library(name: "TerminalFeature", targets: ["TerminalFeature"]),
        .library(name: "DesignSystem", targets: ["DesignSystem"]),
    ],
    dependencies: [
        .package(path: "../AgentsMeshCore"),
        .package(
            url: "https://github.com/pointfreeco/swift-composable-architecture",
            from: "1.15.0"
        ),
        .package(url: "https://github.com/migueldeicaza/SwiftTerm", from: "1.2.0"),
    ],
    targets: [
        .target(
            name: "DesignSystem",
            path: "Sources/DesignSystem"
        ),
        .target(
            name: "AuthFeature",
            dependencies: [
                "DesignSystem",
                "AgentsMeshCore",
                .product(name: "ComposableArchitecture", package: "swift-composable-architecture"),
            ],
            path: "Sources/AuthFeature"
        ),
        .target(
            name: "WorkspaceFeature",
            dependencies: [
                "DesignSystem",
                "AgentsMeshCore",
                .product(name: "ComposableArchitecture", package: "swift-composable-architecture"),
            ],
            path: "Sources/WorkspaceFeature"
        ),
        .target(
            name: "TerminalFeature",
            dependencies: [
                "DesignSystem",
                "AgentsMeshCore",
                .product(name: "ComposableArchitecture", package: "swift-composable-architecture"),
                .product(name: "SwiftTerm", package: "SwiftTerm"),
            ],
            path: "Sources/TerminalFeature"
        ),
        .target(
            name: "AppFeature",
            dependencies: [
                "AuthFeature",
                "WorkspaceFeature",
                "TerminalFeature",
                "AgentsMeshCore",
                .product(name: "ComposableArchitecture", package: "swift-composable-architecture"),
            ],
            path: "Sources/AppFeature"
        ),
    ]
)
