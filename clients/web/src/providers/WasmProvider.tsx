"use client";

import { useEffect, useState } from "react";
import { initWasmCore } from "@/lib/wasm-core";
import { useAuthStore } from "@/stores/auth";

function isEmbedRoute(): boolean {
  if (typeof window === "undefined") return false;
  return window.location.pathname.startsWith("/blocks-embed");
}

export function WasmProvider({ children }: { children: React.ReactNode }) {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    if (isEmbedRoute()) {
      // Embed routes (e.g. iOS WKWebView) use a different platform init
      // (RPC bridge) that the embed page itself wires up. Skipping WASM
      // here prevents two competing `setPlatformInit` providers and
      // avoids spinning up a second Rust core inside the WebView.
      Promise.resolve().then(() => {
        useAuthStore.getState().setHasHydrated(true);
        setReady(true);
      });
      return;
    }
    initWasmCore().then(() => {
      useAuthStore.getState().setHasHydrated(true);
      setReady(true);
    });
  }, []);

  if (!ready) return null;
  return <>{children}</>;
}
