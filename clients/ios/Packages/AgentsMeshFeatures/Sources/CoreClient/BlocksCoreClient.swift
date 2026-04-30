import Foundation
import AgentsMeshCore

/// Blocks slice of `CoreClient`. Mirrors the
/// `BlockstoreService` interface that web's
/// `clients/web/src/lib/api/blockstoreApi.ts` calls — every iOS path
/// that talks to the Rust blockstore (BlocksReducer + the embedded
/// WebView's RPC router) goes through this one slice.
///
/// JSON-string entry points exist for the RPC bridge; typed accessors
/// (Workspace/Block/ChildrenResult) exist for native TCA reducers.
public struct BlocksCoreClient: Sendable {
    // ── Mutations
    public var applyOps: @Sendable (_ reqJson: String) async throws -> String
    public var loadSubtree: @Sendable (_ wsId: String, _ rootId: String) async throws -> Void
    public var loadTypeDefs: @Sendable (_ wsId: String) async throws -> Void
    public var catchup: @Sendable (_ wsId: String) async throws -> Void
    public var applyRemoteOp: @Sendable (_ opJson: String) throws -> Void
    public var setLastOpId: @Sendable (_ wsId: String, _ id: Int64) -> Void

    // ── Async readers (typed)
    public var ensureDefaultWorkspace: @Sendable () async throws -> WorkspaceDto
    public var listWorkspaces: @Sendable () async throws -> [WorkspaceDto]
    public var getSubtree: @Sendable (_ wsId: String, _ rootId: String, _ depth: UInt32) async throws -> ChildrenResultDto
    public var getBlock: @Sendable (_ id: String) async throws -> BlockDto
    public var semanticSearch: @Sendable (_ wsId: String, _ req: SemanticSearchRequestDto) async throws -> [SearchHitDto]

    // ── Async readers (JSON string — for RPC bridge passthrough)
    public var ensureDefaultWorkspaceJson: @Sendable () async throws -> String
    public var listWorkspacesJson: @Sendable () async throws -> String
    public var semanticSearchJson: @Sendable (_ wsId: String, _ reqJson: String) async throws -> String

    // ── Sync flat-map readers — backed by in-process Rust SSOT
    public var workspacesJson: @Sendable () -> String
    public var blocksJson: @Sendable () -> String
    public var refsJson: @Sendable () -> String
    public var nestChildrenJson: @Sendable () -> String
    public var backlinksJson: @Sendable () -> String
    public var lastOpIdsJson: @Sendable () -> String
    public var lastOpId: @Sendable (_ wsId: String) -> Int64

    // ── Sync per-id readers
    public var getBlockJson: @Sendable (_ id: String) -> String?
    public var listChildrenJson: @Sendable (_ parentId: String) -> String
    public var listBacklinksJson: @Sendable (_ targetId: String) -> String
    public var typeDefsJson: @Sendable (_ wsId: String) -> String

    public static let live: BlocksCoreClient = liveBlocksCoreClient()
    public static let unimplemented: BlocksCoreClient = unimplementedBlocksCoreClient()
}
