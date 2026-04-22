// swift-tools-version: 5.9
//
// External Swift packages consumed by AgentsMesh. This file is **not** an
// SPM target layout — it's only a declaration of SPM deps that
// `rules_swift_package_manager` reads to generate `@swiftpkg_<name>`
// Bazel repos.
//
// Runtime app targets live in `clients/ios/` and declare their own
// `swift_library` BUILD targets that depend on these packages by name:
//
//     @swiftpkg_swift_composable_architecture//:ComposableArchitecture
//     @swiftpkg_swiftterm//:SwiftTerm
import PackageDescription

let package = Package(
    name: "ThirdLibraries",
    platforms: [.iOS(.v16)],
    dependencies: [
        .package(
            url: "https://github.com/pointfreeco/swift-composable-architecture",
            from: "1.15.0"
        ),
        .package(url: "https://github.com/migueldeicaza/SwiftTerm", from: "1.2.0"),
    ],
    targets: []
)
