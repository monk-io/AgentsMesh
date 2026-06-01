// Facade re-export of the notification Connect-RPC adapter. Business code
// imports from here (or from the `@/lib/api` barrel) so the wire-shape layer
// stays internal to the facade boundary. Mirrors the pattern used by
// `facade/billingConnect.ts` / `facade/channelConnect.ts` / etc.
//
// Required by `no-restricted-imports` lint rule: business components must
// not import `@/lib/api/connect/*` directly.

export {
  listPreferencesConnect,
  setPreferenceConnect,
  type NotificationPreference,
} from "../connect/notificationConnect";
