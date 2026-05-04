"use client";

import { useCallback, useEffect, useState } from "react";
import { getLocalRunnerService, getRunnerService } from "@agentsmesh/service-runtime";
import { getApiClient } from "@/lib/wasm-core";
import type { ILocalRunnerService, LocalRunnerStatus } from "@agentsmesh/service-interface";

type Phase =
  | { kind: "idle"; status: LocalRunnerStatus }
  | { kind: "installing"; step: string }
  | { kind: "error"; status: LocalRunnerStatus; message: string };

const STEP_LABELS: Record<string, string> = {
  install: "Installing runner binary",
  token: "Minting registration token",
  register: "Registering with backend",
  service_install: "Installing OS service",
  service_start: "Starting service",
};

// Used when the backend's recommended-version endpoint can't be reached.
// Bumped per desktop release; backend's value wins when available.
const RUNNER_RELEASE_VERSION_FALLBACK = "0.4.7";

async function fetchLatestRunnerVersion(): Promise<string> {
  try {
    const raw = await getApiClient().get("/api/v1/runners/latest-release");
    const data = JSON.parse(raw);
    if (typeof data?.version === "string" && data.version) return data.version;
  } catch (err) {
    console.warn("[LocalRunner] backend latest-release unavailable, using bundled fallback", err);
  }
  return RUNNER_RELEASE_VERSION_FALLBACK;
}

async function buildArchiveAssetUrl(svc: ILocalRunnerService): Promise<string | null> {
  const target = await svc.host_target();
  if (!target) return null;
  const ext = target.startsWith("windows_") ? "zip" : "tar.gz";
  const version = await fetchLatestRunnerVersion();
  const base = `https://github.com/anthropics/agentsmesh/releases/download/v${version}`;
  return `${base}/agentsmesh-runner_${target}.${ext}`;
}

export function RegisterLocalRunnerCard() {
  const svc = getLocalRunnerService() as ILocalRunnerService | undefined;
  const [phase, setPhase] = useState<Phase>({ kind: "idle", status: "not_installed" });

  const refreshStatus = useCallback(async () => {
    if (!svc) return;
    const status = await svc.service_status();
    setPhase({ kind: "idle", status });
  }, [svc]);

  useEffect(() => {
    void refreshStatus();
  }, [refreshStatus]);

  const onRegister = useCallback(async () => {
    if (!svc) return;
    try {
      if (!(await svc.is_installed())) {
        setPhase({ kind: "installing", step: "install" });
        const url = await buildArchiveAssetUrl(svc);
        if (!url) throw new Error("unsupported platform for runner auto-install");
        await svc.install_binary(url, null);
      }

      setPhase({ kind: "installing", step: "token" });
      const tokenResp = JSON.parse(
        await getRunnerService().create_token(JSON.stringify({ name: "Desktop" })),
      );
      const token: string | undefined = tokenResp?.token;
      if (!token) throw new Error("backend returned empty registration token");

      setPhase({ kind: "installing", step: "register" });
      await svc.register(token);

      setPhase({ kind: "installing", step: "service_install" });
      await svc.service_install();

      setPhase({ kind: "installing", step: "service_start" });
      await svc.service_start();

      await refreshStatus();
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      const status = await svc.service_status().catch(() => "unknown" as const);
      setPhase({ kind: "error", status, message });
    }
  }, [svc, refreshStatus]);

  if (!svc) return null;

  if (phase.kind === "idle" && phase.status === "running") {
    return (
      <div className="rounded-lg border border-border bg-accent px-4 py-3 text-[13px] text-accent-foreground">
        <div className="font-medium">This Mac is registered as a Runner</div>
        <div className="mt-0.5 text-muted-foreground">Pods will run locally and skip the cloud relay.</div>
      </div>
    );
  }

  const busy = phase.kind === "installing";
  const errorMessage = phase.kind === "error" ? phase.message : null;
  const stepLabel = phase.kind === "installing" ? STEP_LABELS[phase.step] ?? phase.step : null;

  return (
    <div className="rounded-lg border border-border bg-card p-4 text-[13px]">
      <div className="font-semibold text-foreground">Register this Mac as a Runner</div>
      <p className="mt-1 max-w-prose text-muted-foreground">
        Run pods locally with no cloud relay round-trip. Installs the agentsmesh-runner binary
        and starts it as a background OS service.
      </p>
      <div className="mt-3 flex items-center gap-3">
        <button
          type="button"
          onClick={onRegister}
          disabled={busy}
          className="h-9 rounded-md bg-primary px-4 text-sm font-semibold text-primary-foreground disabled:cursor-progress disabled:opacity-60 hover:bg-primary-hover"
        >
          {busy ? "Working…" : "Register"}
        </button>
        {stepLabel && <span className="text-muted-foreground">{stepLabel}</span>}
        {errorMessage && (
          <span className="text-destructive">Failed: {errorMessage}</span>
        )}
      </div>
    </div>
  );
}
