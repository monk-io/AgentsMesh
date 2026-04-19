const DEFAULT_API_URL = "https://agentsmesh.ai";

function getConfiguredUrl(): string {
  // Vite 环境变量优先
  if (import.meta.env.VITE_API_URL) {
    return import.meta.env.VITE_API_URL;
  }
  return DEFAULT_API_URL;
}

export function getApiBaseUrl(): string {
  return getConfiguredUrl();
}

export function getOAuthBaseUrl(): string {
  return getConfiguredUrl();
}

export function getWsBaseUrl(): string {
  const apiUrl = getConfiguredUrl();
  return apiUrl.replace(/^http/, "ws");
}

export function getServerUrl(): string {
  return getConfiguredUrl();
}

export function getServerUrlSSR(): string {
  return getConfiguredUrl();
}
