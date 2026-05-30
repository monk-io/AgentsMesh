import ComposableArchitecture
import Foundation
import AgentsMeshCore

/// Relay terminal data plane for iOS. Delegates to the shared Rust
/// `RelayConnectionPool` via the ffi `RelayManager` — the same pool web drives
/// through WasmRelayManager. The pool owns framing/codec, reconnect/backoff,
/// input dedup, resize debounce, and snapshot replay; Swift keeps only
/// SwiftTerm rendering (fed by `PodOutputDispatcher`) and this thin client.
///
/// The prior Swift-side WebSocket + hand-rolled codec is gone — its MsgType
/// bytes had drifted from the protocol crate; routing all framing through the
/// Rust pool removes that divergence by construction.
public struct RelayWebSocketClient: Sendable {
    public var connect: @Sendable (
        _ info: PodConnectionInfoDto,
        _ podKey: String,
        _ initialCols: UInt16,
        _ initialRows: UInt16
    ) async throws -> Void
    public var sendInput: @Sendable (_ data: Data) async throws -> Void
    public var sendResize: @Sendable (_ cols: UInt16, _ rows: UInt16) async throws -> Void
    public var disconnect: @Sendable () async -> Void
}

extension RelayWebSocketClient: DependencyKey {
    public static let liveValue: RelayWebSocketClient = {
        let session = RelaySession()
        return RelayWebSocketClient(
            connect: { info, podKey, cols, rows in
                try await session.connect(info: info, podKey: podKey, cols: cols, rows: rows)
            },
            sendInput: { data in try await session.sendInput(data) },
            sendResize: { cols, rows in try await session.sendResize(cols: cols, rows: rows) },
            disconnect: { await session.disconnect() }
        )
    }()

    public static let testValue = RelayWebSocketClient(
        connect: unimplemented("RelayWebSocketClient.connect"),
        sendInput: unimplemented("RelayWebSocketClient.sendInput"),
        sendResize: unimplemented("RelayWebSocketClient.sendResize"),
        disconnect: unimplemented("RelayWebSocketClient.disconnect", placeholder: ())
    )
}

public extension DependencyValues {
    var relayWebSocket: RelayWebSocketClient {
        get { self[RelayWebSocketClient.self] }
        set { self[RelayWebSocketClient.self] = newValue }
    }
}

actor RelaySession {
    enum RelayError: Error { case notConnected }

    private let manager = RelayManager()
    private var podKey: String?
    private var subscriptionId: String?

    func connect(
        info: PodConnectionInfoDto,
        podKey: String,
        cols: UInt16,
        rows: UInt16
    ) async throws {
        await disconnect()
        let subId = "terminal-\(podKey)"
        self.podKey = podKey
        self.subscriptionId = subId
        // PodOutputDispatcher routes the Rust OutputCallback by podKey to the
        // attached TerminalView sink. The pool replays a snapshot on subscribe.
        await manager.subscribe(
            podKey: podKey,
            subscriptionId: subId,
            relayUrl: info.relayUrl,
            token: info.token,
            callback: PodOutputDispatcher.shared.callback
        )
        await manager.forceResize(podKey: podKey, cols: cols, rows: rows)
    }

    func sendInput(_ data: Data) async throws {
        guard let key = podKey else { throw RelayError.notConnected }
        await manager.send(podKey: key, data: String(decoding: data, as: UTF8.self))
    }

    func sendResize(cols: UInt16, rows: UInt16) async throws {
        guard let key = podKey else { throw RelayError.notConnected }
        await manager.sendResize(podKey: key, cols: cols, rows: rows)
    }

    func disconnect() async {
        if let key = podKey, let sub = subscriptionId {
            await manager.unsubscribe(podKey: key, subscriptionId: sub)
            PodOutputDispatcher.shared.unregister(podKey: key)
        }
        podKey = nil
        subscriptionId = nil
    }
}
