import React, { useEffect, useState } from "react";
import { ThemeProvider } from "./ThemeProvider";
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
      const userJson = mgr.get_current_user_json?.();
      const user = userJson ? (typeof userJson === "string" ? JSON.parse(userJson) : userJson) : null;
      const orgsJson = await mgr.fetch_organizations();
      const orgs = JSON.parse(orgsJson);
      if (orgs[0]) getApiClient().set_org_slug(orgs[0].slug);
      useAuthStore.setState({ user, organizations: orgs, currentOrg: orgs[0] || null });
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
