// Unit tests for clients/web/src/lib/api/notificationConnect.ts proto →
// snake_case converter. Pins the field-by-field translation across the
// binary wire so the wire-drift bug class (PR 986a38ca6) is compile-time
// impossible for notification preferences.

import { describe, it, expect, vi, beforeEach } from "vitest";
import { create, toBinary } from "@bufbuild/protobuf";
import {
  NotificationPreferenceSchema,
  ListPreferencesResponseSchema,
} from "@proto/notification/v1/notification_pb";

vi.mock("@/lib/wasm-core", () => ({
  getNotificationService: vi.fn(),
}));

import { getNotificationService } from "@/lib/wasm-core";
import { listPreferencesConnect, setPreferenceConnect } from "../notificationConnect";

describe("listPreferencesConnect", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("decodes empty response to empty list", async () => {
    const respProto = create(ListPreferencesResponseSchema, { items: [], total: BigInt(0) });
    const respBytes = toBinary(ListPreferencesResponseSchema, respProto);
    vi.mocked(getNotificationService).mockReturnValue({
      listPreferencesConnect: vi.fn().mockResolvedValue(respBytes),
    });

    const out = await listPreferencesConnect("acme");
    expect(out).toEqual([]);
  });

  it("maps camelCase proto fields to snake_case web shape", async () => {
    const pref = create(NotificationPreferenceSchema, {
      source: "channel:message",
      entityId: "42",
      isMuted: true,
      channels: { toast: true, browser: false },
    });
    const respProto = create(ListPreferencesResponseSchema, {
      items: [pref],
      total: BigInt(1),
    });
    const respBytes = toBinary(ListPreferencesResponseSchema, respProto);
    vi.mocked(getNotificationService).mockReturnValue({
      listPreferencesConnect: vi.fn().mockResolvedValue(respBytes),
    });

    const out = await listPreferencesConnect("acme");
    expect(out).toHaveLength(1);
    expect(out[0]).toEqual({
      source: "channel:message",
      entity_id: "42",
      is_muted: true,
      channels: { toast: true, browser: false },
    });
  });

  it("treats absent entity_id as undefined (not empty string)", async () => {
    // Source-level preference: proto3 `optional` distinguishes "absent" from
    // "explicit empty string". The web shape uses `entity_id?: string` so
    // absent must be undefined, never "".
    const pref = create(NotificationPreferenceSchema, {
      source: "terminal:osc",
      isMuted: false,
      channels: { toast: true },
    });
    const respProto = create(ListPreferencesResponseSchema, { items: [pref], total: BigInt(1) });
    const respBytes = toBinary(ListPreferencesResponseSchema, respProto);
    vi.mocked(getNotificationService).mockReturnValue({
      listPreferencesConnect: vi.fn().mockResolvedValue(respBytes),
    });

    const out = await listPreferencesConnect("acme");
    expect(out[0].entity_id).toBeUndefined();
  });
});

describe("setPreferenceConnect", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("encodes web snake_case pref to proto camelCase wire", async () => {
    const captured: Uint8Array[] = [];
    const respProto = create(NotificationPreferenceSchema, {
      source: "channel:mention",
      entityId: "42",
      isMuted: true,
      channels: { toast: false, email: true },
    });
    const respBytes = toBinary(NotificationPreferenceSchema, respProto);
    vi.mocked(getNotificationService).mockReturnValue({
      setPreferenceConnect: vi.fn().mockImplementation(async (b: Uint8Array) => {
        captured.push(b);
        return respBytes;
      }),
    });

    const out = await setPreferenceConnect("acme", {
      source: "channel:mention",
      entity_id: "42",
      is_muted: true,
      channels: { toast: false, email: true },
    });

    expect(captured).toHaveLength(1);
    expect(out).toEqual({
      source: "channel:mention",
      entity_id: "42",
      is_muted: true,
      channels: { toast: false, email: true },
    });
  });

  it("sends empty channels map when none provided (server fills defaults)", async () => {
    const respProto = create(NotificationPreferenceSchema, {
      source: "channel:message",
      isMuted: false,
      channels: { toast: true, browser: true },
    });
    const respBytes = toBinary(NotificationPreferenceSchema, respProto);
    vi.mocked(getNotificationService).mockReturnValue({
      setPreferenceConnect: vi.fn().mockResolvedValue(respBytes),
    });

    const out = await setPreferenceConnect("acme", {
      source: "channel:message",
      is_muted: false,
      channels: {},
    });
    expect(out.channels).toEqual({ toast: true, browser: true });
    expect(out.entity_id).toBeUndefined();
  });
});
