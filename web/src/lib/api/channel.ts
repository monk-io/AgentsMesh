import { getChannelService } from "@/lib/wasm-core";
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
};
