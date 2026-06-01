import { createElectronServiceProvider } from "@agentsmesh/electron-adapter";
import {
  registerServiceProvider, markServiceReady, setPlatformInit,
} from "@agentsmesh/service-runtime";
import { installRealtimeMirror } from "./realtime-mirror";

setPlatformInit(async () => {
  const provider = createElectronServiceProvider();
  registerServiceProvider(provider);
  markServiceReady();
  installRealtimeMirror();
});

export function isElectron(): boolean {
  return typeof window !== "undefined" && !!(window as any).electronAPI;
}
