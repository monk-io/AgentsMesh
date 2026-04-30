import Foundation
import AgentsMeshCore
import CoreClient

/// JSON-RPC router for the embedded block-detail WebView.
///
/// Mirrors the `BlockstoreService` interface that web's
/// `clients/web/src/lib/api/blockstoreApi.ts` + `stores/blockstore.ts`
/// expect. Method names are the snake_case keys web sends; JSON wire
/// shape matches `clients/web/src/lib/api/blockstoreTypes.ts` 1:1, so
/// swapping the WASM provider for this RPC bridge is a one-line change
/// in `registerServiceProvider` on the web side.
///
/// Sync flat-map readers (`blocks_json` etc.) hit the in-process Rust
/// SSOT (`AgentsMeshCore.blockstore`) — same backing store mutations
/// land in, so reads always reflect the latest writes.
public protocol IosRpcRoute {
    func dispatch(method: String, args: [String: Any]) async throws -> Any?
}

public struct BlockstoreRpcRoute: IosRpcRoute {
    let core: CoreClient
    public init(core: CoreClient) { self.core = core }

    public func dispatch(method: String, args: [String: Any]) async throws -> Any? {
        switch method {
        // ── Async mutations / fetches
        case "apply_ops":
            guard let req = args["req"] else { throw RpcError.badArgs("req") }
            let json = try jsonString(from: req)
            let result = try await core.blocks.applyOps(json)
            return parsedJSON(result) ?? [:]

        case "load_subtree":
            guard let wsId = args["wsId"] as? String,
                  let rootId = args["rootId"] as? String else {
                throw RpcError.badArgs("wsId+rootId")
            }
            try await core.blocks.loadSubtree(wsId, rootId)
            return NSNull()

        case "load_type_defs":
            guard let wsId = args["wsId"] as? String else { throw RpcError.badArgs("wsId") }
            try await core.blocks.loadTypeDefs(wsId)
            return NSNull()

        case "catchup":
            guard let wsId = args["wsId"] as? String else { throw RpcError.badArgs("wsId") }
            try await core.blocks.catchup(wsId)
            return NSNull()

        case "ensure_default_workspace":
            return parsedJSON(try await core.blocks.ensureDefaultWorkspaceJson()) ?? [:]

        case "list_workspaces":
            return parsedJSON(try await core.blocks.listWorkspacesJson()) ?? ["workspaces": []]

        case "semantic_search":
            guard let wsId = args["wsId"] as? String, let q = args["query"] else {
                throw RpcError.badArgs("wsId+query")
            }
            let qJson = try jsonString(from: q)
            return parsedJSON(try await core.blocks.semanticSearchJson(wsId, qJson)) ?? ["hits": []]

        case "apply_remote_op":
            guard let op = args["op"] else { throw RpcError.badArgs("op") }
            let json = try jsonString(from: op)
            try core.blocks.applyRemoteOp(json)
            return NSNull()

        case "set_last_op_id":
            guard let wsId = args["wsId"] as? String,
                  let id = args["id"] as? Int else { throw RpcError.badArgs("wsId+id") }
            core.blocks.setLastOpId(wsId, Int64(id))
            return NSNull()

        // ── Sync flat-map readers (return strings; web parses)
        case "workspaces_json":     return core.blocks.workspacesJson()
        case "blocks_json":         return core.blocks.blocksJson()
        case "refs_json":           return core.blocks.refsJson()
        case "nest_children_json":  return core.blocks.nestChildrenJson()
        case "backlinks_json":      return core.blocks.backlinksJson()
        case "last_op_ids_json":    return core.blocks.lastOpIdsJson()

        // ── Sync per-id readers
        case "get_block_json":
            guard let id = args["id"] as? String else { throw RpcError.badArgs("id") }
            return core.blocks.getBlockJson(id) ?? NSNull()

        case "list_children_json":
            guard let id = args["id"] as? String else { throw RpcError.badArgs("id") }
            return core.blocks.listChildrenJson(id)

        case "list_backlinks_json":
            guard let id = args["id"] as? String else { throw RpcError.badArgs("id") }
            return core.blocks.listBacklinksJson(id)

        case "type_defs_json":
            guard let wsId = args["wsId"] as? String else { throw RpcError.badArgs("wsId") }
            return core.blocks.typeDefsJson(wsId)

        case "last_op_id":
            guard let wsId = args["wsId"] as? String else { throw RpcError.badArgs("wsId") }
            return Int(core.blocks.lastOpId(wsId))

        default:
            throw RpcError.unknown(method)
        }
    }
}

public enum RpcError: Error, LocalizedError {
    case badArgs(String)
    case unknown(String)

    public var errorDescription: String? {
        switch self {
        case .badArgs(let f): return "Missing or invalid arg: \(f)"
        case .unknown(let m): return "Unknown RPC method: \(m)"
        }
    }
}

private func jsonString(from value: Any) throws -> String {
    let data = try JSONSerialization.data(withJSONObject: value, options: [.fragmentsAllowed])
    return String(data: data, encoding: .utf8) ?? "{}"
}

private func parsedJSON(_ s: String) -> Any? {
    guard let data = s.data(using: .utf8) else { return nil }
    return try? JSONSerialization.jsonObject(with: data, options: [.fragmentsAllowed])
}
