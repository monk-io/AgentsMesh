// Relay protocol message-type registry — byte values mirror
// clients/core/crates/protocol (the SSOT codec). Frame encode/decode now lives
// in the Rust relay pool; this enum survives only for consumers that branch on
// an inbound ACP message type (see acpEventDispatcher.ts).
export const MsgType = {
  Snapshot: 0x01,
  Output: 0x02,
  Input: 0x03,
  Resize: 0x04,
  Ping: 0x05,
  Pong: 0x06,
  Control: 0x07,
  RunnerDisconnected: 0x08,
  RunnerReconnected: 0x09,
  Resync: 0x0a,
  AcpEvent: 0x0b,
  AcpCommand: 0x0c,
  AcpSnapshot: 0x0d,
} as const;
