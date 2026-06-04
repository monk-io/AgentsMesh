import SwiftUI
import AgentsMeshCore

/// Top banner shown while the realtime events stream is (re)connecting. Mirrors
/// the web `EventsConnectionBanner`: "connected" is healthy, "disconnected" is
/// the idle/logged-out state (no banner), so only "connecting"/"reconnecting"
/// warn — debounced 5s to avoid a flash on the normal quick reconnect.
struct EventsConnectionBanner: View {
    @ObservedObject private var conn = CoreConnectionStore.shared
    @State private var visible = false

    private var recovering: Bool {
        conn.state == "connecting" || conn.state == "reconnecting"
    }

    var body: some View {
        Group {
            if visible {
                HStack(spacing: 8) {
                    Circle().fill(.white.opacity(0.9)).frame(width: 8, height: 8)
                    Text("Reconnecting to live updates…")
                        .font(.footnote.weight(.medium))
                }
                .foregroundColor(.white)
                .padding(.horizontal, 16)
                .padding(.vertical, 6)
                .frame(maxWidth: .infinity)
                .background(Color.orange.opacity(0.95))
                .transition(.move(edge: .top).combined(with: .opacity))
            }
        }
        .task(id: conn.state) {
            guard recovering else {
                visible = false
                return
            }
            try? await Task.sleep(nanoseconds: 5_000_000_000)
            if !Task.isCancelled {
                visible = true
            }
        }
        .animation(.easeInOut(duration: 0.2), value: visible)
    }
}
