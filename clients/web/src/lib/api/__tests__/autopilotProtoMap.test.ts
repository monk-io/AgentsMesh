// autopilotProtoMap converter test — AutopilotControllerData /
// AutopilotIterationData (snake_case view-model) → AutopilotControllerSnapshot
// / AutopilotIterationSnapshot (camelCase + BigInt). Verifies field-name
// translation + the wire-schema gap that the protoMap respects (state-side
// model has more fields than the wire-side proto.autopilot.v1.AutopilotController).

import { describe, it, expect } from "vitest";
import { controllerToProto, iterationToProto } from "../autopilotProtoMap";
import type {
  AutopilotControllerData, AutopilotIterationData,
} from "@/lib/viewModels/autopilot";

const ctrlFixture: AutopilotControllerData = {
  id: 1,
  autopilot_controller_key: "ctrl-1",
  pod_key: "pod-xyz",
  phase: "running",
  current_iteration: 3,
  max_iterations: 10,
  circuit_breaker: { state: "closed", reason: undefined },
  user_takeover: false,
  prompt: "fix the bug",
  started_at: "2026-05-27T10:00:00Z",
  last_iteration_at: "2026-05-27T10:05:00Z",
  created_at: "2026-05-27T09:00:00Z",
};

const iterFixture: AutopilotIterationData = {
  id: 7,
  autopilot_controller_id: 1,
  iteration: 2,
  phase: "completed",
  summary: "step done",
  files_changed: ["a.ts"],
  duration_ms: 1234,
  created_at: "2026-05-27T10:01:00Z",
};

describe("controllerToProto", () => {
  it("translates snake_case scalars into the snapshot schema", () => {
    const proto = controllerToProto(ctrlFixture);
    expect(proto.autopilotControllerKey).toBe("ctrl-1");
    expect(proto.podKey).toBe("pod-xyz");
    expect(proto.phase).toBe("running");
    expect(proto.prompt).toBe("fix the bug");
    expect(proto.createdAt).toBe("2026-05-27T09:00:00Z");
  });

  it("converts iteration counts to BigInt for int64 wire fields", () => {
    const proto = controllerToProto(ctrlFixture);
    expect(proto.maxIterations).toBe(BigInt(10));
    expect(proto.currentIteration).toBe(BigInt(3));
  });

  it("unfolds circuit_breaker.{state,reason} into flat snapshot fields", () => {
    const proto = controllerToProto(ctrlFixture);
    expect(proto.circuitBreakerState).toBe("closed");
    expect(proto.circuitBreakerReason).toBeUndefined();
  });

  it("preserves a circuit-breaker reason when set", () => {
    const tripped: AutopilotControllerData = {
      ...ctrlFixture,
      circuit_breaker: { state: "open", reason: "too many errors" },
    };
    const proto = controllerToProto(tripped);
    expect(proto.circuitBreakerState).toBe("open");
    expect(proto.circuitBreakerReason).toBe("too many errors");
  });
});

describe("iterationToProto", () => {
  it("translates snake_case scalars into the iteration snapshot", () => {
    const proto = iterationToProto(iterFixture);
    expect(proto.id).toBe(BigInt(7));
    expect(proto.iterationNumber).toBe(BigInt(2));
    expect(proto.status).toBe("completed");
    expect(proto.result).toBe("step done");
    expect(proto.startedAt).toBe("2026-05-27T10:01:00Z");
  });

  it("encodes controllerKey from autopilot_controller_id via toString", () => {
    const proto = iterationToProto(iterFixture);
    // The view-model lacks a controller-key string field on iterations —
    // the protoMap synthesises one from the numeric controller id.
    expect(proto.controllerKey).toBe("1");
  });

  it("emits empty controllerKey when autopilot_controller_id absent", () => {
    const orphan: AutopilotIterationData = { ...iterFixture, autopilot_controller_id: undefined as unknown as number };
    const proto = iterationToProto(orphan);
    expect(proto.controllerKey).toBe("");
  });
});
