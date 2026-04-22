import ComposableArchitecture
import SwiftUI
import AuthFeature
import WorkspaceFeature
import TerminalFeature
import DesignSystem

/// Root SwiftUI view dispatching on `AppFeature.State`.
public struct AppView: View {
    @Bindable var store: StoreOf<AppFeature>

    public init(store: StoreOf<AppFeature>) {
        self.store = store
    }

    public var body: some View {
        Group {
            switch store.state {
            case .loading:
                ProgressView().controlSize(.large).tint(AMColors.primary)

            case .login:
                if let loginStore = store.scope(state: \.login, action: \.login) {
                    LoginView(store: loginStore)
                }

            case .dashboard:
                if let dashStore = store.scope(state: \.dashboard, action: \.dashboard) {
                    DashboardView(store: dashStore)
                }
            }
        }
        .onAppear { store.send(.onLaunch) }
    }
}

public struct DashboardView: View {
    @Bindable var store: StoreOf<DashboardFeature>

    public init(store: StoreOf<DashboardFeature>) {
        self.store = store
    }

    public var body: some View {
        NavigationStack {
            PodListView(
                store: store.scope(state: \.workspace, action: \.workspace)
            )
        }
        .fullScreenCover(
            item: $store.scope(state: \.terminal, action: \.terminal)
        ) { terminalStore in
            NavigationStack {
                TerminalView(store: terminalStore)
                    .toolbar {
                        ToolbarItem(placement: .topBarLeading) {
                            Button("Close") {
                                store.send(.terminalDismissed)
                            }
                        }
                    }
            }
        }
    }
}
