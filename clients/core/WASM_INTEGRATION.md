# AgentsMesh Rust Core — Web (WASM) Integration

## Build

```bash
cd core/crates/wasm
wasm-pack build --target web --out-dir ../../pkg
```

Output in `core/pkg/`:
- `agentsmesh_wasm.js` — JS glue (~22KB)
- `agentsmesh_wasm_bg.wasm` — WASM binary (~137KB)
- `agentsmesh_wasm.d.ts` — TypeScript declarations
- `package.json` — ESM module config

## Usage in Web frontend

```typescript
import init, {
  version,
  WasmPodState,
  WasmTicketState,
  WasmApiClient,
  WasmAuthManager,
  relay_encode_input,
  relay_encode_resize,
  relay_encode_ping,
  relay_encode_control,
  relay_encode_resync,
  relay_encode_acp_command,
  relay_decode_message,
} from './pkg/agentsmesh_wasm';

// Initialize WASM module (required before any other call)
await init();

// Check version
console.log('Core version:', version());
```

### State management

```typescript
// Pod state
const podState = new WasmPodState();
podState.upsert_pod('{"key":"pod-1","status":"running",...}');
const allPods = JSON.parse(podState.pods_json());
const currentPod = podState.current_pod_json(); // returns JS value or null
podState.update_pod_status('pod-1', 'terminated');
podState.remove_pod('pod-1');

// Ticket state
const ticketState = new WasmTicketState();
ticketState.set_tickets('[{"slug":"T-1",...}]');
const tickets = JSON.parse(ticketState.tickets_json());
const labels = JSON.parse(ticketState.labels_json());
const columns = JSON.parse(ticketState.board_columns_json());
```

### Relay protocol

```typescript
// Encode terminal input
const inputMsg = relay_encode_input(new TextEncoder().encode('ls -la'));

// Encode terminal resize
const resizeMsg = relay_encode_resize(120, 40);

// Encode ping keepalive
const pingMsg = relay_encode_ping();

// Encode control / ACP command
const ctrlMsg = relay_encode_control(new TextEncoder().encode('signal'));
const acpMsg = relay_encode_acp_command(new TextEncoder().encode('{"cmd":"status"}'));

// Encode resync request
const resyncMsg = relay_encode_resync();

// Decode incoming binary message
const decoded = relay_decode_message(binaryData);
if (decoded) {
  const msgType: number = decoded.type;
  const payload: Uint8Array = decoded.payload;
}
```

## Integration with existing Web (Next.js)

### 1. Copy pkg into the Web project

```bash
# From project root
cp -r core/pkg web/src/lib/wasm/
```

### 2. Configure Next.js for WASM

```typescript
// next.config.ts
const nextConfig = {
  webpack(config) {
    config.experiments = { ...config.experiments, asyncWebAssembly: true };
    return config;
  },
};
```

### 3. Replace Zustand stores with WASM state

```typescript
// Before: TypeScript-only state management
const usePodStore = create((set) => ({
  pods: [],
  upsertPod: (pod) => { /* complex timestamp guard logic in TS */ },
}));

// After: Rust WASM state management
import init, { WasmPodState } from '@/lib/wasm/agentsmesh_wasm';

let podState: WasmPodState;

async function ensureInit() {
  if (!podState) {
    await init();
    podState = new WasmPodState();
  }
}

const usePodStore = create((set) => ({
  pods: [],
  upsertPod: async (podJson: string, timestamp?: bigint) => {
    await ensureInit();
    podState.upsert_pod(podJson, timestamp);
    set({ pods: JSON.parse(podState.pods_json()) });
  },
}));
```

### 4. Replace relay codec

```typescript
// Before: TypeScript protocol encoder
function encodeInput(data: Uint8Array): ArrayBuffer { /* manual binary packing */ }

// After: Rust WASM relay codec
import { relay_encode_input, relay_decode_message } from '@/lib/wasm/agentsmesh_wasm';

ws.send(relay_encode_input(data));
ws.onmessage = (e) => {
  const msg = relay_decode_message(new Uint8Array(e.data));
  if (msg) handleMessage(msg.type, msg.payload);
};
```

## Memory management

WASM structs (`WasmPodState`, `WasmTicketState`, etc.) allocate memory on the WASM heap. Call `.free()` when done, or use `using` with `Symbol.dispose`:

```typescript
// Manual cleanup
const state = new WasmPodState();
try { /* use state */ } finally { state.free(); }

// Or with TC39 Explicit Resource Management
using state = new WasmPodState();
// auto-freed at end of scope
```

For long-lived stores (typical in Next.js apps), keep a single instance and skip manual cleanup.
