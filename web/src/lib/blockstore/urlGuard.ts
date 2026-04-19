// Client-side URL guardrails for user-authored URLs that end up in
// <iframe src>, <img src>, <a href>. Browsers already refuse javascript:
// on <img> but NOT always on <iframe>, and a misconfigured <a href> with
// javascript: is a one-click XSS. Keep the policy strict and centralised so
// every renderer flows through the same check.

const SAFE_SCHEMES = new Set(["http:", "https:"]);

// isSafeURL returns true only for absolute http/https URLs. Relative paths,
// custom schemes (javascript:, data:, vbscript:, mailto:), and malformed
// strings all fail closed. Callers should show the user an error when this
// returns false rather than silently dropping the value.
export function isSafeURL(raw: string): boolean {
  if (!raw || typeof raw !== "string") return false;
  try {
    const u = new URL(raw);
    return SAFE_SCHEMES.has(u.protocol);
  } catch {
    return false;
  }
}

// sanitizeURL returns the input only if it passes isSafeURL, otherwise an
// empty string. Use for rendering into href/src attributes where an unsafe
// value must not reach the DOM even if isSafeURL was forgotten upstream.
export function sanitizeURL(raw: string): string {
  return isSafeURL(raw) ? raw : "";
}
