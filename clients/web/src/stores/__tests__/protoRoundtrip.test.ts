// Proto roundtrip tests — entity-heavy state schemas (runner / autopilot / repo).
//
// Why this test exists: every state-domain migration replaced a
// JSON.stringify → serde_json::from_str bypass with `protoCreate → toBinary`
// on the renderer + prost::Message::decode on the Rust side. A field-name
// typo or BigInt/number mismatch would silently zero-out the field during
// the roundtrip; tsc can't catch this because Schema fields are typed loose
// (optional everywhere). Vitest catches it by encoding a real fixture,
// decoding it back, and asserting the values survive.
//
// Companion file `protoRoundtripOpaque.test.ts` covers the schemas that
// carry an opaque JSON blob (acp / mesh / app / blockstore / auth).

import { describe, it, expect } from "vitest";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";

import {
  ReplaceCachedRunnersRequestSchema,
  ReplaceAvailableRunnersRequestSchema,
  SetCurrentRunnerRequestSchema,
  PatchCachedRunnerRequestSchema,
  RemoveCachedRunnerRequestSchema,
} from "@proto/runner_state/v1/runner_state_pb";
import { RunnerSchema } from "@proto/runner_api/v1/runner_pb";

import {
  ReplaceCachedControllersRequestSchema,
  SetCurrentControllerRequestSchema,
  InsertControllerRequestSchema,
  PatchControllerRequestSchema,
  RemoveControllerRequestSchema,
  ReplaceCachedIterationsRequestSchema,
  AppendIterationRequestSchema,
  UpdateThinkingRequestSchema,
  AutopilotControllerSnapshotSchema,
  AutopilotIterationSnapshotSchema,
} from "@proto/autopilot_state/v1/autopilot_state_pb";

import {
  ReplaceCachedRepositoriesRequestSchema,
  SetCurrentRepoRequestSchema,
  ReplaceBranchesRequestSchema,
  InsertRepositoryRequestSchema,
  PatchRepositoryRequestSchema,
} from "@proto/repo_state/v1/repo_state_pb";
import { RepositorySchema, BranchSchema } from "@proto/repository/v1/repository_pb";

describe("proto roundtrip — runner_state.v1", () => {
  function makeRunner() {
    return create(RunnerSchema, {
      id: BigInt(42),
      nodeId: "node-abc",
      status: "online",
      currentPods: 3,
      maxConcurrentPods: 8,
      isEnabled: true,
    });
  }

  it("ReplaceCachedRunnersRequest carries runners[]", () => {
    const req = create(ReplaceCachedRunnersRequestSchema, { runners: [makeRunner()] });
    const decoded = fromBinary(
      ReplaceCachedRunnersRequestSchema,
      toBinary(ReplaceCachedRunnersRequestSchema, req),
    );
    expect(decoded.runners).toHaveLength(1);
    expect(decoded.runners[0].id).toBe(BigInt(42));
    expect(decoded.runners[0].nodeId).toBe("node-abc");
    expect(decoded.runners[0].status).toBe("online");
  });

  it("ReplaceAvailableRunnersRequest carries runners[]", () => {
    const req = create(ReplaceAvailableRunnersRequestSchema, { runners: [makeRunner()] });
    const decoded = fromBinary(
      ReplaceAvailableRunnersRequestSchema,
      toBinary(ReplaceAvailableRunnersRequestSchema, req),
    );
    expect(decoded.runners[0].nodeId).toBe("node-abc");
  });

  it("SetCurrentRunnerRequest — runner present", () => {
    const req = create(SetCurrentRunnerRequestSchema, { runner: makeRunner() });
    const decoded = fromBinary(
      SetCurrentRunnerRequestSchema,
      toBinary(SetCurrentRunnerRequestSchema, req),
    );
    expect(decoded.runner?.id).toBe(BigInt(42));
  });

  it("SetCurrentRunnerRequest — runner absent (clear)", () => {
    const req = create(SetCurrentRunnerRequestSchema, {});
    const decoded = fromBinary(
      SetCurrentRunnerRequestSchema,
      toBinary(SetCurrentRunnerRequestSchema, req),
    );
    expect(decoded.runner).toBeUndefined();
  });

  it("PatchCachedRunnerRequest carries runner", () => {
    const req = create(PatchCachedRunnerRequestSchema, { runner: makeRunner() });
    const decoded = fromBinary(
      PatchCachedRunnerRequestSchema,
      toBinary(PatchCachedRunnerRequestSchema, req),
    );
    expect(decoded.runner?.id).toBe(BigInt(42));
  });

  it("RemoveCachedRunnerRequest carries runner_id", () => {
    const req = create(RemoveCachedRunnerRequestSchema, { runnerId: BigInt(99) });
    const decoded = fromBinary(
      RemoveCachedRunnerRequestSchema,
      toBinary(RemoveCachedRunnerRequestSchema, req),
    );
    expect(decoded.runnerId).toBe(BigInt(99));
  });
});

describe("proto roundtrip — autopilot_state.v1", () => {
  function makeController() {
    return create(AutopilotControllerSnapshotSchema, {
      autopilotControllerKey: "ctrl-1",
      podKey: "pod-xyz",
      phase: "running",
      maxIterations: BigInt(10),
      currentIteration: BigInt(3),
      circuitBreakerState: "closed",
    });
  }

  function makeIteration() {
    return create(AutopilotIterationSnapshotSchema, {
      id: BigInt(7),
      controllerKey: "ctrl-1",
      iterationNumber: BigInt(2),
      status: "completed",
    });
  }

  it("ReplaceCachedControllersRequest carries controllers[]", () => {
    const req = create(ReplaceCachedControllersRequestSchema, { controllers: [makeController()] });
    const decoded = fromBinary(
      ReplaceCachedControllersRequestSchema,
      toBinary(ReplaceCachedControllersRequestSchema, req),
    );
    expect(decoded.controllers).toHaveLength(1);
    expect(decoded.controllers[0].autopilotControllerKey).toBe("ctrl-1");
    expect(decoded.controllers[0].podKey).toBe("pod-xyz");
    expect(decoded.controllers[0].maxIterations).toBe(BigInt(10));
  });

  it("SetCurrentControllerRequest carries controller", () => {
    const req = create(SetCurrentControllerRequestSchema, { controller: makeController() });
    const decoded = fromBinary(
      SetCurrentControllerRequestSchema,
      toBinary(SetCurrentControllerRequestSchema, req),
    );
    expect(decoded.controller?.podKey).toBe("pod-xyz");
  });

  it("InsertControllerRequest carries controller", () => {
    const req = create(InsertControllerRequestSchema, { controller: makeController() });
    const decoded = fromBinary(
      InsertControllerRequestSchema,
      toBinary(InsertControllerRequestSchema, req),
    );
    expect(decoded.controller?.autopilotControllerKey).toBe("ctrl-1");
  });

  it("PatchControllerRequest carries key + controller", () => {
    const req = create(PatchControllerRequestSchema, {
      autopilotControllerKey: "ctrl-1",
      controller: makeController(),
    });
    const decoded = fromBinary(
      PatchControllerRequestSchema,
      toBinary(PatchControllerRequestSchema, req),
    );
    expect(decoded.autopilotControllerKey).toBe("ctrl-1");
    expect(decoded.controller?.phase).toBe("running");
  });

  it("RemoveControllerRequest carries key", () => {
    const req = create(RemoveControllerRequestSchema, { autopilotControllerKey: "ctrl-1" });
    const decoded = fromBinary(
      RemoveControllerRequestSchema,
      toBinary(RemoveControllerRequestSchema, req),
    );
    expect(decoded.autopilotControllerKey).toBe("ctrl-1");
  });

  it("ReplaceCachedIterationsRequest carries key + iterations[]", () => {
    const req = create(ReplaceCachedIterationsRequestSchema, {
      autopilotControllerKey: "ctrl-1",
      iterations: [makeIteration()],
    });
    const decoded = fromBinary(
      ReplaceCachedIterationsRequestSchema,
      toBinary(ReplaceCachedIterationsRequestSchema, req),
    );
    expect(decoded.iterations[0].id).toBe(BigInt(7));
  });

  it("AppendIterationRequest carries key + iteration", () => {
    const req = create(AppendIterationRequestSchema, {
      autopilotControllerKey: "ctrl-1",
      iteration: makeIteration(),
    });
    const decoded = fromBinary(
      AppendIterationRequestSchema,
      toBinary(AppendIterationRequestSchema, req),
    );
    expect(decoded.iteration?.iterationNumber).toBe(BigInt(2));
  });

  it("UpdateThinkingRequest carries opaque JSON", () => {
    const blob = JSON.stringify({ kind: "thought", text: "considering options" });
    const req = create(UpdateThinkingRequestSchema, {
      autopilotControllerKey: "ctrl-1",
      thinkingJson: blob,
    });
    const decoded = fromBinary(
      UpdateThinkingRequestSchema,
      toBinary(UpdateThinkingRequestSchema, req),
    );
    expect(decoded.thinkingJson).toBe(blob);
  });
});

describe("proto roundtrip — repo_state.v1", () => {
  function makeRepo() {
    return create(RepositorySchema, {
      id: BigInt(7),
      organizationId: BigInt(2),
      providerType: "github",
      providerBaseUrl: "https://github.com",
      httpCloneUrl: "https://github.com/owner/repo.git",
      sshCloneUrl: "git@github.com:owner/repo.git",
      externalId: "owner/repo",
      name: "repo",
      slug: "owner-repo",
      defaultBranch: "main",
      visibility: "private",
      isActive: true,
    });
  }

  it("ReplaceCachedRepositoriesRequest carries repositories[]", () => {
    const req = create(ReplaceCachedRepositoriesRequestSchema, { repositories: [makeRepo()] });
    const decoded = fromBinary(
      ReplaceCachedRepositoriesRequestSchema,
      toBinary(ReplaceCachedRepositoriesRequestSchema, req),
    );
    expect(decoded.repositories).toHaveLength(1);
    expect(decoded.repositories[0].id).toBe(BigInt(7));
    expect(decoded.repositories[0].slug).toBe("owner-repo");
  });

  it("SetCurrentRepoRequest — repository present", () => {
    const req = create(SetCurrentRepoRequestSchema, { repository: makeRepo() });
    const decoded = fromBinary(
      SetCurrentRepoRequestSchema,
      toBinary(SetCurrentRepoRequestSchema, req),
    );
    expect(decoded.repository?.id).toBe(BigInt(7));
  });

  it("SetCurrentRepoRequest — repository absent (clear)", () => {
    const req = create(SetCurrentRepoRequestSchema, {});
    const decoded = fromBinary(
      SetCurrentRepoRequestSchema,
      toBinary(SetCurrentRepoRequestSchema, req),
    );
    expect(decoded.repository).toBeUndefined();
  });

  it("ReplaceBranchesRequest carries branches[]", () => {
    const branch = create(BranchSchema, { name: "main" });
    const req = create(ReplaceBranchesRequestSchema, { branches: [branch] });
    const decoded = fromBinary(
      ReplaceBranchesRequestSchema,
      toBinary(ReplaceBranchesRequestSchema, req),
    );
    expect(decoded.branches[0].name).toBe("main");
  });

  it("InsertRepositoryRequest carries repository", () => {
    const req = create(InsertRepositoryRequestSchema, { repository: makeRepo() });
    const decoded = fromBinary(
      InsertRepositoryRequestSchema,
      toBinary(InsertRepositoryRequestSchema, req),
    );
    expect(decoded.repository?.providerType).toBe("github");
  });

  it("PatchRepositoryRequest carries id + repository", () => {
    const req = create(PatchRepositoryRequestSchema, { id: "7", repository: makeRepo() });
    const decoded = fromBinary(
      PatchRepositoryRequestSchema,
      toBinary(PatchRepositoryRequestSchema, req),
    );
    expect(decoded.id).toBe("7");
    expect(decoded.repository?.id).toBe(BigInt(7));
  });
});
