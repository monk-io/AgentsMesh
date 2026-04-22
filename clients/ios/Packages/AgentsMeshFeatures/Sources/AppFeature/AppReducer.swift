import ComposableArchitecture
import SwiftUI
import AuthFeature
import WorkspaceFeature
import TerminalFeature
import AgentsMeshCore

/// Root reducer — three phases: (1) loading while we try to restore a
/// Keychain-backed session, (2) login form, (3) authenticated dashboard.
@Reducer
public struct AppFeature {
    public init() {}

    @ObservableState
    public enum State: Equatable {
        case loading
        case login(LoginFeature.State)
        case dashboard(DashboardFeature.State)

        public init() { self = .loading }
    }

    public enum Action: Sendable {
        case onLaunch
        case sessionRestored(Bool)
        case login(LoginFeature.Action)
        case dashboard(DashboardFeature.Action)
    }

    @Dependency(\.coreClient) var core

    public var body: some ReducerOf<Self> {
        Reduce { state, action in
            switch action {
            case .onLaunch:
                return .run { send in
                    let restored = (try? core.restoreSession()) ?? false
                    await send(.sessionRestored(restored))
                }

            case .sessionRestored(true):
                state = .dashboard(DashboardFeature.State())
                return .none

            case .sessionRestored(false):
                state = .login(LoginFeature.State())
                return .none

            case .login(.delegate(.didAuthenticate)):
                state = .dashboard(DashboardFeature.State())
                return .none

            case .dashboard(.workspace(.delegate(.didLogout))):
                state = .login(LoginFeature.State())
                return .none

            case .login, .dashboard:
                return .none
            }
        }
        .ifCaseLet(\.login, action: \.login) { LoginFeature() }
        .ifCaseLet(\.dashboard, action: \.dashboard) { DashboardFeature() }
    }
}
