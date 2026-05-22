import { app, BrowserWindow, dialog, ipcMain, shell } from "electron";
import path from "path";
import { AppState } from "@agentsmesh/node-bridge";
import { createLocalRunnerStubs, type LocalRunnerStubMap } from "./local_runner_stubs";
import { acquireSingleInstance } from "./single_instance";
import {
  registerProtocol,
  attachSecondInstanceUrlHandler,
  installOpenUrlHandler,
  captureColdLaunchUrl,
  flushPendingUrl,
} from "./oauth_deep_link";
import * as serverConfig from "./server_config";
import { isValidServerUrl } from "../shared/server-config-types";

// SSOT: server_config owns the persisted choice (server.json), main owns
// the cold-start env override policy. Keeping the env-override resolution
// out of server_config means that module stays a pure function library
// (easier to test, harder to misuse from `serverConfig:set` handler).
//
// Cold-start resolution order:
//   1. AGENTSMESH_API_URL env (dev / e2e / ad-hoc) — does NOT pollute the
//      dialog SSOT, just redirects requests for this process lifetime.
//   2. activeUrl(currentCfg) — server.json or DEFAULT.
//
// Once the user saves through the dialog, `serverConfig:set` rebinds to
// `activeUrl(next)` directly, shadowing the env override.

// Stashed at module load (before app.ready); flushed in whenReady so users
// actually see the dialog. Calling dialog.showErrorBox at module load is
// unreliable — most Electron dialog APIs assume app.ready has fired.
let pendingStartupErrorMsg: string | null = null;

function resolveColdStartApiUrl(cfg: serverConfig.ServerConfig): string {
  const envOverride = process.env.AGENTSMESH_API_URL;
  if (envOverride && isValidServerUrl(envOverride)) return envOverride;
  try {
    return serverConfig.activeUrl(cfg);
  } catch (e) {
    pendingStartupErrorMsg = (e as Error).message;
    return serverConfig.activeUrl(serverConfig.DEFAULT);
  }
}

let currentCfg = serverConfig.load();
let currentApiUrl = resolveColdStartApiUrl(currentCfg);

const storageDir = path.join(app.getPath("userData"), "agentsmesh");

// Headless flag for e2e: keeps the window invisible + drops the macOS
// dock icon so the test process doesn't steal focus from the user's IDE.
const isHeadlessTest = process.env.NODE_ENV === "test";

let appState: AppState;
let stubs: LocalRunnerStubMap | null = null;
let mainWindow: BrowserWindow | null = null;
const appStateHandlers = new Set<string>();

const getMainWindow = () => mainWindow;

if (acquireSingleInstance()) {
  registerProtocol();
  attachSecondInstanceUrlHandler(getMainWindow);
  captureColdLaunchUrl();
}

function createWindow() {
  const win = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    title: "AgentsMesh",
    show: !isHeadlessTest,
    paintWhenInitiallyHidden: true,
    skipTaskbar: isHeadlessTest,
    webPreferences: {
      preload: path.join(__dirname, "../preload/index.js"),
      contextIsolation: true,
      sandbox: false,
      nodeIntegration: false,
    },
  });

  win.webContents.setWindowOpenHandler(({ url }) => {
    if (/^https?:\/\//i.test(url) || url.startsWith("mailto:") || url.startsWith("agentsmesh://")) {
      shell.openExternal(url);
    }
    return { action: "deny" };
  });

  if (process.env.ELECTRON_RENDERER_URL) {
    win.loadURL(process.env.ELECTRON_RENDERER_URL);
    win.webContents.openDevTools({ mode: "detach" });
  } else {
    win.loadFile(path.join(__dirname, "../renderer/index.html"));
  }

  mainWindow = win;
  win.on("closed", () => {
    if (mainWindow === win) mainWindow = null;
  });
  flushPendingUrl(getMainWindow);
}

// Re-bind every IPC channel that fronts an `AppState` method. Called once
// at boot and again when the user switches server (which constructs a new
// AppState bound to the new base_url). Must `removeHandler` first because
// `ipcMain.handle` throws on duplicate registration.
function bindAppStateHandlers() {
  for (const ch of appStateHandlers) {
    ipcMain.removeHandler(ch);
  }
  appStateHandlers.clear();

  const proto = Object.getPrototypeOf(appState);
  const methodNames = Object.getOwnPropertyNames(proto).filter(
    (k) => k !== "constructor" && typeof (appState as any)[k] === "function",
  );
  for (const m of methodNames) {
    ipcMain.handle(m, async (_e, ...args: unknown[]) => {
      try {
        if (stubs && m in stubs) {
          return await stubs[m](...args);
        }
        return await (appState as any)[m](...args);
      } catch (err) {
        throw typeof err === "string" ? new Error(err) : err;
      }
    });
    appStateHandlers.add(m);
  }
  console.log(`[electron] Registered ${methodNames.length} IPC handlers`);
}

// Replace the running AppState with one bound to a new base_url. Used when
// the user picks a different server. Old service instances inside the prior
// AppState are dropped naturally once their last `Arc` ref expires
// (in-flight requests fade out without aborting).
//
// Known limitation: Rust services without explicit shutdown logic
// (LocalRunnerManager, RelayManager) may keep tokio tasks alive past
// the rebind, leaking until process exit. Fine in practice because
// server switches are rare; would need graceful shutdown plumbing in
// Rust core to fix properly.
function rebindAppState(newApiUrl: string) {
  console.log(`[electron] Rebinding AppState: ${currentApiUrl} → ${newApiUrl}`);
  appState = new AppState(newApiUrl, storageDir);
  // Stubs are scoped to the AppState that owns them — re-create alongside.
  // (isHeadlessTest itself is fixed at startup; the conditional here is
  // not a runtime check, just a "if we needed stubs before, we need them now".)
  if (isHeadlessTest) {
    stubs = createLocalRunnerStubs();
  }
  bindAppStateHandlers();
  currentApiUrl = newApiUrl;
}

function registerStaticHandlers() {
  ipcMain.handle("shellOpen", async (_e, url: string) => {
    if (/^https?:\/\//i.test(url) || url.startsWith("mailto:") || url.startsWith("agentsmesh://")) {
      await shell.openExternal(url);
    }
  });

  // server-config IPC. The sync variants are deliberate: preload reads
  // them at boot to populate `window.electronAPI.apiUrl` / serverConfig
  // .snapshot synchronously, so renderer's getApiBaseUrl() stays
  // synchronous (no async ceremony in OAuth/WS hot paths).
  //
  // The Electron sync-IPC anti-pattern warning is about runtime calls
  // that block the renderer's UI thread; here the call happens during
  // preload BEFORE any renderer code runs, so there's no UI to block.
  // On `mainWindow.reload()`, preload re-executes and re-reads — that's
  // also how we propagate `serverConfig:set` updates without a separate
  // mutable IPC channel.
  //
  // Invariant: registerStaticHandlers() MUST run before createWindow()
  // so the sync handlers exist when preload sends. Enforced by ordering
  // in app.whenReady() below.
  ipcMain.on("serverConfig:getActiveUrlSync", (e) => {
    e.returnValue = currentApiUrl;
  });
  ipcMain.on("serverConfig:getSync", (e) => {
    e.returnValue = currentCfg;
  });
  ipcMain.handle("serverConfig:get", () => serverConfig.load());
  ipcMain.handle("serverConfig:set", async (_e, raw: serverConfig.ServerConfig) => {
    // `raw` is whatever the renderer (or a misbehaving stub) sent over
    // IPC — type-erased at the boundary. Validate via activeUrl first
    // (throws on malformed custom URL → propagates back to dialog), then
    // let `save` normalise + persist. Use `save`'s return value as the new
    // currentCfg so memory and disk stay byte-identical; assigning `raw`
    // directly was the third-round bug.
    serverConfig.activeUrl(raw); // validate, throw on invalid
    const next = serverConfig.save(raw);
    const newUrl = serverConfig.activeUrl(next);
    currentCfg = next;
    if (newUrl !== currentApiUrl) {
      rebindAppState(newUrl);
    }
    // Always reload renderer after a save — preload's sync snapshot of
    // serverConfig is captured at boot, so even when the resolved URL
    // doesn't change (e.g. env override masks both old and new), the
    // *cfg* fields the dialog reads can have changed (kind/custom* etc).
    // Reloading is the cheap way to re-snapshot without a separate
    // mutable IPC channel.
    mainWindow?.reload();
  });

  registerLegacyApiAliases();
}

// Legacy IPC aliases for method names that predate the R6 Connect-RPC
// refactor. Desktop e2e specs still invoke `userGetMe` /
// `autopilotFetchControllers` / `runnerFetchRunners` /
// `channelCreateChannel` by name. The Rust napi handlers were renamed
// (and switched to proto binary payloads) without an alias hop, so the
// invokes hit `No handler registered`. We forward through the Connect
// JSON wire here — it preserves the failure-surface details the
// orbstack-port-conflict spec depends on (status + URL in the error
// message) and avoids dragging proto-js into the main bundle.
function registerLegacyApiAliases() {
  const callConnectJson = async (
    service: string,
    method: string,
    payload: unknown = {},
  ): Promise<string> => {
    const url = `${currentApiUrl}/${service}/${method}`;
    const token = (appState as { authGetToken?: () => string | null }).authGetToken?.();
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
      "Connect-Protocol-Version": "1",
    };
    if (token) headers.Authorization = `Bearer ${token}`;
    const res = await fetch(url, {
      method: "POST",
      headers,
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      const body = await res.text().catch(() => "");
      // Surface the standard `auth_expired` token the desktop renderer +
      // e2e specs key off when the backend returned an Unauthorized.
      // The Connect-JSON error envelope is `{"code":"unauthenticated", ...}`
      // — rewrite the code so callers don't need to know two vocabularies.
      const message = body.includes("unauthenticated")
        ? `auth_expired ${res.status} ${url} ${body}`
        : `${res.status} ${res.statusText} ${url} ${body}`;
      throw new Error(message.trim());
    }
    return await res.text();
  };

  const orgSlug = () => {
    const raw = (appState as { authGetCurrentOrgJson?: () => string | null })
      .authGetCurrentOrgJson?.();
    if (!raw) return "";
    try { return (JSON.parse(raw) as { slug?: string }).slug ?? ""; }
    catch { return ""; }
  };

  ipcMain.handle("userGetMe", () =>
    callConnectJson("proto.user.v1.UserService", "GetMe"),
  );
  ipcMain.handle("autopilotFetchControllers", () =>
    callConnectJson(
      "proto.autopilot.v1.AutopilotControllerService",
      "ListAutopilotControllers",
      { orgSlug: orgSlug() },
    ),
  );
  ipcMain.handle("runnerFetchRunners", () =>
    callConnectJson("proto.runner_api.v1.RunnerService", "ListRunners", {
      orgSlug: orgSlug(),
    }),
  );
  ipcMain.handle("channelCreateChannel", (_e, requestJson: string) => {
    const req = JSON.parse(requestJson) as Record<string, unknown>;
    return callConnectJson(
      "proto.channel.v1.ChannelService",
      "CreateChannel",
      { orgSlug: orgSlug(), ...req },
    );
  });

  // Generic binary Connect-RPC proxy. Web's wasm-side services expose
  // `<method>Connect(Uint8Array) -> Uint8Array`; ElectronXxxService
  // adapters that don't yet have hand-written `_connect` IPC handlers
  // route through this instead. The protobuf encode/decode stays on the
  // renderer; main only ferries bytes (as number[]) over IPC and forwards
  // to the backend Connect endpoint.
  ipcMain.handle("connectCall", async (
    _e, service: string, method: string, bodyArr: number[],
  ) => {
    const url = `${currentApiUrl}/${service}/${method}`;
    const token = (appState as { authGetToken?: () => string | null }).authGetToken?.();
    const headers: Record<string, string> = {
      "Content-Type": "application/proto",
      "Connect-Protocol-Version": "1",
    };
    if (token) headers.Authorization = `Bearer ${token}`;
    const res = await fetch(url, {
      method: "POST",
      headers,
      body: Uint8Array.from(bodyArr),
    });
    if (!res.ok) {
      const text = await res.text().catch(() => "");
      throw new Error(`${res.status} ${res.statusText} ${url} ${text}`.trim());
    }
    const bytes = new Uint8Array(await res.arrayBuffer());
    return Array.from(bytes);
  });
}

app.whenReady().then(() => {
  console.log(`[electron] Starting, API: ${currentApiUrl}, storage: ${storageDir}`);
  if (isHeadlessTest && process.platform === "darwin") {
    app.dock?.hide();
  }
  // Surface any startup error stashed during module load (server.json
  // pointed at a malformed custom URL etc.). Dialog APIs need app.ready,
  // which is why we deferred this from `resolveColdStartApiUrl`.
  if (pendingStartupErrorMsg) {
    dialog.showErrorBox(
      "Invalid server configuration",
      `${pendingStartupErrorMsg}\n\nFalling back to AgentsMesh Global. Open Server Settings to pick a different server.`,
    );
    pendingStartupErrorMsg = null;
  }
  appState = new AppState(currentApiUrl, storageDir);
  if (isHeadlessTest) {
    stubs = createLocalRunnerStubs();
  }
  registerStaticHandlers();
  bindAppStateHandlers();
  installOpenUrlHandler(getMainWindow);
  createWindow();

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});
