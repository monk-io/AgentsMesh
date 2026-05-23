// Wasm-state ChannelMessage projection: keeps the rich AST as opaque strings
// (`content_json` / `mentions_json`) to match the proto-derived cache shape
// in `agentsmesh_types::proto_channel_state_v1::ChannelMessage`. Web's
// `ChannelMessage.content` (parsed `MessageContent`) does not deserialize into
// the wasm side's `content_json: Option<String>` — convert at the boundary.

import type { ChannelMessage } from "@/lib/api/channel";
import type { MessageContent, MessageMentions } from "@/lib/api/channel-message-types";

export type WasmChannelMessage = Omit<ChannelMessage, "content" | "mentions"> & {
  content_json?: string;
  mentions_json?: string;
};

export function toWasmMessage(m: ChannelMessage): WasmChannelMessage {
  const { content, mentions, ...rest } = m;
  const out: WasmChannelMessage = { ...rest };
  if (content) out.content_json = JSON.stringify(content);
  if (mentions) out.mentions_json = JSON.stringify(mentions);
  return out;
}

export function fromWasmMessage(m: WasmChannelMessage): ChannelMessage {
  const { content_json, mentions_json, ...rest } = m;
  const out: ChannelMessage = { ...rest } as ChannelMessage;
  if (content_json) {
    try { out.content = JSON.parse(content_json) as MessageContent; } catch { /* ignore */ }
  }
  if (mentions_json) {
    try { out.mentions = JSON.parse(mentions_json) as MessageMentions; } catch { /* ignore */ }
  }
  return out;
}
