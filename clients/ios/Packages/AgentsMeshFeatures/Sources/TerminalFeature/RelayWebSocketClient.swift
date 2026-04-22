import ComposableArchitecture
import Foundation
import AgentsMeshCore

/// Swift-owned Relay WebSocket client. Framing (Input/Resize/Ping/etc.)
/// is computed in Rust via `relay_encode_*`; we just ship the resulting
/// bytes over `URLSessionWebSocketTask`. Incoming binary messages are
/// decoded via `relay_decode_message` and dispatched by `MsgType`:
/// - `Output` (0x01) bytes → `PodOutputDispatcher.feed`
/// - `Ping` (0x05) → silently ack
/// - everything else → ignored for MVP
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
        let actor = RelaySession()
        return RelayWebSocketClient(
            connect: { info, podKey, cols, rows in
                try await actor.connect(info: info, podKey: podKey, cols: cols, rows: rows)
            },
            sendInput: { data in try await actor.sendInput(data) },
            sendResize: { cols, rows in try await actor.sendResize(cols: cols, rows: rows) },
            disconnect: { await actor.disconnect() }
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

/// Owns the active URLSessionWebSocketTask + reader loop. `actor` gives
/// us serialized access to mutable task state from TCA `.run` closures.
actor RelaySession {
    enum RelayError: Error {
        case notConnected
        case connectionClosed(String?)
    }

    private var task: URLSessionWebSocketTask?
    private var podKey: String?
    private var readerTask: Task<Void, Never>?

    func connect(
        info: PodConnectionInfoDto,
        podKey: String,
        cols: UInt16,
        rows: UInt16
    ) async throws {
        await disconnect()
        guard var components = URLComponents(string: info.relayUrl) else {
            throw URLError(.badURL)
        }
        var queryItems = components.queryItems ?? []
        queryItems.append(URLQueryItem(name: "token", value: info.token))
        queryItems.append(URLQueryItem(name: "pod_key", value: info.podKey))
        components.queryItems = queryItems
        guard let url = components.url else { throw URLError(.badURL) }

        let session = URLSession(configuration: .default)
        let t = session.webSocketTask(with: url)
        t.resume()
        self.task = t
        self.podKey = podKey

        // Kick off the reader before sending the initial resize so we
        // don't miss the server's hello frame.
        startReader(task: t, podKey: podKey)

        try await sendResize(cols: cols, rows: rows)
    }

    func sendInput(_ data: Data) async throws {
        guard let t = task else { throw RelayError.notConnected }
        let framed = Data(relayEncodeInput(data: Array(data)))
        try await t.send(.data(framed))
    }

    func sendResize(cols: UInt16, rows: UInt16) async throws {
        guard let t = task else { throw RelayError.notConnected }
        let framed = Data(relayEncodeResize(cols: cols, rows: rows))
        try await t.send(.data(framed))
    }

    func disconnect() async {
        readerTask?.cancel()
        readerTask = nil
        task?.cancel(with: .goingAway, reason: nil)
        task = nil
        if let key = podKey {
            PodOutputDispatcher.shared.unregister(podKey: key)
            podKey = nil
        }
    }

    private func startReader(task: URLSessionWebSocketTask, podKey: String) {
        readerTask = Task.detached { [weak self] in
            while !Task.isCancelled {
                do {
                    let msg = try await task.receive()
                    let data: Data
                    switch msg {
                    case .data(let d): data = d
                    case .string(let s): data = Data(s.utf8)
                    @unknown default: continue
                    }
                    await self?.handleIncoming(data: data)
                } catch {
                    await self?.handleDisconnect(reason: error.localizedDescription)
                    break
                }
            }
        }
    }

    private func handleIncoming(data: Data) async {
        guard let decoded = try? relayDecodeMessage(data: Array(data)) else { return }
        guard let key = podKey else { return }
        // MsgType::Output = 0x01 per agentsmesh_protocol
        switch decoded.kind {
        case 0x01:
            PodOutputDispatcher.shared.feed(podKey: key, data: Data(decoded.payload))
        default:
            break // ignore for MVP
        }
    }

    private func handleDisconnect(reason: String?) async {
        readerTask = nil
        task = nil
    }
}
