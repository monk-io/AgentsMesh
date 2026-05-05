"use client";

import { STEP_LABELS, useLocalRunnerOnboarding } from "@/hooks/useLocalRunnerOnboarding";

export function RegisterLocalRunnerCard() {
  const { unsupported, phase, onRegister } = useLocalRunnerOnboarding();
  if (unsupported) return null;

  if (phase.kind === "idle" && phase.status === "running") {
    return (
      <div className="rounded-lg border border-border bg-accent px-4 py-3 text-[13px] text-accent-foreground">
        <div className="font-medium">This Mac is registered as a Runner</div>
        <div className="mt-0.5 text-muted-foreground">Pods will run locally and skip the cloud relay.</div>
      </div>
    );
  }

  const busy = phase.kind === "installing";
  const stepLabel = phase.kind === "installing" ? STEP_LABELS[phase.step] : null;
  const errorLine = phase.kind === "error"
    ? phase.step
      ? `Failed at ${STEP_LABELS[phase.step]}: ${phase.message}`
      : `Failed: ${phase.message}`
    : null;

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
          {busy ? "Working…" : phase.kind === "error" ? "Retry" : "Register"}
        </button>
        {stepLabel && <span className="text-muted-foreground">{stepLabel}</span>}
        {errorLine && <span className="text-destructive">{errorLine}</span>}
      </div>
    </div>
  );
}
