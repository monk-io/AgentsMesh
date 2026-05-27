// Facade re-export of the token-usage Connect-RPC adapter. Business code
// imports from here so the wire-shape layer stays internal to the facade
// boundary. Mirrors `facade/billingConnect.ts` / `facade/notificationConnect.ts`.
//
// Required by `no-restricted-imports` lint rule: business components must
// not import `@/lib/api/connect/*` directly.

export {
  getDashboard,
  type TokenUsageDashboard,
  type DashboardParams,
} from "../connect/tokenUsageConnect";
