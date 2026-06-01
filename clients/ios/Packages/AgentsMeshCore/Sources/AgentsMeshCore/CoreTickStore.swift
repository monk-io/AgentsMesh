import Foundation

/// Observable tick store fed by the Rust `EventSubscriptionManager`.
///
/// Lifecycle:
///   1. App launch → `CoreBridge.shared.bootstrap(...)` constructs Rust core.
///   2. Anywhere mid-flight (typically `SceneDelegate.willEnterForeground` or
///      App.init) → `CoreTickStore.shared.install(on: CoreBridge.shared.core)`.
///   3. Each realtime event → Rust dispatches into AppState → fires
///      `TickCallback.onTick(tick)` → this store flips `@Published tick`.
///   4. SwiftUI views observing `coreTick.tick` re-derive their state from
///      Rust selectors (e.g. `core.podsJson()`).
///
/// Threading: UniFFI invokes the callback on a tokio worker. We hop to
/// `@MainActor` before publishing — SwiftUI traps on off-thread updates.
//
// ObservableObject + @Published (not the iOS-17 `@Observable` macro) — the app
// deployment target is iOS 16. Reducers poll `tick` via CoreClient.tickStream;
// SwiftUI observers (if any) use @ObservedObject.
@MainActor
public final class CoreTickStore: ObservableObject, @unchecked Sendable {
    public static let shared = CoreTickStore()

    /// Monotonic counter. Increments after every event applied to AppState.
    /// SwiftUI observers re-render on each change; they read fresh state
    /// via `CoreBridge.shared.core.podsJson()` (etc) in their derive blocks.
    @Published public private(set) var tick: UInt64 = 0

    private init() {}

    /// Hand the Rust core a callback that updates this store. Idempotent —
    /// calling again replaces the previous registration (also single-slot
    /// on the Rust side).
    public nonisolated func install(on core: AgentsMeshCore) {
        let callback: TickCallback = DispatchCallback { [weak self] tick in
            Task { @MainActor in
                self?.tick = tick
            }
        }
        core.setTickCallback(callback: callback)
    }

    /// Remove the callback (e.g. on app teardown).
    public nonisolated func uninstall(from core: AgentsMeshCore) {
        core.clearTickCallback()
    }

    fileprivate final class DispatchCallback: TickCallback, @unchecked Sendable {
        private let yield: (UInt64) -> Void
        init(yield: @escaping (UInt64) -> Void) {
            self.yield = yield
        }
        func onTick(tick: UInt64) {
            self.yield(tick)
        }
    }
}

/// Convenience for tests that need to wait until N ticks have arrived
/// (e.g. mock backend pushes 3 events → assert state.pods.count after
/// the 3rd tick lands). XCTest pattern:
///
///     await store.waitUntilTick(>= 3, timeout: 5)
///
extension CoreTickStore {
    /// Suspend until `tick >= target` or the timeout elapses. Returns the
    /// observed tick. Polls every 50ms — cheap because SwiftUI Observation
    /// is already piping these updates on main.
    public func waitUntilTick(atLeast target: UInt64, timeout: TimeInterval = 10) async -> UInt64 {
        let start = Date()
        while tick < target {
            if Date().timeIntervalSince(start) > timeout { break }
            try? await Task.sleep(nanoseconds: 50_000_000)
        }
        return tick
    }
}
