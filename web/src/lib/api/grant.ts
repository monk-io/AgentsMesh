import { request, orgPath } from "./base";

export interface ResourceGrant {
  id: number;
  resource_type: string;
  resource_id: string;
  user_id: number;
  granted_by: number;
  created_at: string;
  user?: { id: number; email: string; username: string; name?: string };
  granted_by_user?: { id: number; email: string; username: string; name?: string };
}

export const grantApi = {
  list: (resourceType: string, resourceId: string) =>
    request<{ grants: ResourceGrant[] }>(
      orgPath(`/${resourceType}s/${resourceId}/grants`)
    ),

  grant: (resourceType: string, resourceId: string, userId: number) =>
    request<{ grant: ResourceGrant }>(
      orgPath(`/${resourceType}s/${resourceId}/grants`),
      { method: "POST", body: { user_id: userId } }
    ),

  revoke: (resourceType: string, resourceId: string, grantId: number) =>
    request<{ message: string }>(
      orgPath(`/${resourceType}s/${resourceId}/grants/${grantId}`),
      { method: "DELETE" }
    ),
};
