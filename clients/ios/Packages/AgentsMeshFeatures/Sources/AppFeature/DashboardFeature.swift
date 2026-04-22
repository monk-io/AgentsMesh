import ComposableArchitecture
import SwiftUI
import WorkspaceFeature
import TerminalFeature

/// Dashboard = workspace list + optional full-screen terminal.
/// Split from AppFeature so the Terminal lifetime is scoped to being
/// signed in (killing the terminal on logout is implicit).
@Reducer
public struct DashboardFeature {
    public init() {}

    @ObservableState
    public struct State: Equatable {
        public var workspace = PodListFeature.State()
        public var terminal: TerminalFeature.State?

        public init() {}
    }

    public enum Action: Sendable {
        case workspace(PodListFeature.Action)
        case terminal(TerminalFeature.Action)
        case terminalDismissed
    }

    public var body: some ReducerOf<Self> {
        Scope(state: \.workspace, action: \.workspace) { PodListFeature() }
        Reduce { state, action in
            switch action {
            case .workspace(.delegate(.openTerminal(let key))):
                state.terminal = TerminalFeature.State(podKey: key)
                return .none

            case .terminalDismissed:
                state.terminal = nil
                return .none

            case .terminal(.delegate(.dismissRequested)):
                state.terminal = nil
                return .none

            case .workspace, .terminal:
                return .none
            }
        }
        .ifLet(\.terminal, action: \.terminal) { TerminalFeature() }
    }
}
