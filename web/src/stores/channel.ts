export { useChannelStore, useChannels, useCurrentChannel, type Channel } from "./channelStore";
export { useChannelMessageStore, EMPTY_CACHE, type ChannelMessageCache } from "./channelMessageStore";
export type { ChannelMessageState } from "./channelMessageStore";

import { reconnectRegistry } from "@/lib/realtime";
import { useChannelMessageStore } from "./channelMessageStore";

reconnectRegistry.register({
  name: "channel:unread",
  fn: () => useChannelMessageStore.getState().fetchUnreadCounts?.(),
  priority: "low",
});
