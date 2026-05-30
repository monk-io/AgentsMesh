export type ConnectionStatus = "connecting" | "connected" | "disconnected" | "error";

export interface ConnectionHandle {
  send: (data: string) => void;
  unsubscribe: () => void;
}

export type RelayStatusInfo = {
  status: ConnectionStatus | "none";
  runnerDisconnected: boolean;
};

export type StatusListener = (info: RelayStatusInfo) => void;
