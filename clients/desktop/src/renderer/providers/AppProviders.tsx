import React, { useEffect, useState } from "react";
// Import via the `next-themes` alias (resolves to ./shims/next-themes →
// ./ThemeProvider) so this file and any web cross-imports of `next-themes`
// land on the same module identity. Direct `./ThemeProvider` imports
// can become a separate bundle module from the aliased path, causing
// the two `ThemeContext` instances and "useTheme must be used within
// ThemeProvider" failures we saw at runtime.
import { ThemeProvider } from "next-themes";
import { DesktopIntlProvider } from "./IntlProvider";
import { RealtimeProvider } from "./RealtimeProvider";
import { Toaster } from "sonner";
import { ensurePlatformReady, getAuthManager, getApiClient } from "@agentsmesh/service-runtime";
import { useAuthStore } from "@/stores/auth";

async function restoreSession() {
  await ensurePlatformReady();
  const mgr = getAuthManager();
  try {
    const restored = await mgr.restore_session();
    if (restored) {
      // Auth migrated to Rust SSOT — fetch_organizations() pushes data
      // into the Rust manager; the renderer reads via `useCurrentOrg`
      // / `useAuthOrganizations` which run `useMemo([_tick])` over the
      // Rust getters. Setting `user`/`organizations`/`currentOrg`
      // directly on Zustand was a no-op (those keys no longer exist
      // after the SSOT migration). Bumping `_tick` is what makes the
      // hooks re-evaluate against the freshly fetched data.
      await mgr.fetch_organizations();
      const orgsJson = mgr.get_organizations_json?.();
      const orgs = orgsJson
        ? (typeof orgsJson === "string" ? JSON.parse(orgsJson) : orgsJson)
        : [];
      if (orgs[0]) getApiClient().set_org_slug(orgs[0].slug);
      useAuthStore.setState((s) => ({ _tick: s._tick + 1 }));
    }
  } catch { /* session expired or invalid */ }
  useAuthStore.getState().setHasHydrated(true);
}

function PlatformGate({ children }: { children: React.ReactNode }) {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    restoreSession().then(() => setReady(true));
  }, []);

  if (!ready) return null;
  return <>{children}</>;
}

export function AppProviders({ children }: { children: React.ReactNode }) {
  return (
    <ThemeProvider defaultTheme="system" attribute="class">
      <DesktopIntlProvider>
        <PlatformGate>
          <RealtimeProvider>
            {children}
          </RealtimeProvider>
        </PlatformGate>
        <Toaster richColors position="top-right" />
      </DesktopIntlProvider>
    </ThemeProvider>
  );
}
