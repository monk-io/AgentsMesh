import SwiftUI
import ComposableArchitecture
import AgentsMeshCore
import AppFeature

@main
struct AgentsMeshApp: App {
    init() {
        // Bootstrap Rust core once per process. API URL comes from
        // Info.plist so build configurations (dev/staging/prod) can
        // swap backends without a code change.
        let baseURL = Bundle.main.object(forInfoDictionaryKey: "AGENTSMESH_API_URL")
            as? String ?? "https://api.agentsmesh.ai"
        CoreBridge.shared.bootstrap(
            baseURL: baseURL,
            storage: KeychainStorage()
        )
    }

    var body: some Scene {
        WindowGroup {
            AppView(store: Store(initialState: AppFeature.State()) { AppFeature() })
        }
    }
}
