// AUTO-GENERATED — regenerate: pnpm --filter desktop e2e:gen
//
// Source of truth: clients/core/crates/node-bridge/index.d.ts (the
// napi-rs-emitted TypeScript declaration of AppState). Desktop main
// reflects over the prototype to register one ipcMain handler per method,
// so this mirror is what the renderer can actually invoke at runtime.
export interface IpcMethodSchema {
  name: string;
  group: string;
  params: Array<{ name: string; type: string }>;
  returnType: string;
}

export const ipcSchema: IpcMethodSchema[] = [
  {
    "name": "apikeyCreateConnect",
    "group": "apikey",
    "params": [],
    "returnType": "any",
    "ipcExposable": true
  },
  {
    "name": "apikeyDeleteConnect",
    "group": "apikey",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "apikeyGetConnect",
    "group": "apikey",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "apikeyListConnect",
    "group": "apikey",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "apikeyRevokeConnect",
    "group": "apikey",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "apikeyUpdateConnect",
    "group": "apikey",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authApplySessionProto",
    "group": "auth",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "authBootstrap",
    "group": "auth",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "authBootstrapProto",
    "group": "auth",
    "params": [],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authClearSession",
    "group": "auth",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "authFetchOrganizations",
    "group": "auth",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "authFetchOrganizationsProto",
    "group": "auth",
    "params": [],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authGetCurrentOrgJson",
    "group": "auth",
    "params": [],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "authGetCurrentUserJson",
    "group": "auth",
    "params": [],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "authGetCurrentUserProto",
    "group": "auth",
    "params": [],
    "returnType": "Array<number> | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "authGetExpiresAt",
    "group": "auth",
    "params": [],
    "returnType": "number | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "authGetOrganizationsJson",
    "group": "auth",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "authGetToken",
    "group": "auth",
    "params": [],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "authIsAuthenticated",
    "group": "auth",
    "params": [],
    "returnType": "boolean",
    "ipcExposable": true
  },
  {
    "name": "authLogin",
    "group": "auth",
    "params": [
      {
        "name": "email",
        "type": "string"
      },
      {
        "name": "password",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "authLoginProto",
    "group": "auth",
    "params": [
      {
        "name": "email",
        "type": "string"
      },
      {
        "name": "password",
        "type": "string"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authLogout",
    "group": "auth",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "authRefreshToken",
    "group": "auth",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "authRefreshTokenProto",
    "group": "auth",
    "params": [],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authSetCurrentOrgProto",
    "group": "auth",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "authSetOrganizationsProto",
    "group": "auth",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "authSwitchOrg",
    "group": "auth",
    "params": [
      {
        "name": "slug",
        "type": "string"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "authConnectForgotPasswordConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectLoginConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectLogoutConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectOauthCallbackConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectOauthRedirectConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectRefreshTokenConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectRegisterConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectResendVerificationConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectResetPasswordConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "authConnectVerifyEmailConnect",
    "group": "auth_connect",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "autopilotAppendIteration",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotControllersJson",
    "group": "autopilot",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "autopilotCurrentControllerJson",
    "group": "autopilot",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "autopilotGetControllerByPodKeyJson",
    "group": "autopilot",
    "params": [
      {
        "name": "podKey",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "autopilotGetIterationsJson",
    "group": "autopilot",
    "params": [
      {
        "name": "key",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "autopilotGetThinkingHistoryJson",
    "group": "autopilot",
    "params": [
      {
        "name": "key",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "autopilotGetThinkingJson",
    "group": "autopilot",
    "params": [
      {
        "name": "key",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "autopilotInsertController",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotPatchController",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotRemoveControllerProto",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotReplaceCachedControllers",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotReplaceCachedIterations",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotSetCurrentControllerProto",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "autopilotUpdateThinkingProto",
    "group": "autopilot",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "bindingAcceptBindingConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingApproveScopesConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingCheckBindingConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingGetBoundPodsConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingGetPendingBindingsConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingListBindingsConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingRejectBindingConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingRequestBindingConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingRequestScopesConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "bindingUnbindConnect",
    "group": "binding",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "blockstoreApplyRemoteOp",
    "group": "blockstore",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreBacklinksJson",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreBlocksJson",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreCatchup",
    "group": "blockstore",
    "params": [
      {
        "name": "workspaceId",
        "type": "string"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreEnsureDefaultWorkspace",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreGetBlockJson",
    "group": "blockstore",
    "params": [
      {
        "name": "id",
        "type": "string"
      }
    ],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "blockstoreLastOpId",
    "group": "blockstore",
    "params": [
      {
        "name": "workspaceId",
        "type": "string"
      }
    ],
    "returnType": "number",
    "ipcExposable": true
  },
  {
    "name": "blockstoreLastOpIdsJson",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreListBacklinksJson",
    "group": "blockstore",
    "params": [
      {
        "name": "targetId",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreListChildrenJson",
    "group": "blockstore",
    "params": [
      {
        "name": "parentId",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreListWorkspaces",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreLoadSubtree",
    "group": "blockstore",
    "params": [
      {
        "name": "workspaceId",
        "type": "string"
      },
      {
        "name": "rootId",
        "type": "string"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreLoadTypeDefs",
    "group": "blockstore",
    "params": [
      {
        "name": "workspaceId",
        "type": "string"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreNestChildrenJson",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreProjectLocalOps",
    "group": "blockstore",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreRefsJson",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreReplaceWorkspaces",
    "group": "blockstore",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreSetLastOpId",
    "group": "blockstore",
    "params": [
      {
        "name": "workspaceId",
        "type": "string"
      },
      {
        "name": "id",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreTypeDefsJson",
    "group": "blockstore",
    "params": [
      {
        "name": "workspaceId",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "blockstoreUpsertBlocks",
    "group": "blockstore",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreUpsertRefs",
    "group": "blockstore",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreUpsertWorkspace",
    "group": "blockstore",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "blockstoreWorkspacesJson",
    "group": "blockstore",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelApplyChannelMessageEditedEvent",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelApplyIncomingChannelMessage",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "boolean",
    "ipcExposable": true
  },
  {
    "name": "channelArchiveChannelConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelChannelMembersJson",
    "group": "channel",
    "params": [
      {
        "name": "id",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelChannelPodsJson",
    "group": "channel",
    "params": [
      {
        "name": "id",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelChannelsJson",
    "group": "channel",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelClearChannelMentions",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelClearChannelUnread",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelCreateChannelConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelCurrentChannelJson",
    "group": "channel",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelDeleteChannelMessageConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelEditChannelMessageConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelFilterChannelsJson",
    "group": "channel",
    "params": [
      {
        "name": "query",
        "type": "string"
      },
      {
        "name": "includeArchived",
        "type": "boolean"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelGetChannelConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelGetChannelJson",
    "group": "channel",
    "params": [
      {
        "name": "id",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelGetChannelUnreadCountsConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelGetLastMessageJson",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelGetMentionCount",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "number",
    "ipcExposable": true
  },
  {
    "name": "channelGetMessagesJson",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelGetUnreadCount",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "number",
    "ipcExposable": true
  },
  {
    "name": "channelIncrementMention",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelIncrementUnread",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelInsertChannel",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelInsertChannelMessage",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelListChannelMembersConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelListChannelMessagesConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelListChannelsConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelMarkChannelReadConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelMentionCountsJson",
    "group": "channel",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelPatchChannelMemberCount",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelPrependCachedChannelMessages",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelRemoveChannelMember",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelRemoveMessage",
    "group": "channel",
    "params": [
      {
        "name": "channelId",
        "type": "number"
      },
      {
        "name": "messageId",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelReplaceCachedChannelMessages",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelReplaceCachedChannels",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelReplaceChannelMembers",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelReplaceChannelPods",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelReplaceChannelUnreadCounts",
    "group": "channel",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelSelectChannel",
    "group": "channel",
    "params": [
      {
        "name": "id",
        "type": "number | undefined | null"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelSendChannelMessageConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelSetCurrentChannel",
    "group": "channel",
    "params": [
      {
        "name": "id",
        "type": "number | undefined | null"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelSetCurrentUserId",
    "group": "channel",
    "params": [
      {
        "name": "userId",
        "type": "number | undefined | null"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "channelSortedChannelIdsJson",
    "group": "channel",
    "params": [
      {
        "name": "mode",
        "type": "string"
      },
      {
        "name": "includeArchived",
        "type": "boolean"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelTotalMentionCount",
    "group": "channel",
    "params": [],
    "returnType": "number",
    "ipcExposable": true
  },
  {
    "name": "channelTotalUnreadCount",
    "group": "channel",
    "params": [],
    "returnType": "number",
    "ipcExposable": true
  },
  {
    "name": "channelUnarchiveChannelConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "channelUnreadCountsJson",
    "group": "channel",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "channelUpdateChannelConnect",
    "group": "channel",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "envBundleCreateEnvBundleConnect",
    "group": "env_bundle",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "envBundleDeleteEnvBundleConnect",
    "group": "env_bundle",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "envBundleGetEnvBundleConnect",
    "group": "env_bundle",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "envBundleListEnvBundlesConnect",
    "group": "env_bundle",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "envBundleSetPrimaryEnvBundleConnect",
    "group": "env_bundle",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "envBundleUpdateEnvBundleConnect",
    "group": "env_bundle",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "eventsConnect",
    "group": "events",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "eventsDisconnect",
    "group": "events",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "eventsGetConnectionState",
    "group": "events",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "eventsOnConnectionStateChange",
    "group": "events",
    "params": [
      {
        "name": "callback",
        "type": "(arg: string) => void"
      }
    ],
    "returnType": "number",
    "ipcExposable": false
  },
  {
    "name": "eventsSubscribeAll",
    "group": "events",
    "params": [
      {
        "name": "callback",
        "type": "(arg: string) => void"
      }
    ],
    "returnType": "number",
    "ipcExposable": false
  },
  {
    "name": "eventsUnsubscribe",
    "group": "events",
    "params": [
      {
        "name": "id",
        "type": "number"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "extensionCreateSkillRegistryConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionDeleteSkillRegistryConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionInstallCustomMcpServerConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionInstallMcpFromMarketConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionInstallSkillFromGithubConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionInstallSkillFromMarketConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionInstallSkillFromUploadedFileConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionListMarketMcpServersConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionListMarketSkillsConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionListRepoMcpServersConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionListRepoSkillsConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionListSkillRegistriesConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionListSkillRegistryOverridesConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionPresignSkillUploadConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionSyncSkillRegistryConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionTogglePlatformRegistryConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionUninstallMcpServerConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionUninstallSkillConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionUpdateMcpServerConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "extensionUpdateSkillConnect",
    "group": "extension",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "fileUploadFile",
    "group": "file",
    "params": [
      {
        "name": "fileData",
        "type": "Array<number>"
      },
      {
        "name": "filename",
        "type": "string"
      },
      {
        "name": "contentType",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "invitationAcceptInvitationConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "invitationCreateInvitationConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "invitationGetInvitationByTokenConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "invitationListInvitationsConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "invitationListPendingInvitationsConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "invitationResendInvitationConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "invitationRevokeInvitationConnect",
    "group": "invitation",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "localRunnerBinaryPath",
    "group": "local_runner",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "localRunnerFallbackVersion",
    "group": "local_runner",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "localRunnerHostTarget",
    "group": "local_runner",
    "params": [],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "localRunnerInstallBinary",
    "group": "local_runner",
    "params": [
      {
        "name": "releaseUrl",
        "type": "string"
      },
      {
        "name": "expectedSha256",
        "type": "string | undefined | null"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "localRunnerInstalledVersion",
    "group": "local_runner",
    "params": [],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "localRunnerIsInstalled",
    "group": "local_runner",
    "params": [],
    "returnType": "boolean",
    "ipcExposable": true
  },
  {
    "name": "localRunnerIsRegistered",
    "group": "local_runner",
    "params": [],
    "returnType": "boolean",
    "ipcExposable": true
  },
  {
    "name": "localRunnerLocalNodeId",
    "group": "local_runner",
    "params": [],
    "returnType": "string | undefined | null",
    "ipcExposable": true
  },
  {
    "name": "localRunnerRegister",
    "group": "local_runner",
    "params": [
      {
        "name": "token",
        "type": "string"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "localRunnerServiceInstall",
    "group": "local_runner",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "localRunnerServiceStart",
    "group": "local_runner",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "localRunnerServiceStatus",
    "group": "local_runner",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "localRunnerServiceStop",
    "group": "local_runner",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "localRunnerServiceUninstall",
    "group": "local_runner",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcAppendCachedRuns",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcClearCurrentLoop",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcClearLoopRuns",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcCurrentLoopJson",
    "group": "loop_svc",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "loopSvcGetLoopBySlugJson",
    "group": "loop_svc",
    "params": [
      {
        "name": "slug",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "loopSvcInsertLoopRun",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcLoopsJson",
    "group": "loop_svc",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "loopSvcPatchLoopFromAction",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcPatchLoopRunStatus",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcReplaceCachedLoops",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcReplaceCachedRuns",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "loopSvcRunsJson",
    "group": "loop_svc",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "loopSvcSetCurrentLoop",
    "group": "loop_svc",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "meshBatchGetTicketPodsConnect",
    "group": "mesh",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "meshClearTopology",
    "group": "mesh",
    "params": [],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "meshCreatePodForTicketConnect",
    "group": "mesh",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "meshFetchTopology",
    "group": "mesh",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetActiveNodesJson",
    "group": "mesh",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetChannelsForNodeJson",
    "group": "mesh",
    "params": [
      {
        "name": "podKey",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetEdgesForNodeJson",
    "group": "mesh",
    "params": [
      {
        "name": "podKey",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetMeshTopologyConnect",
    "group": "mesh",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "meshGetNodeJson",
    "group": "mesh",
    "params": [
      {
        "name": "podKey",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetNodesByRunnerJson",
    "group": "mesh",
    "params": [
      {
        "name": "runnerId",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetRunnerInfoJson",
    "group": "mesh",
    "params": [
      {
        "name": "runnerId",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshGetTicketPodsConnect",
    "group": "mesh",
    "params": [
      {
        "name": "request",
        "type": "Uint8Array"
      }
    ],
    "returnType": "Uint8Array",
    "ipcExposable": true
  },
  {
    "name": "meshReplaceTopology",
    "group": "mesh",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "meshSelectedNode",
    "group": "mesh",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "meshSelectNode",
    "group": "mesh",
    "params": [
      {
        "name": "podKey",
        "type": "string | undefined | null"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "meshTopologyJson",
    "group": "mesh",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "podCurrentPodJson",
    "group": "pod",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "podGetPodJson",
    "group": "pod",
    "params": [
      {
        "name": "podKey",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "podPodsJson",
    "group": "pod",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "promocodeGetRedemptionHistoryConnect",
    "group": "promocode",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "promocodeRedeemPromoCodeConnect",
    "group": "promocode",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "promocodeValidatePromoCodeConnect",
    "group": "promocode",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "runnerAuthorizeRunner",
    "group": "runner",
    "params": [
      {
        "name": "requestBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "runnerAvailableRunnersJson",
    "group": "runner",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "runnerCurrentRunnerJson",
    "group": "runner",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "runnerGetAuthStatus",
    "group": "runner",
    "params": [
      {
        "name": "requestBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "runnerGetRunnerJson",
    "group": "runner",
    "params": [
      {
        "name": "id",
        "type": "number"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "runnerPatchCachedRunner",
    "group": "runner",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "runnerRemoveCachedRunner",
    "group": "runner",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "runnerReplaceAvailableRunners",
    "group": "runner",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "runnerReplaceCachedRunners",
    "group": "runner",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "runnerRunnersJson",
    "group": "runner",
    "params": [],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "runnerSetCurrentRunnerProto",
    "group": "runner",
    "params": [
      {
        "name": "reqBytes",
        "type": "Array<number>"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "runnerUpdateRunnerStatus",
    "group": "runner",
    "params": [
      {
        "name": "id",
        "type": "number"
      },
      {
        "name": "status",
        "type": "string"
      }
    ],
    "returnType": "void",
    "ipcExposable": true
  },
  {
    "name": "ssoDiscoverConnect",
    "group": "sso",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ssoLdapAuthConnect",
    "group": "sso",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketAddSupportTicketMessageConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketAssociateAttachmentsConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketCreateSupportTicketConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketGetAttachmentUrlConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketGetSupportTicketConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketListSupportTicketsConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "supportTicketPresignAttachmentUploadConnect",
    "group": "support_ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketAddAssigneeConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketAddLabelConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketCreateLabelConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketCreateTicketConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketDeleteLabelConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketDeleteTicketConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketGetActiveTicketsConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketGetBoardConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketGetSubTicketsConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketGetTicketConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketGetTicketPods",
    "group": "ticket",
    "params": [
      {
        "name": "slug",
        "type": "string"
      },
      {
        "name": "activeOnly",
        "type": "boolean | undefined | null"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "ticketListLabelsConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketListTicketsConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRemoveAssigneeConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRemoveLabelConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketTicketPodsJson",
    "group": "ticket",
    "params": [
      {
        "name": "slug",
        "type": "string"
      }
    ],
    "returnType": "string",
    "ipcExposable": true
  },
  {
    "name": "ticketUpdateLabelConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketUpdateTicketConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketUpdateTicketStatusConnect",
    "group": "ticket",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsCreateCommentConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsCreateRelationConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsDeleteCommentConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsDeleteRelationConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsLinkCommitConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsListCommentsConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsListCommitsConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsListMergeRequestsConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsListRelationsConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsUnlinkCommitConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "ticketRelationsUpdateCommentConnect",
    "group": "ticket_relations",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "userChangePasswordConnect",
    "group": "user",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "userDeleteIdentityConnect",
    "group": "user",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "userGetMeConnect",
    "group": "user",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "userListIdentitiesConnect",
    "group": "user",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "userSearchUsersConnect",
    "group": "user",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  },
  {
    "name": "userUpdateMeConnect",
    "group": "user",
    "params": [
      {
        "name": "request",
        "type": "Array<number>"
      }
    ],
    "returnType": "Array<number>",
    "ipcExposable": true
  }
];
