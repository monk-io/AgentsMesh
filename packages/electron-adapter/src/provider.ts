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
import { ElectronInvitationService } from './invitation';
import { ElectronApiKeyService } from './apikey';
import { ElectronBindingService } from './binding';
import { ElectronMessageService } from './message';
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
import { ElectronAuthApiService } from './auth_api';
import { ElectronAuthService } from './auth';
import {
  ElectronPodState, ElectronRunnerState, ElectronMeshState, ElectronTicketState,
  ElectronChannelState, ElectronLoopState, ElectronAutopilotState, ElectronOrgState,
  ElectronUserState, ElectronGitProviderState, ElectronRepoState,
  ElectronAcpManager, ElectronRelayManager,
} from './state_adapters';

class ElectronApiClientProxy {
  set_token(_token: string, _refresh: string) {}
  set_org_slug(_slug: string) {}
  clear_auth() {}
  get_org_slug(): string | undefined { return undefined; }
  get_token(): string | undefined { return undefined; }
  org_path(path: string): string { return path; }
}

export function createElectronServiceProvider(baseUrl = '') {
  return {
    apiClient: new ElectronApiClientProxy(),
    authManager: new ElectronAuthService(baseUrl),
    podService: new ElectronPodService(),
    runnerService: new ElectronRunnerService(),
    ticketService: new ElectronTicketService(),
    channelService: new ElectronChannelService(),
    loopService: new ElectronLoopService(),
    autopilotService: new ElectronAutopilotService(),
    meshService: new ElectronMeshService(),
    billingService: new ElectronBillingService(),
    extensionService: new ElectronExtensionService(),
    repositoryService: new ElectronRepositoryService(),
    invitationService: new ElectronInvitationService(),
    apiKeyService: new ElectronApiKeyService(),
    bindingService: new ElectronBindingService(),
    messageService: new ElectronMessageService(),
    notificationService: new ElectronNotificationService(),
    orgApiService: new ElectronOrgService(),
    userApiService: new ElectronUserService(),
    userCredentialService: new ElectronUserCredentialService(),
    agentService: new ElectronAgentService(),
    ssoService: new ElectronSSOService(),
    fileService: new ElectronFileService(),
    supportTicketService: new ElectronSupportTicketService(),
    ticketRelationsService: new ElectronTicketRelationsService(),
    tokenUsageService: new ElectronTokenUsageService(),
    promoCodeService: new ElectronPromoCodeService(),
    authApiService: new ElectronAuthApiService(),
    podState: new ElectronPodState(),
    runnerState: new ElectronRunnerState(),
    meshState: new ElectronMeshState(),
    ticketState: new ElectronTicketState(),
    channelState: new ElectronChannelState(),
    loopState: new ElectronLoopState(),
    autopilotState: new ElectronAutopilotState(),
    orgState: new ElectronOrgState(),
    userState: new ElectronUserState(),
    gitProviderState: new ElectronGitProviderState(),
    repoState: new ElectronRepoState(),
    acpManager: new ElectronAcpManager(),
    relayManager: new ElectronRelayManager(),
  };
}
