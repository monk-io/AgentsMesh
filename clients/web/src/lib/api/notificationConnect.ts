// Connect-RPC adapter for proto.notification.v1.NotificationService.
//
// Encodes requests via @bufbuild/protobuf .toBinary(), passes the Uint8Array
// to the wasm bridge (binary in / binary out — conventions §2.5), decodes
// responses via .fromBinary().
//
// The renderer-side `NotificationPreference` type (lib/api/notificationTypes)
// stays in `snake_case`. This adapter is the single point that maps proto
// camelCase ↔ web snake_case so call sites continue to read
// `pref.entity_id` / `pref.is_muted`. Diverging the type surface across the
// codebase is out of scope for the migration PR (runbook V2 pitfall 7).

import {
  ListPreferencesRequestSchema,
  ListPreferencesResponseSchema,
  SetPreferenceRequestSchema,
  NotificationPreferenceSchema,
  type NotificationPreference as ProtoNotificationPreference,
} from "@proto/notification/v1/notification_pb";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import { getNotificationService } from "@/lib/wasm-core";
import type { NotificationPreference } from "@/lib/api";

// Map proto camelCase + `entityId?: string` → web snake_case +
// `entity_id?: string`. Mirrors the legacy REST shape so existing call
// sites can swap to this adapter without per-component refactors.
function toWebPreference(p: ProtoNotificationPreference): NotificationPreference {
  const out: NotificationPreference = {
    source: p.source,
    is_muted: p.isMuted,
    channels: p.channels,
  };
  if (p.entityId !== undefined && p.entityId !== "") {
    out.entity_id = p.entityId;
  }
  return out;
}

export async function listPreferencesConnect(
  orgSlug: string,
): Promise<NotificationPreference[]> {
  const req = create(ListPreferencesRequestSchema, { orgSlug });
  const bytes = toBinary(ListPreferencesRequestSchema, req);
  const respBytes = await getNotificationService().listPreferencesConnect(bytes);
  const resp = fromBinary(ListPreferencesResponseSchema, new Uint8Array(respBytes));
  return resp.items.map(toWebPreference);
}

export async function setPreferenceConnect(
  orgSlug: string,
  pref: NotificationPreference,
): Promise<NotificationPreference> {
  const req = create(SetPreferenceRequestSchema, {
    orgSlug,
    source: pref.source,
    entityId: pref.entity_id,
    isMuted: pref.is_muted,
    channels: pref.channels ?? {},
  });
  const bytes = toBinary(SetPreferenceRequestSchema, req);
  const respBytes = await getNotificationService().setPreferenceConnect(bytes);
  const resp = fromBinary(NotificationPreferenceSchema, new Uint8Array(respBytes));
  return toWebPreference(resp);
}
