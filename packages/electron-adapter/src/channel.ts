import { invoke } from "./invoke";
import type { IChannelService } from "@agentsmesh/service-interface";
import { ChannelLocalState } from "./channel_state";

export class ElectronChannelService extends ChannelLocalState implements IChannelService {
  async fetch_channels(includeArchived?: boolean | null): Promise<string> {
    const result = await invoke<string>("channelFetchChannels", includeArchived);
    const parsed = JSON.parse(result);
    this._channelsCache = JSON.stringify(parsed.channels ?? []);
    return result;
  }

  async fetch_channel(id: bigint): Promise<string> {
    const result = await invoke<string>("channelFetchChannel", Number(id));
    this.update_channel_local(id, result);
    return result;
  }

  async fetch_messages(channelId: bigint, limit?: number | null, beforeId?: bigint | null): Promise<string> {
    const result = await invoke<string>("channelFetchMessages", Number(channelId), limit, beforeId ? Number(beforeId) : null);
    const parsed = JSON.parse(result);
    this._messagesCache.set(String(channelId), JSON.stringify(parsed.messages ?? []));
    return result;
  }

  async fetch_unread_counts(): Promise<string> {
    const result = await invoke<string>("channelFetchUnreadCounts");
    this._unreadCountsCache = result;
    return result;
  }

  async create_channel(json: string): Promise<string> {
    const result = await invoke<string>("channelCreateChannel", json);
    this.add_channel_local(result);
    return result;
  }

  async update_channel(id: bigint, json: string): Promise<string> {
    const result = await invoke<string>("channelUpdateChannel", Number(id), json);
    this.update_channel_local(id, result);
    return result;
  }

  async archive_channel(id: bigint): Promise<void> {
    await invoke<void>("channelArchiveChannel", Number(id));
  }

  async unarchive_channel(id: bigint): Promise<void> {
    await invoke<void>("channelUnarchiveChannel", Number(id));
  }

  async send_message(channelId: bigint, json: string): Promise<string> {
    const result = await invoke<string>("channelSendMessage", Number(channelId), json);
    this.add_message(channelId, result);
    return result;
  }

  async edit_message(channelId: bigint, messageId: bigint, content: string): Promise<string> {
    const result = await invoke<string>("channelEditMessage", Number(channelId), Number(messageId), content);
    this.update_message_local(channelId, result);
    return result;
  }

  async delete_message(channelId: bigint, messageId: bigint): Promise<void> {
    await invoke<void>("channelDeleteMessage", Number(channelId), Number(messageId));
    this.remove_message_local(channelId, messageId);
  }

  async mark_read(channelId: bigint, messageId: bigint): Promise<void> {
    await invoke<void>("channelMarkRead", Number(channelId), Number(messageId));
    this.clear_channel_unread(channelId);
  }

  async mute_channel(channelId: bigint, muted: boolean): Promise<void> {
    await invoke<void>("channelMuteChannel", Number(channelId), muted);
  }

  async join_channel(channelId: bigint, podKey: string): Promise<string> {
    return invoke<string>("channelJoinChannel", Number(channelId), podKey);
  }

  async leave_channel(channelId: bigint, podKey: string): Promise<string> {
    return invoke<string>("channelLeaveChannel", Number(channelId), podKey);
  }

  async get_channel_pods(id: bigint): Promise<string> {
    return invoke<string>("channelGetChannelPods", Number(id));
  }
}
