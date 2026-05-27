// channelProtoMap converter tests — ChannelData / ChannelPodSummary /
// ChannelMemberData (snake_case view-model) → proto.channel_state.v1.*
// (camelCase + BigInt). Companion to channelStore proto tests which cover the
// Channel store-shape mapper. These tests guard the facade-layer mappers used
// by channelApi.update / getPods / listMembers (proto-bytes SSOT path).

import { describe, it, expect } from "vitest";
import {
  channelDataToProtoChannel,
  channelPodSummaryToProtoPod,
  channelMemberDataToProto,
} from "../channelProtoMap";
import type { ChannelData } from "../facade/channel";
import type {
  ChannelMemberData,
  ChannelPodSummary,
} from "../connect/channelMembersConnect";

const channelFixture: ChannelData = {
  id: 7,
  organization_id: 3,
  name: "general",
  description: "main channel",
  document: "doc body",
  repository_id: 11,
  ticket_id: 22,
  ticket_slug: "PROJ-22",
  created_by_pod: "pod-abc",
  created_by_user_id: 5,
  visibility: "public",
  is_archived: false,
  is_member: true,
  member_count: 12,
  agent_count: 3,
  created_at: "2026-05-01T00:00:00Z",
  updated_at: "2026-05-27T10:00:00Z",
};

describe("channelDataToProtoChannel", () => {
  it("maps every snake_case scalar to camelCase + BigInt for int64s", () => {
    const proto = channelDataToProtoChannel(channelFixture);
    expect(proto.id).toBe(BigInt(7));
    expect(proto.organizationId).toBe(BigInt(3));
    expect(proto.name).toBe("general");
    expect(proto.description).toBe("main channel");
    expect(proto.document).toBe("doc body");
    expect(proto.repositoryId).toBe(BigInt(11));
    expect(proto.ticketId).toBe(BigInt(22));
    expect(proto.ticketSlug).toBe("PROJ-22");
    expect(proto.createdByPod).toBe("pod-abc");
    expect(proto.createdByUserId).toBe(BigInt(5));
    expect(proto.visibility).toBe("public");
    expect(proto.isArchived).toBe(false);
    expect(proto.isMember).toBe(true);
    expect(proto.memberCount).toBe(BigInt(12));
    expect(proto.agentCount).toBe(BigInt(3));
    expect(proto.createdAt).toBe("2026-05-01T00:00:00Z");
    expect(proto.updatedAt).toBe("2026-05-27T10:00:00Z");
  });

  it("leaves optional fields undefined when source absent", () => {
    const minimal: ChannelData = {
      id: 1,
      organization_id: 1,
      name: "x",
      visibility: "private",
      is_archived: false,
      is_member: false,
      member_count: 0,
      agent_count: 0,
      created_at: "",
      updated_at: "",
    };
    const proto = channelDataToProtoChannel(minimal);
    expect(proto.repositoryId).toBeUndefined();
    expect(proto.ticketId).toBeUndefined();
    expect(proto.ticketSlug).toBeUndefined();
    expect(proto.createdByPod).toBeUndefined();
    expect(proto.createdByUserId).toBeUndefined();
    expect(proto.description).toBeUndefined();
  });
});

describe("channelPodSummaryToProtoPod", () => {
  it("maps the 5 summary fields onto the wire Pod", () => {
    const summary: ChannelPodSummary = {
      id: 17,
      pod_key: "pod-xyz",
      alias: "worker-1",
      status: "running",
      agent_status: "thinking",
    };
    const proto = channelPodSummaryToProtoPod(summary);
    expect(proto.id).toBe(BigInt(17));
    expect(proto.podKey).toBe("pod-xyz");
    expect(proto.alias).toBe("worker-1");
    expect(proto.status).toBe("running");
    expect(proto.agentStatus).toBe("thinking");
  });

  it("leaves other Pod fields at proto3 defaults (zero / undefined)", () => {
    const summary: ChannelPodSummary = {
      id: 1, pod_key: "k", status: "running", agent_status: "idle",
    };
    const proto = channelPodSummaryToProtoPod(summary);
    expect(proto.runnerId).toBeUndefined();
    expect(proto.repository).toBeUndefined();
    expect(proto.agentSlug).toBe("");
  });
});

describe("channelMemberDataToProto", () => {
  it("maps snake_case fields to camelCase + BigInt for id types", () => {
    const member: ChannelMemberData = {
      channel_id: 7,
      user_id: 99,
      role: "admin",
      is_muted: true,
      joined_at: "2026-05-01T00:00:00Z",
    };
    const proto = channelMemberDataToProto(member);
    expect(proto.channelId).toBe(BigInt(7));
    expect(proto.userId).toBe(BigInt(99));
    expect(proto.role).toBe("admin");
    expect(proto.isMuted).toBe(true);
    expect(proto.joinedAt).toBe("2026-05-01T00:00:00Z");
  });
});
