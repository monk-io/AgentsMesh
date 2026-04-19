import { invoke } from "./invoke";
import type { IUserApiService } from "@agentsmesh/service-interface";

export class ElectronUserService implements IUserApiService {
  async get_me(): Promise<string> {
    return invoke<string>("userGetMe");
  }

  async get_organizations(): Promise<string> {
    return invoke<string>("userGetOrganizations");
  }
}
