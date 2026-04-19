import { invoke } from "./invoke";
import type { ISSOService } from "@agentsmesh/service-interface";

export class ElectronSSOService implements ISSOService {
  async discover(email: string): Promise<string> {
    return invoke<string>("ssoDiscover", email);
  }

  async ldap_auth(domain: string, json: string): Promise<string> {
    return invoke<string>("ssoLdapAuth", domain, json);
  }
}
