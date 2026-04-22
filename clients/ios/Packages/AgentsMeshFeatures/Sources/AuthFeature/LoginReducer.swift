import ComposableArchitecture
import Foundation
import AgentsMeshCore

@Reducer
public struct LoginFeature {
    public init() {}

    @ObservableState
    public struct State: Equatable {
        public var email: String = ""
        public var password: String = ""
        public var isSubmitting: Bool = false
        public var errorMessage: String?

        public init() {}

        var canSubmit: Bool {
            !email.isEmpty && !password.isEmpty && !isSubmitting
        }
    }

    public enum Action: BindableAction, Sendable {
        case binding(BindingAction<State>)
        case submitTapped
        case loginSucceeded(AuthSessionDto)
        case loginFailed(String)
        /// Emitted to the parent reducer (AppFeature) after a successful
        /// login so it can transition to the dashboard.
        case delegate(Delegate)

        public enum Delegate: Equatable, Sendable {
            case didAuthenticate
        }
    }

    @Dependency(\.coreClient) var core

    public var body: some ReducerOf<Self> {
        BindingReducer()
        Reduce { state, action in
            switch action {
            case .binding:
                return .none

            case .submitTapped:
                guard state.canSubmit else { return .none }
                state.isSubmitting = true
                state.errorMessage = nil
                let email = state.email
                let password = state.password
                return .run { send in
                    do {
                        let session = try await core.login(email, password)
                        await send(.loginSucceeded(session))
                    } catch let err as CoreError {
                        await send(.loginFailed(Self.describe(err)))
                    } catch {
                        await send(.loginFailed(error.localizedDescription))
                    }
                }

            case .loginSucceeded:
                state.isSubmitting = false
                state.password = ""
                return .send(.delegate(.didAuthenticate))

            case .loginFailed(let message):
                state.isSubmitting = false
                state.errorMessage = message
                return .none

            case .delegate:
                return .none
            }
        }
    }

    static func describe(_ err: CoreError) -> String {
        switch err {
        case .authExpired: return "Session expired"
        case .http(_, _, let message): return message
        case .network(let message): return "Network: \(message)"
        case .invalidJson(let message): return "Invalid response: \(message)"
        case .notFound(let resource, _): return "\(resource) not found"
        case .notConnected: return "Not connected"
        case .unknown(let message): return message
        }
    }
}
