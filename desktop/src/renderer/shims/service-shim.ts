export {
  NOOP_PROXY, isServiceReady as isWasmReady,
  registerServiceProvider, parseWasmAny,
  ensurePlatformReady as initWasmCore,
  getApiClient, getAuthManager, getPodState, getPodService,
  getTicketService, getChannelService, getRunnerService,
  getLoopService, getAutopilotService, getMeshService,
  getBillingService, getRepositoryService, getExtensionService,
  getInvitationService, getApiKeyService, getBindingService,
  getMessageService, getNotificationService, getPromoCodeService,
  getTokenUsageService, getSSOService, getUserApiService,
  getUserCredentialService, getOrgApiService, getAgentService,
  getTicketRelationsService, getFileService, getSupportTicketService,
  getAuthApiService, getRunnerState, getMeshState, getTicketState,
  getChannelState, getLoopState, getAcpManager, getOrgState,
  getUserState, getGitProviderState, getRepoState,
  getAutopilotState, getRelayManager,
} from "@agentsmesh/service-runtime";

// Desktop uses Tauri IPC for relay — these are no-ops
const noop = () => new Uint8Array(0);
export const relay_encode_input = noop;
export const relay_encode_resize = noop;
export const relay_encode_ping = noop;
export const relay_encode_control = noop;
export const relay_encode_resync = noop;
export const relay_encode_acp_command = noop;
export const relay_decode_message = () => ({ type: 0, data: new Uint8Array(0) });
