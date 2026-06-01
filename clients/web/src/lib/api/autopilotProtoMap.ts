// AutopilotController (web snake_case) ↔ proto.autopilot_state.v1.AutopilotControllerSnapshot
// Mirror of podProtoMap.ts — denormalises renderer view-model types into the
// proto state mutation contract for binary-bytes dispatch to the wasm bridge.
//
// Used by stores/autopilot.ts to encode mutation requests.

import { create as protoCreate } from "@bufbuild/protobuf";
import {
  AutopilotControllerSnapshotSchema,
  AutopilotIterationSnapshotSchema,
  type AutopilotControllerSnapshot,
  type AutopilotIterationSnapshot,
} from "@proto/autopilot_state/v1/autopilot_state_pb";
import type { AutopilotControllerData, AutopilotIterationData } from "@/lib/viewModels/autopilot";

function asBigInt(v: number | undefined | null): bigint | undefined {
  return v === undefined || v === null ? undefined : BigInt(v);
}

export function controllerToProto(c: AutopilotControllerData): AutopilotControllerSnapshot {
  return protoCreate(AutopilotControllerSnapshotSchema, {
    autopilotControllerKey: c.autopilot_controller_key,
    podKey: c.pod_key,
    phase: c.phase,
    prompt: c.prompt,
    maxIterations: asBigInt(c.max_iterations),
    currentIteration: asBigInt(c.current_iteration),
    circuitBreakerState: c.circuit_breaker?.state,
    circuitBreakerReason: c.circuit_breaker?.reason,
    createdAt: c.created_at,
  });
}

export function iterationToProto(i: AutopilotIterationData): AutopilotIterationSnapshot {
  return protoCreate(AutopilotIterationSnapshotSchema, {
    id: BigInt(i.id),
    controllerKey: i.autopilot_controller_id?.toString() ?? "",
    iterationNumber: asBigInt(i.iteration),
    status: i.phase,
    result: i.summary,
    startedAt: i.created_at,
  });
}
