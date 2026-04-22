import ComposableArchitecture
import Foundation
import AgentsMeshCore

/// Typed wrapper over the Rust `AgentsMeshCore` methods that reducers need.
/// Structuring as a TCA `DependencyClient` gives us trivial test overrides
/// and decouples reducers from `CoreBridge.shared`.
public struct CoreClient: Sendable {
    // Auth
    public var login: @Sendable (_ email: String, _ password: String) async throws -> AuthSessionDto
    public var logout: @Sendable () async throws -> Void
    public var isAuthenticated: @Sendable () -> Bool
    public var restoreSession: @Sendable () throws -> Bool
    public var currentUser: @Sendable () -> UserDto?
    public var currentOrg: @Sendable () -> OrganizationDto?
    public var fetchOrganizations: @Sendable () async throws -> [OrganizationDto]
    public var switchOrg: @Sendable (_ slug: String) throws -> Void

    // Workspace
    public var listPods: @Sendable (_ status: String?) async throws -> PodListResponseDto
    public var getPod: @Sendable (_ key: String) async throws -> PodDto
    public var terminatePod: @Sendable (_ key: String) async throws -> Void
    public var getPodRelayConnection: @Sendable (_ key: String) async throws -> PodConnectionInfoDto
    public var listRunners: @Sendable () async throws -> RunnerListResponseDto
}

extension CoreClient: DependencyKey {
    public static let liveValue: CoreClient = {
        // Closures capture CoreBridge.shared.core lazily so this struct
        // can be instantiated before bootstrap (e.g. in previews).
        let core = { CoreBridge.shared.core }
        return CoreClient(
            login: { email, password in
                try await core().login(email: email, password: password)
            },
            logout: {
                try await core().logout()
            },
            isAuthenticated: {
                core().isAuthenticated()
            },
            restoreSession: {
                try core().restoreSession()
            },
            currentUser: {
                core().getCurrentUser()
            },
            currentOrg: {
                core().getCurrentOrg()
            },
            fetchOrganizations: {
                try await core().fetchOrganizations()
            },
            switchOrg: { slug in
                try core().switchOrg(slug: slug)
            },
            listPods: { status in
                try await core().listPods(
                    status: status, runnerId: nil, createdById: nil,
                    limit: nil, offset: nil
                )
            },
            getPod: { key in
                try await core().getPod(podKey: key)
            },
            terminatePod: { key in
                try await core().terminatePod(podKey: key)
            },
            getPodRelayConnection: { key in
                try await core().getPodRelayConnection(podKey: key)
            },
            listRunners: {
                try await core().listRunners(status: nil)
            }
        )
    }()

    public static let testValue: CoreClient = CoreClient(
        login: unimplemented("CoreClient.login"),
        logout: unimplemented("CoreClient.logout"),
        isAuthenticated: unimplemented("CoreClient.isAuthenticated", placeholder: false),
        restoreSession: unimplemented("CoreClient.restoreSession", placeholder: false),
        currentUser: unimplemented("CoreClient.currentUser", placeholder: nil),
        currentOrg: unimplemented("CoreClient.currentOrg", placeholder: nil),
        fetchOrganizations: unimplemented("CoreClient.fetchOrganizations"),
        switchOrg: unimplemented("CoreClient.switchOrg"),
        listPods: unimplemented("CoreClient.listPods"),
        getPod: unimplemented("CoreClient.getPod"),
        terminatePod: unimplemented("CoreClient.terminatePod"),
        getPodRelayConnection: unimplemented("CoreClient.getPodRelayConnection"),
        listRunners: unimplemented("CoreClient.listRunners")
    )
}

public extension DependencyValues {
    var coreClient: CoreClient {
        get { self[CoreClient.self] }
        set { self[CoreClient.self] = newValue }
    }
}
