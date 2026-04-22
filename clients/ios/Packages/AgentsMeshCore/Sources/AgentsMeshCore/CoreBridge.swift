import Foundation

/// App-wide singleton that owns the Rust `AgentsMeshCore` handle.
///
/// Why a singleton: the Rust side holds a tokio runtime and a long-lived
/// reqwest client + auth manager. We don't want to re-initialize those on
/// every SwiftUI view. TCA reducers reach this through `CoreClient` in
/// `DependencyKeys.swift` — they never call `CoreBridge.shared` directly
/// (keeps reducers testable).
public final class CoreBridge: @unchecked Sendable {
    public static let shared = CoreBridge()

    private var _core: AgentsMeshCore?
    private let lock = NSLock()

    private init() {}

    /// Call once during app launch. `baseURL` is the backend origin
    /// (e.g. `https://agentsmesh.example.com`).
    public func bootstrap(baseURL: String, storage: KeychainStorage) {
        lock.lock(); defer { lock.unlock() }
        guard _core == nil else { return }
        _core = AgentsMeshCore(baseUrl: baseURL, storage: storage)
    }

    /// Access the underlying Rust core. Traps if `bootstrap` wasn't called.
    public var core: AgentsMeshCore {
        lock.lock(); defer { lock.unlock() }
        guard let c = _core else {
            preconditionFailure(
                "CoreBridge not bootstrapped — call CoreBridge.shared.bootstrap() at launch."
            )
        }
        return c
    }

    /// Test hook: reset between test cases.
    public func __reset() {
        lock.lock(); defer { lock.unlock() }
        _core = nil
    }
}
