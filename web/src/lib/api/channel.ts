import { getChannelService, getApiClient } from "@/lib/wasm-core";
export type { MentionPayload, ChannelData, ChannelMessage } from "./channelTypes";

export const channelApi = {
  getPods: async (channelId: number) => {
    const json = await getChannelService().get_channel_pods(BigInt(channelId));
    return JSON.parse(json);
  },
  joinPod: async (channelId: number, podKey: string) => {
    const json = await getChannelService().join_channel(BigInt(channelId), podKey);
    return JSON.parse(json);
  },
  leavePod: async (channelId: number, podKey: string) => {
    const json = await getChannelService().leave_channel(BigInt(channelId), podKey);
    return JSON.parse(json);
  },
  // TODO(wasm): move to ChannelService when member APIs land.
  listMembers: async (channelId: number) => {
    const raw = await getApiClient().get(`/api/v1/channels/${channelId}/members`);
    return typeof raw === "string" ? JSON.parse(raw) : raw;
  },
  inviteMembers: async (channelId: number, userIds: number[]) => {
    const raw = await getApiClient().post(`/api/v1/channels/${channelId}/members`, { user_ids: userIds });
    return typeof raw === "string" ? JSON.parse(raw) : raw;
  },
  removeMember: async (channelId: number, userId: number) => {
    const raw = await getApiClient().delete(`/api/v1/channels/${channelId}/members/${userId}`);
    return typeof raw === "string" ? JSON.parse(raw) : raw;
  },
};
