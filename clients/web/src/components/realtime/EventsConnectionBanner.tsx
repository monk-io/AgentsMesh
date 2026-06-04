"use client";

import { useEffect, useState } from "react";
import { useTranslations } from "next-intl";
import type { ConnectionState } from "@/lib/realtime";

// events 连接进入非 connected 持续超过此时长才显示 banner，避开正常重连的瞬时
// 闪烁；恢复 connected 立即隐藏。与 RelayStatusOverlay（per-pod relay 灯）正交：
// 这条反映的是全局 events 实时流健康，独立于任何终端。
const SHOW_DELAY_MS = 5000;

export function EventsConnectionBanner({
  connectionState,
}: {
  connectionState: ConnectionState;
}) {
  const t = useTranslations("relayStatus");
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    // Only warn while actively (re)connecting. "disconnected" is the idle /
    // logged-out state (no banner on the login page); "connected" is healthy.
    // Both transitions go through a timer so we never setState synchronously
    // inside the effect: recovering shows after a debounce, recovery hides on
    // the next tick.
    const recovering = connectionState === "connecting" || connectionState === "reconnecting";
    const timer = setTimeout(() => setVisible(recovering), recovering ? SHOW_DELAY_MS : 0);
    return () => clearTimeout(timer);
  }, [connectionState]);

  if (!visible) return null;

  return (
    <div
      role="status"
      data-testid="events-connection-banner"
      className="fixed inset-x-0 top-0 z-[60] flex items-center justify-center gap-2 bg-amber-500/95 px-4 py-1.5 text-center text-sm font-medium text-white shadow"
    >
      <span className="inline-block h-2 w-2 animate-pulse rounded-full bg-white/90" />
      {t("eventsReconnecting")}
    </div>
  );
}
