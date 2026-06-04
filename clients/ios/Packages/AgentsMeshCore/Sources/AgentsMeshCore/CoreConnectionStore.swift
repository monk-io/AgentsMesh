import Foundation

/// Observable connection-state store fed by the Rust `EventSubscriptionManager`.
///
/// Lifecycle mirrors `CoreTickStore`: install once at launch; the Rust loop
/// fires `ConnectionStateCallback` on every transition (connecting / connected /
/// reconnecting / disconnected). SwiftUI shows a reconnect banner while the
/// state is connecting/reconnecting.
///
/// Threading: UniFFI invokes the callback on a tokio worker. We hop to
/// `@MainActor` before publishing — SwiftUI traps on off-thread updates.
//
// ObservableObject + @Published (not iOS-17 `@Observable`) — deployment target
// is iOS 16, matching CoreTickStore.
@MainActor
public final class CoreConnectionStore: ObservableObject, @unchecked Sendable {
    public static let shared = CoreConnectionStore()

    /// "connecting" | "connected" | "reconnecting" | "disconnected".
    @Published public private(set) var state: String = "disconnected"

    private init() {}

    /// Hand the Rust core a listener that mirrors connection-state changes into
    /// this store. Idempotent on the Rust side (single-slot listener set).
    public nonisolated func install(on core: AgentsMeshCore) {
        let callback: ConnectionStateCallback = DispatchCallback { [weak self] state in
            Task { @MainActor in
                self?.state = state
            }
        }
        Task { await core.eventsOnConnectionStateChange(callback: callback) }
    }

    fileprivate final class DispatchCallback: ConnectionStateCallback, @unchecked Sendable {
        private let yield: (String) -> Void
        init(yield: @escaping (String) -> Void) {
            self.yield = yield
        }
        func onConnectionStateChange(state: String) {
            self.yield(state)
        }
    }
}
