// Multi-tab UI propagation for loop_run:started.
//
// FIXME(follow-up): The loop detail page renders the run history list
// keyed by run id. To assert cross-tab UI update we need:
//   (a) a stable testid on the run-history row (LoopRunListItem.tsx),
//       e.g. `data-run-id` so we can select the new run deterministically; or
//   (b) verify via the run count text "N runs" — but that requires the
//       text to live behind a stable testid too.
//
// Wire-level coverage exists in tests/realtime/loop-events-wire.spec.ts
// (loop_run:started + completed). The UI assertion this spec would add is
// "tab A triggers run → tab B's run-history row appears without manual
// reload".
//
// Tracking: follow-up issue "test(e2e-web): loop run multi-tab UI
// propagation" — depends on (a) above.
import { test } from "../../fixtures/index";

test.describe("Loop run · multi-tab UI propagation", () => {
  test.fixme("tab A trigger run → tab B history list adds row", async () => {
    // See file-level comment.
  });
});
