import Foundation
import AgentsMeshCore

/// Construction site for the `live` and `unimplemented` `BlocksCoreClient`
/// instances. Pulled out of `BlocksCoreClient.swift` to keep that file
/// focused on the public surface and stay under the 200-line cap.

func liveBlocksCoreClient() -> BlocksCoreClient {
    let core = { CoreBridge.shared.core }
    return BlocksCoreClient(
        applyOps: { json in try await core().blocksApplyOps(reqJson: json) },
        loadSubtree: { ws, root in try await core().blocksLoadSubtree(workspaceId: ws, rootId: root) },
        loadTypeDefs: { ws in try await core().blocksLoadTypeDefs(workspaceId: ws) },
        catchup: { ws in try await core().blocksCatchup(workspaceId: ws) },
        applyRemoteOp: { json in try core().blocksApplyRemoteOp(opJson: json) },
        setLastOpId: { ws, id in core().blocksSetLastOpId(workspaceId: ws, id: id) },

        ensureDefaultWorkspace: { try await core().blocksEnsureDefaultWorkspace() },
        listWorkspaces: { try await core().blocksListWorkspaces() },
        getSubtree: { ws, root, depth in
            try await core().blocksGetSubtree(workspaceId: ws, rootId: root, maxDepth: depth)
        },
        getBlock: { id in try await core().blocksGetBlock(id: id) },
        semanticSearch: { ws, req in
            try await core().blocksSemanticSearch(workspaceId: ws, req: req)
        },

        ensureDefaultWorkspaceJson: { try await core().blocksEnsureDefaultWorkspaceJson() },
        listWorkspacesJson: { try await core().blocksListWorkspacesJson() },
        semanticSearchJson: { ws, json in
            try await core().blocksSemanticSearchJson(workspaceId: ws, reqJson: json)
        },

        workspacesJson: { core().blocksWorkspacesJson() },
        blocksJson: { core().blocksBlocksJson() },
        refsJson: { core().blocksRefsJson() },
        nestChildrenJson: { core().blocksNestChildrenJson() },
        backlinksJson: { core().blocksBacklinksJson() },
        lastOpIdsJson: { core().blocksLastOpIdsJson() },
        lastOpId: { ws in core().blocksLastOpId(workspaceId: ws) },

        getBlockJson: { id in core().blocksGetBlockJson(id: id) },
        listChildrenJson: { id in core().blocksListChildrenJson(parentId: id) },
        listBacklinksJson: { id in core().blocksListBacklinksJson(targetId: id) },
        typeDefsJson: { ws in core().blocksTypeDefsJson(workspaceId: ws) }
    )
}

func unimplementedBlocksCoreClient() -> BlocksCoreClient {
    BlocksCoreClient(
        applyOps: { _ in fatalError("unimplemented: blocks.applyOps") },
        loadSubtree: { _, _ in fatalError("unimplemented: blocks.loadSubtree") },
        loadTypeDefs: { _ in fatalError("unimplemented: blocks.loadTypeDefs") },
        catchup: { _ in fatalError("unimplemented: blocks.catchup") },
        applyRemoteOp: { _ in fatalError("unimplemented: blocks.applyRemoteOp") },
        setLastOpId: { _, _ in fatalError("unimplemented: blocks.setLastOpId") },

        ensureDefaultWorkspace: { fatalError("unimplemented: blocks.ensureDefaultWorkspace") },
        listWorkspaces: { fatalError("unimplemented: blocks.listWorkspaces") },
        getSubtree: { _, _, _ in fatalError("unimplemented: blocks.getSubtree") },
        getBlock: { _ in fatalError("unimplemented: blocks.getBlock") },
        semanticSearch: { _, _ in fatalError("unimplemented: blocks.semanticSearch") },

        ensureDefaultWorkspaceJson: { fatalError("unimplemented: blocks.ensureDefaultWorkspaceJson") },
        listWorkspacesJson: { fatalError("unimplemented: blocks.listWorkspacesJson") },
        semanticSearchJson: { _, _ in fatalError("unimplemented: blocks.semanticSearchJson") },

        workspacesJson: { "{}" },
        blocksJson: { "{}" },
        refsJson: { "{}" },
        nestChildrenJson: { "{}" },
        backlinksJson: { "{}" },
        lastOpIdsJson: { "{}" },
        lastOpId: { _ in 0 },

        getBlockJson: { _ in nil },
        listChildrenJson: { _ in "{\"blocks\":[],\"refs\":[]}" },
        listBacklinksJson: { _ in "{\"refs\":[]}" },
        typeDefsJson: { _ in "{\"blocks\":[]}" }
    )
}
