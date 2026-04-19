import { app, BrowserWindow, ipcMain, shell } from "electron";
import path from "path";
import { AppState } from "@agentsmesh/node-bridge";

const apiUrl = process.env.AGENTSMESH_API_URL ?? "http://localhost:25350";
const storageDir = path.join(app.getPath("userData"), "agentsmesh");

let appState: AppState;

function createWindow() {
  const win = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    title: "AgentsMesh",
    webPreferences: {
      preload: path.join(__dirname, "../preload/index.js"),
      contextIsolation: true,
      sandbox: false,
      nodeIntegration: false,
    },
  });

  win.webContents.setWindowOpenHandler(({ url }) => {
    shell.openExternal(url);
    return { action: "deny" };
  });

  if (process.env.ELECTRON_RENDERER_URL) {
    win.loadURL(process.env.ELECTRON_RENDERER_URL);
    win.webContents.openDevTools({ mode: "detach" });
  } else {
    win.loadFile(path.join(__dirname, "../renderer/index.html"));
  }
}

function registerIpcHandlers() {
  ipcMain.handle("shellOpen", async (_e, url: string) => {
    await shell.openExternal(url);
  });

  const proto = Object.getPrototypeOf(appState);
  const methodNames = Object.getOwnPropertyNames(proto).filter(
    (k) => k !== "constructor" && typeof (appState as any)[k] === "function",
  );
  for (const m of methodNames) {
    ipcMain.handle(m, async (_e, ...args: unknown[]) => {
      try {
        return await (appState as any)[m](...args);
      } catch (err) {
        throw typeof err === "string" ? new Error(err) : err;
      }
    });
  }
  console.log(`[electron] Registered ${methodNames.length} IPC handlers`);
}

app.whenReady().then(() => {
  console.log(`[electron] Starting, API: ${apiUrl}, storage: ${storageDir}`);
  appState = new AppState(apiUrl, storageDir);
  registerIpcHandlers();
  createWindow();

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});
