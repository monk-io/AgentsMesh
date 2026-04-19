"use client";

import { useEffect, useState } from "react";
import { initWasmCore } from "@/lib/wasm-core";
import { useAuthStore } from "@/stores/auth";

export function WasmProvider({ children }: { children: React.ReactNode }) {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    initWasmCore().then(() => {
      useAuthStore.getState().setHasHydrated(true);
      setReady(true);
    });
  }, []);

  if (!ready) return null;
  return <>{children}</>;
}
