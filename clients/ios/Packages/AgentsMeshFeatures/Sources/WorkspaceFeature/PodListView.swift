import ComposableArchitecture
import SwiftUI
import AgentsMeshCore
import DesignSystem

public struct PodListView: View {
    let store: StoreOf<PodListFeature>

    public init(store: StoreOf<PodListFeature>) {
        self.store = store
    }

    public var body: some View {
        ZStack {
            AMColors.background.ignoresSafeArea()
            content
        }
        .navigationTitle("Workspace")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button("Sign out") { store.send(.logoutTapped) }
            }
        }
        .onAppear { store.send(.onAppear) }
        .refreshable { store.send(.refreshRequested) }
    }

    @ViewBuilder
    private var content: some View {
        if store.isLoading && store.pods.isEmpty {
            ProgressView().controlSize(.large)
        } else if store.pods.isEmpty {
            emptyState
        } else {
            ScrollView {
                LazyVStack(spacing: AMSpacing.s) {
                    runnerBar
                    ForEach(store.pods, id: \.key) { pod in
                        Button { store.send(.podTapped(pod.key)) } label: {
                            PodRow(pod: pod)
                        }
                        .buttonStyle(.plain)
                    }
                    if let error = store.errorMessage {
                        Text(error).font(AMTypography.caption)
                            .foregroundStyle(AMColors.destructive)
                            .padding(.top, AMSpacing.m)
                    }
                }
                .padding(AMSpacing.l)
            }
        }
    }

    private var runnerBar: some View {
        let onlineCount = store.runners.filter { $0.status == .online }.count
        return AMCard {
            HStack {
                Image(systemName: "server.rack").foregroundStyle(AMColors.mutedForeground)
                VStack(alignment: .leading, spacing: 2) {
                    Text("Runners").font(AMTypography.caption)
                        .foregroundStyle(AMColors.mutedForeground)
                    Text("\(onlineCount) / \(store.runners.count) online")
                        .font(AMTypography.body.weight(.medium))
                }
                Spacer()
            }
        }
    }

    private var emptyState: some View {
        VStack(spacing: AMSpacing.m) {
            Text("No pods yet").font(AMTypography.heading)
            Text("Create a pod from the Web or Desktop app to see it here.")
                .font(AMTypography.body)
                .foregroundStyle(AMColors.mutedForeground)
                .multilineTextAlignment(.center)
                .padding(.horizontal, AMSpacing.xl)
        }
    }
}

private struct PodRow: View {
    let pod: PodDto

    var body: some View {
        AMCard {
            HStack(spacing: AMSpacing.m) {
                StatusDot(status: pod.status)
                VStack(alignment: .leading, spacing: 2) {
                    Text(pod.alias ?? pod.key)
                        .font(AMTypography.body.weight(.medium))
                    Text(pod.agentSlug.isEmpty ? "unknown agent" : pod.agentSlug)
                        .font(AMTypography.caption)
                        .foregroundStyle(AMColors.mutedForeground)
                }
                Spacer()
                if let status = pod.agentStatus, !status.isEmpty {
                    Text(status).font(AMTypography.caption)
                        .foregroundStyle(AMColors.mutedForeground)
                }
            }
        }
    }
}

private struct StatusDot: View {
    let status: PodStatusDto

    var body: some View {
        Circle().fill(color).frame(width: 10, height: 10)
    }

    private var color: Color {
        switch status {
        case .running: return .green
        case .initializing, .creating, .pending: return .orange
        case .paused, .disconnected: return .yellow
        case .terminated, .completed: return AMColors.mutedForeground
        case .error, .failed: return AMColors.destructive
        case .stopping, .orphaned, .unknown: return AMColors.mutedForeground
        }
    }
}
