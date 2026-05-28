// Realtime EventBus subscription helper for e2e specs.
//
// Wraps `streamConnect()` against the proto.events.v1.EventsService stream
// the production renderer also consumes. The handful of patterns below
// cover every variant of "trigger an API mutation then assert the
// corresponding `*:status_changed` / `*:created` / etc. event landed on
// the wire with the expected proto-typed payload".
//
// Why not drive the renderer's EventSubscriptionManager directly: this
// helper proves backend → wire fidelity *without* dragging React/Zustand/
// wasm rehydration into the assertion. The store-side propagation is
// covered by the multi-tab specs separately (Phase 2).
import {
  SubscribeRequestSchema,
  EventSchema,
  type Event,
} from "../../../../proto/gen/ts/events/v1/events_pb";

import { streamConnect } from "./connect-stream";

const SETTLE_MS_BEFORE_PUBLISH = 500;

export interface EventStreamOpts {
  token: string;
  orgSlug: string;
  signal: AbortSignal;
}

/**
 * Open a Subscribe stream and yield every event in arrival order until the
 * caller aborts (via `opts.signal`) or the backend closes. Errors during
 * stream open propagate; mid-stream errors after the first frame surface
 * as a return (Connect's standard behavior — see connect-stream.ts).
 */
export async function* subscribeEvents(opts: EventStreamOpts): AsyncGenerator<Event> {
  for await (const ev of streamConnect(
    "proto.events.v1.EventsService",
    "Subscribe",
    SubscribeRequestSchema,
    EventSchema,
    { orgSlug: opts.orgSlug },
    { token: opts.token, signal: opts.signal },
  )) {
    yield ev;
  }
}

export interface ExpectedEvent<T = Record<string, unknown>> {
  type: string;
  data: T;
}

export interface AwaitEventOpts {
  token: string;
  orgSlug: string;
  predicate: (type: string, data: Record<string, unknown>) => boolean;
  timeoutMs?: number;
}

/**
 * Subscribe, run the action, and resolve with the first event whose
 * (type, data) satisfies `predicate`. Rejects with a descriptive error
 * on timeout or stream open failure.
 *
 * Settles for SETTLE_MS_BEFORE_PUBLISH between subscribe open and the
 * action firing — without it the backend hub may fan-out the event
 * before our subscriber is in its registry, causing a flaky miss.
 */
export async function withEventSubscription<R, T extends Record<string, unknown> = Record<string, unknown>>(
  opts: AwaitEventOpts,
  action: () => Promise<R>,
): Promise<{ event: ExpectedEvent<T>; actionResult: R }> {
  const timeout = opts.timeoutMs ?? 10_000;
  const ctrl = new AbortController();

  let captured: ExpectedEvent<T> | null = null;
  let actionResult: R | undefined;

  const drain = (async () => {
    try {
      for await (const ev of subscribeEvents({
        token: opts.token,
        orgSlug: opts.orgSlug,
        signal: ctrl.signal,
      })) {
        let data: Record<string, unknown>;
        try {
          data = JSON.parse(ev.dataJson) as Record<string, unknown>;
        } catch {
          data = {};
        }
        if (opts.predicate(ev.type, data)) {
          captured = { type: ev.type, data: data as T };
          ctrl.abort();
          return;
        }
      }
    } catch {
      /* abort or clean close */
    }
  })();

  await new Promise((r) => setTimeout(r, SETTLE_MS_BEFORE_PUBLISH));

  actionResult = await action();

  const result = await Promise.race([
    drain.then(() => "drained" as const),
    new Promise<"timeout">((r) => setTimeout(() => r("timeout"), timeout)),
  ]);
  ctrl.abort();

  if (result === "timeout" || captured === null) {
    throw new Error(
      `withEventSubscription: timed out after ${timeout}ms waiting for matching event`,
    );
  }
  return { event: captured, actionResult };
}
