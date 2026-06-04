import SwiftUI
import ComposableArchitecture
import AgentsMeshCore
import AppFeature

@main
struct AgentsMeshApp: App {
    @Environment(\.scenePhase) private var scenePhase

    init() {
        let env = ProcessInfo.processInfo.environment
        // Bootstrap Rust core once per process. Resolution order:
        //   1. AGENTSMESH_API_URL env (lets XCUITest launchEnvironment swap)
        //   2. Info.plist (build-config default)
        //   3. production fallback
        let baseURL = env["AGENTSMESH_API_URL"]
            ?? Bundle.main.object(forInfoDictionaryKey: "AGENTSMESH_API_URL") as? String
            ?? "https://api.agentsmesh.ai"

        let storage = KeychainStorage()
        // E2E hook: forces the app onto the login screen on launch
        // by wiping the keychain-backed session before bootstrap.
        if env["AGENTSMESH_RESET_SESSION"] == "1" {
            storage.clear()
        }

        CoreBridge.shared.bootstrap(baseURL: baseURL, storage: storage)
    }

    var body: some Scene {
        WindowGroup {
            AppView(store: Store(initialState: AppFeature.State()) { AppFeature() })
                // Wire the Rust dispatch tick → CoreTickStore once per launch.
                // Reducers observe it (via CoreClient.tickStream) to re-derive
                // state from runtime.state selectors after each realtime event.
                .task {
                    CoreTickStore.shared.install(on: CoreBridge.shared.core)
                    CoreConnectionStore.shared.install(on: CoreBridge.shared.core)
                }
                // Re-arm the realtime stream on foreground: skip the Rust
                // reconnect backoff after suspension (the socket is released on
                // background). No-op when already connected. iOS-16 onChange
                // (single-arg closure) — deployment target predates iOS 17.
                .onChange(of: scenePhase) { phase in
                    if phase == .active {
                        Task { await CoreBridge.shared.core.eventsNudge() }
                    }
                }
        }
    }
}
