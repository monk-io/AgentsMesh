export class ChannelLocalState {
  _channelsCache = "[]";
  _currentChannelCache: string | null = null;
  _messagesCache = new Map<string, string>();
  _unreadCountsCache = "{}";
  _mentionCountsCache = "{}";

  channels_json(): string { return this._channelsCache; }
  current_channel_json(): unknown { return this._currentChannelCache; }
  unread_counts_json(): string { return this._unreadCountsCache; }
  mention_counts_json(): string { return this._mentionCountsCache; }

  get_channel_json(id: bigint): unknown {
    const chs = JSON.parse(this._channelsCache) as { id: number }[];
    const c = chs.find(x => x.id === Number(id));
    return c ? JSON.stringify(c) : null;
  }

  get_messages_json(channelId: bigint): unknown {
    return this._messagesCache.get(String(channelId)) ?? null;
  }

  get_last_message_json(_channelId: bigint): unknown { return null; }

  get_unread_count(channelId: bigint): number {
    const counts = JSON.parse(this._unreadCountsCache) as Record<string, number>;
    return counts[String(channelId)] ?? 0;
  }

  get_mention_count(channelId: bigint): number {
    const counts = JSON.parse(this._mentionCountsCache) as Record<string, number>;
    return counts[String(channelId)] ?? 0;
  }

  total_unread_count(): number {
    const counts = JSON.parse(this._unreadCountsCache) as Record<string, number>;
    return Object.values(counts).reduce((a, b) => a + b, 0);
  }

  total_mention_count(): number {
    const counts = JSON.parse(this._mentionCountsCache) as Record<string, number>;
    return Object.values(counts).reduce((a, b) => a + b, 0);
  }

  filter_channels_json(_query: string, _includeArchived: boolean): string {
    return this._channelsCache;
  }

  sorted_channel_ids_json(_mode: string, _includeArchived: boolean): string {
    return "[]";
  }

  set_channels(json: string): void { this._channelsCache = json; }
  set_current_channel(id?: bigint | null): void { this._currentChannelCache = id != null ? String(id) : null; }
  set_current_user(_json: string): void {}
  set_current_user_id(_id?: bigint | null): void {}
  set_unread_counts(json: string): void { this._unreadCountsCache = json; }
  set_mention_counts(json: string): void { this._mentionCountsCache = json; }
  set_messages(channelId: bigint, json: string, _hasMore: boolean): void { this._messagesCache.set(String(channelId), json); }
  set_last_message(_channelId: bigint, _json: string): void {}

  add_channel_local(json: string): void {
    const chs = JSON.parse(this._channelsCache) as unknown[];
    chs.push(JSON.parse(json));
    this._channelsCache = JSON.stringify(chs);
  }

  remove_channel_local(id: bigint): void {
    const chs = JSON.parse(this._channelsCache) as { id: number }[];
    this._channelsCache = JSON.stringify(chs.filter(x => x.id !== Number(id)));
  }

  update_channel_local(id: bigint, json: string): void {
    const chs = JSON.parse(this._channelsCache) as { id: number }[];
    const idx = chs.findIndex(x => x.id === Number(id));
    if (idx >= 0) chs[idx] = { ...chs[idx], ...JSON.parse(json) };
    this._channelsCache = JSON.stringify(chs);
  }

  add_message(channelId: bigint, json: string): void {
    const key = String(channelId);
    const msgs = JSON.parse(this._messagesCache.get(key) ?? "[]") as unknown[];
    msgs.push(JSON.parse(json));
    this._messagesCache.set(key, JSON.stringify(msgs));
  }

  remove_message_local(channelId: bigint, messageId: bigint): void {
    const key = String(channelId);
    const msgs = JSON.parse(this._messagesCache.get(key) ?? "[]") as { id: number }[];
    this._messagesCache.set(key, JSON.stringify(msgs.filter(x => x.id !== Number(messageId))));
  }

  update_message_local(channelId: bigint, json: string): void {
    const key = String(channelId);
    const msg = JSON.parse(json) as { id: number };
    const msgs = JSON.parse(this._messagesCache.get(key) ?? "[]") as { id: number }[];
    const idx = msgs.findIndex(x => x.id === msg.id);
    if (idx >= 0) msgs[idx] = msg;
    this._messagesCache.set(key, JSON.stringify(msgs));
  }

  prepend_messages(channelId: bigint, json: string, _hasMore: boolean): void {
    const key = String(channelId);
    const existing = JSON.parse(this._messagesCache.get(key) ?? "[]") as unknown[];
    const newer = JSON.parse(json) as unknown[];
    this._messagesCache.set(key, JSON.stringify([...newer, ...existing]));
  }

  on_new_message(_json: string): boolean { return false; }

  increment_unread(channelId: bigint): void {
    const counts = JSON.parse(this._unreadCountsCache) as Record<string, number>;
    counts[String(channelId)] = (counts[String(channelId)] ?? 0) + 1;
    this._unreadCountsCache = JSON.stringify(counts);
  }

  increment_mention(channelId: bigint): void {
    const counts = JSON.parse(this._mentionCountsCache) as Record<string, number>;
    counts[String(channelId)] = (counts[String(channelId)] ?? 0) + 1;
    this._mentionCountsCache = JSON.stringify(counts);
  }

  clear_channel_unread(channelId: bigint): void {
    const counts = JSON.parse(this._unreadCountsCache) as Record<string, number>;
    delete counts[String(channelId)];
    this._unreadCountsCache = JSON.stringify(counts);
  }

  clear_channel_mentions(channelId: bigint): void {
    const counts = JSON.parse(this._mentionCountsCache) as Record<string, number>;
    delete counts[String(channelId)];
    this._mentionCountsCache = JSON.stringify(counts);
  }

  select_channel(id?: bigint | null): unknown {
    this.set_current_channel(id);
    return this.get_channel_json(id!);
  }
}
