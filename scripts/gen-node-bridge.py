#!/usr/bin/env python3
"""Convert tauri-bridge command files to node-bridge #[napi] methods."""
import re
import sys
from pathlib import Path

TAURI_BRIDGE = Path("core/crates/tauri-bridge/src/commands")
OUT = Path("core/crates/node-bridge/src/commands_gen.rs")

# Map of file -> service field in AppState
SERVICE_MAP = {
    "pod.rs": "pod",
    "ticket.rs": "ticket",
    "ticket_api.rs": "ticket",
    "channel.rs": "channel",
    "channel_state.rs": "channel",
    "channel_api.rs": "channel",
    "runner.rs": "runner",
    "runner_api.rs": "runner",
    "agent.rs": "agent",
    "autopilot.rs": "autopilot",
    "autopilot_api.rs": "autopilot",
    "loop_service.rs": "loop_svc",
    "billing.rs": "billing",
    "extension.rs": "extension",
    "mesh.rs": "mesh",
    "repository.rs": "repository",
    "binding.rs": "binding",
    "message.rs": "message",
    "apikey.rs": "apikey",
    "invitation.rs": "invitation",
    "notification.rs": "notification",
    "org.rs": "org",
    "user.rs": "user",
    "user_credential.rs": "user_credential",
    "ticket_relations.rs": "ticket_relations",
    "support_ticket.rs": "support_ticket",
    "file.rs": "file",
    "auth_api.rs": "auth_api",
    "misc.rs": None,  # misc handled separately
}

# Files already ported manually
SKIP = {"mod.rs", "auth.rs"}

# Pattern: parse #[tauri::command] functions
COMMAND_RE = re.compile(
    r'#\[tauri::command\]\s*'
    r'pub\s+(async\s+)?fn\s+(\w+)\s*\(\s*'
    r'state:\s*State<\'_,\s*AppState>\s*,?\s*'
    r'([^)]*)\)\s*'
    r'->\s*Result<([^,>]+)(?:,\s*String)?\s*>\s*\{'
    r'([^}]+)\}',
    re.DOTALL,
)

# Simpler: match the whole block greedily then parse
SIG_RE = re.compile(
    r'#\[tauri::command\]\s*'
    r'pub\s+(async\s+)?fn\s+(\w+)\s*\(\s*'
    r'state:\s*State<\'_,\s*AppState>\s*,?\s*'
    r'([^)]*)\)\s*'
    r'->\s*([^{]+?)\s*\{',
    re.DOTALL,
)

def parse_fn(text, start_idx):
    """Parse a function starting at start_idx, return (name, is_async, params, return_type, body_end_idx, body)."""
    m = SIG_RE.search(text, start_idx)
    if not m:
        return None
    is_async = bool(m.group(1))
    name = m.group(2)
    params = m.group(3).strip().rstrip(",").strip()
    ret_type = m.group(4).strip()
    # Find matching closing brace
    brace_start = m.end()
    depth = 1
    i = brace_start
    while i < len(text) and depth > 0:
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
        i += 1
    body = text[brace_start:i-1].strip()
    return name, is_async, params, ret_type, i, body

def convert_body(body, service_field, ret_napi):
    """Replace `state.{service}` with `self.{service}`, fix error mapping for napi::Result."""
    body = body.replace(f"state.{service_field}", f"self.{service_field}")
    body = body.replace("|e| e.to_string()", "err")
    # For napi::Result<T>, the final expression returning Result<T, String> needs .map_err(err)
    if ret_napi.startswith("napi::Result"):
        lines = body.strip().split("\n")
        last = lines[-1].rstrip()
        # Cases to handle:
        # - ends with `.await` (e.g., `svc.foo().await`) → append `.map_err(err)`
        # - ends with `serde_json::to_string(...).map_err(err)` → already OK
        # - ends with `Ok(...)` → already OK
        # - ends with `.await?` → already handled with ? operator
        # Skip if already ends with `.map_err(err)` or `Ok(` or `?`
        stripped = last.rstrip(";").rstrip()
        if (stripped.endswith(".await") and not stripped.endswith("?")):
            # Add .map_err(err) — return type already matches
            lines[-1] = last + ".map_err(err)"
            body = "\n".join(lines)
    return body

def transform_return(ret: str) -> str:
    """Convert `Result<X, String>` to `napi::Result<X>`."""
    m = re.match(r'Result<(.+?)\s*,\s*String\s*>', ret.strip())
    if m:
        return f"napi::Result<{m.group(1).strip()}>"
    return ret

def process_file(path: Path, service_field: str):
    """Parse all commands in a tauri-bridge file and generate napi methods."""
    text = path.read_text()
    out = []
    idx = 0
    while True:
        parsed = parse_fn(text, idx)
        if not parsed:
            break
        name, is_async, params, ret, next_idx, body = parsed
        idx = next_idx

        ret_napi = transform_return(ret)
        body_new = convert_body(body, service_field, ret_napi)

        asyn = "async " if is_async else ""
        params_clean = ", ".join(p.strip() for p in params.split(",") if p.strip()) if params else ""
        sig = f"    #[napi]\n    pub {asyn}fn {name}(&self"
        if params_clean:
            sig += f", {params_clean}"
        sig += f") -> {ret_napi} {{\n"
        # Indent body lines
        body_indented = "\n".join("        " + l for l in body_new.strip().split("\n"))
        out.append(sig + body_indented + "\n    }\n")
    return "\n".join(out)

def main():
    sections = []
    for fname in sorted(SERVICE_MAP.keys()):
        if fname in SKIP:
            continue
        service = SERVICE_MAP[fname]
        if service is None:
            continue  # misc handled separately
        p = TAURI_BRIDGE / fname
        if not p.exists():
            continue
        methods = process_file(p, service)
        if methods.strip():
            sections.append(f"    // ===== {fname} =====\n{methods}")

    output = (
        "// Auto-generated from tauri-bridge — DO NOT edit manually\n"
        "// Regenerate with: python3 scripts/gen-node-bridge.py\n\n"
        "use napi_derive::napi;\n"
        "use crate::{AppState, err};\n\n"
        "#[napi]\n"
        "impl AppState {\n"
        + "\n".join(sections)
        + "\n}\n"
    )
    OUT.write_text(output)
    print(f"Generated {OUT} ({len(sections)} service sections)")

if __name__ == "__main__":
    main()
