import { ElectronPodService } from './pod';
import { ElectronRunnerService } from './runner';
import { ElectronTicketService } from './ticket';
import { ElectronChannelService } from './channel';
import { ElectronLoopService } from './loop';
import { ElectronAutopilotService } from './autopilot';
import { ElectronMeshService } from './mesh';
import { ElectronBillingService } from './billing';
import { ElectronExtensionService } from './extension';
import { ElectronRepositoryService } from './repository';
import { ElectronInvitationConnectService } from './invitation';
import { ElectronApiKeyService } from './apikey';
import { ElectronBindingService } from './binding';
import { ElectronNotificationService } from './notification';
import { ElectronOrgService } from './org';
import { ElectronUserService } from './user';
import { ElectronUserCredentialService } from './user_credential';
import { ElectronAgentService } from './agent';
import { ElectronSSOService } from './sso';
import { ElectronFileService } from './file';
import { ElectronSupportTicketService } from './support_ticket';
import { ElectronTicketRelationsService } from './ticket_relations';
import { ElectronTokenUsageService } from './token_usage';
import { ElectronPromoCodeService } from './promocode';
import { ElectronAuthService } from './auth';
import { ElectronAuthConnectService } from './auth_connect';
import { ElectronBlockstoreService } from './blockstore';
import { ElectronLocalRunnerService } from './local_runner';
import { invoke } from './invoke';
import {
  ElectronOrgState, ElectronUserState, ElectronGitProviderState, ElectronRepoState,
  ElectronAcpManager, ElectronRelayManager,
} from './state_adapters';

/**
 * Mirrors `WasmApiClient` shape for legacy callers that still go through
 * `lib/api/base.request`. Every raw HTTP method delegates to node-bridge IPC
 * handlers (`api_get` / `api_post` / ...), which call the shared Rust
 * `ApiClient` with auth token + org slug already bound on the native side.
 *
 * Plan I6 SSOT: orgSlug is read from the AuthManager (single source of
 * truth) instead of being mirrored via `set_org_slug()`. ElectronAuthService
 * caches the current org locally so `org_path()` stays synchronous.
 */
class ElectronApiClientProxy {
  constructor(private readonly auth: ElectronAuthService) {}

  /** Must match Rust `ApiClient::org_path`. */
  org_path(path: string): string {
    const slug = this.auth.get_current_org_slug();
    return slug ? `/api/v1/orgs/${slug}${path}` : `/api/v1${path}`;
  }

  async get(endpoint: string): Promise<string> {
    return invoke<string>("apiGet", endpoint);
  }
  async post(endpoint: string, body: string): Promise<string> {
    return invoke<string>("apiPost", endpoint, body ?? "");
  }
  async put(endpoint: string, body: string): Promise<string> {
    return invoke<string>("apiPut", endpoint, body ?? "");
  }
  async patch(endpoint: string, body: string): Promise<string> {
    return invoke<string>("apiPatch", endpoint, body ?? "");
  }
  async delete(endpoint: string): Promise<string> {
    return invoke<string>("apiDelete", endpoint);
  }

  /**
   * `WasmApiClient` exposes `create_events_manager()` returning a stream
   * subscriber. Desktop doesn't (yet) have a Connect ServerStream bridge
   * over IPC — we return a no-op manager so `EventSubscriptionManager`
   * boots without `is not a function` crashing the renderer. Realtime
   * events silently no-op on desktop until the main-process bridge lands.
   */
  create_events_manager(): NoopEventsManager {
    return new NoopEventsManager();
  }
}

class NoopEventsManager {
  private nextId = 1;
  async subscribe_all(_cb: (json: string) => void): Promise<number> { return this.nextId++; }
  async on_connection_state_change(cb: (state: string) => void): Promise<number> {
    queueMicrotask(() => cb("connected"));
    return this.nextId++;
  }
  async unsubscribe(_id: number): Promise<void> { /* no-op */ }
  async connect(): Promise<void> { /* no-op */ }
  async disconnect(): Promise<void> { /* no-op */ }
}

// user_credential proto bundles three backend services (git credential, agent
// credential, repository provider) under a single wasm-side facade. Map each
// method to the Connect service that actually owns it so calls land in the
// right backend handler.
const USER_CREDENTIAL_METHOD_OVERRIDES: Record<string, string> = {
  listGitCredentials: "proto.user_credential.v1.UserGitCredentialService",
  getGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  createGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  updateGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  deleteGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  getDefaultGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  setDefaultGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  clearDefaultGitCredential: "proto.user_credential.v1.UserGitCredentialService",
  listAgentCredentialProfiles: "proto.user_credential.v1.UserAgentCredentialService",
  listAgentCredentialProfilesForAgent: "proto.user_credential.v1.UserAgentCredentialService",
  getAgentCredentialProfile: "proto.user_credential.v1.UserAgentCredentialService",
  createAgentCredentialProfile: "proto.user_credential.v1.UserAgentCredentialService",
  updateAgentCredentialProfile: "proto.user_credential.v1.UserAgentCredentialService",
  deleteAgentCredentialProfile: "proto.user_credential.v1.UserAgentCredentialService",
  setDefaultAgentCredentialProfile: "proto.user_credential.v1.UserAgentCredentialService",
};

export function createElectronServiceProvider(baseUrl = '') {
  // Services carry the full state (each IXxxService is a superset of IXxxState).
  // The provider returns the same instance for both keys so renderer readers
  // that grab `xxxState` see the cache Service writes to on fetch / upsert.
  // This is what makes Pod / Runner / Mesh / Ticket / Loop / Autopilot actually
  // populate in desktop first-load — previously `xxxState` was a stub class
  // returning `"[]"` that shadowed the real data.
  const podService = new ElectronPodService();
  const runnerService = new ElectronRunnerService();
  const ticketService = new ElectronTicketService();
  const channelService = new ElectronChannelService();
  const loopService = new ElectronLoopService();
  const autopilotService = new ElectronAutopilotService();
  const meshService = new ElectronMeshService();
  // AuthManager is constructed first so ApiClient can borrow it as the org
  // slug source (Plan I6 SSOT). Order matters here.
  const authManager = new ElectronAuthService(baseUrl);

  return {
    apiClient: new ElectronApiClientProxy(authManager),
    authManager,
    podService: withConnectFallback(podService, "proto.pod.v1.PodService"),
    runnerService: withConnectFallback(runnerService, "proto.runner_api.v1.RunnerService"),
    ticketService: withConnectFallback(ticketService, "proto.ticket.v1.TicketService"),
    channelService: withConnectFallback(channelService, "proto.channel.v1.ChannelService"),
    loopService: withConnectFallback(loopService, "proto.loop.v1.LoopService"),
    autopilotService: withConnectFallback(autopilotService, "proto.autopilot.v1.AutopilotControllerService"),
    meshService: withConnectFallback(meshService, "proto.mesh.v1.MeshService"),
    billingService: withConnectFallback(new ElectronBillingService(), "proto.billing.v1.BillingService"),
    extensionService: withConnectFallback(new ElectronExtensionService(), "proto.extension.v1.SkillRegistryService"),
    repositoryService: withConnectFallback(new ElectronRepositoryService(), "proto.repository.v1.RepositoryService"),
    invitationService: withConnectFallback(new ElectronInvitationConnectService(), "proto.invitation.v1.InvitationService"),
    apiKeyService: withConnectFallback(new ElectronApiKeyService(), "proto.apikey.v1.ApiKeyService"),
    bindingService: withConnectFallback(new ElectronBindingService(), "proto.binding.v1.BindingService"),
    notificationService: withConnectFallback(new ElectronNotificationService(), "proto.notification.v1.NotificationService"),
    orgApiService: withConnectFallback(new ElectronOrgService(), "proto.org.v1.OrgService"),
    userApiService: withConnectFallback(new ElectronUserService(), "proto.user.v1.UserService"),
    userCredentialService: withConnectFallback(
      new ElectronUserCredentialService(),
      "proto.user_credential.v1.UserRepositoryProviderService",
      USER_CREDENTIAL_METHOD_OVERRIDES,
    ),
    agentService: withConnectFallback(new ElectronAgentService(), "proto.agent.v1.AgentService"),
    ssoService: withConnectFallback(new ElectronSSOService(), "proto.sso.v1.SSOService"),
    fileService: withConnectFallback(new ElectronFileService(), "proto.file.v1.FileService"),
    supportTicketService: withConnectFallback(new ElectronSupportTicketService(), "proto.support_ticket.v1.SupportTicketService"),
    ticketRelationsService: withConnectFallback(new ElectronTicketRelationsService(), "proto.ticket_relations.v1.TicketRelationsService"),
    tokenUsageService: withConnectFallback(new ElectronTokenUsageService(), "proto.token_usage.v1.TokenUsageService"),
    promoCodeService: withConnectFallback(new ElectronPromoCodeService(), "proto.promocode.v1.PromoCodeService"),
    authConnectService: withConnectFallback(new ElectronAuthConnectService(), "proto.auth.v1.AuthService"),
    blockstoreService: withConnectFallback(new ElectronBlockstoreService(), "proto.blockstore.v1.BlockstoreService"),
    localRunnerService: new ElectronLocalRunnerService(),
    // State facets — share the Service instance, not a separate stub.
    podState: podService,
    runnerState: runnerService,
    meshState: meshService,
    ticketState: ticketService,
    channelState: channelService,
    loopState: loopService,
    autopilotState: autopilotService,
    orgState: new ElectronOrgState(),
    userState: new ElectronUserState(),
    gitProviderState: new ElectronGitProviderState(),
    repoState: new ElectronRepoState(),
    acpManager: new ElectronAcpManager(),
    relayManager: new ElectronRelayManager(),
  };
}

// Web's wasm-side services expose `<verb>Connect(Uint8Array) -> Uint8Array`
// methods generated from the proto schema. Most ElectronXxxService classes
// still only carry the legacy JSON-shaped surface, so renderer code that
// reaches for the proto wire (e.g. `lib/api/channelConnect.ts`) hits
// `<method> is not a function`. This wrapper auto-forwards any
// `*Connect` lookup that the service doesn't implement onto the generic
// `connectCall` IPC handler in main/index.ts, deriving the proto
// method name from the camelCased TS name.
//
// `methodOverrides` lets a single TS-facing service span multiple backend
// Connect services (e.g. user_credential bundles UserGitCredentialService,
// UserAgentCredentialService, and UserRepositoryProviderService — wasm
// dispatches per method but the TS facade is one object).
function withConnectFallback<T extends object>(
  service: T,
  protoPath: string,
  methodOverrides?: Record<string, string>,
): T {
  return new Proxy(service, {
    get(target, prop) {
      const value = Reflect.get(target, prop);
      if (value !== undefined) return value;
      if (typeof prop !== "string" || !prop.endsWith("Connect")) return undefined;
      const camel = prop.slice(0, -"Connect".length);
      const protoMethod = camel.charAt(0).toUpperCase() + camel.slice(1);
      const targetService = methodOverrides?.[camel] ?? protoPath;
      return async (request: Uint8Array): Promise<Uint8Array> => {
        const resp = await invoke<number[] | Uint8Array>(
          "connectCall", targetService, protoMethod, Array.from(request),
        );
        return resp instanceof Uint8Array ? resp : new Uint8Array(resp);
      };
    },
  });
}
