import { invoke } from "./invoke";
import type { IPromoCodeService } from "@agentsmesh/service-interface";

export class ElectronPromoCodeService implements IPromoCodeService {
  async validate(json: string): Promise<string> {
    return invoke<string>("promocodeValidate", json);
  }

  async redeem(json: string): Promise<void> {
    await invoke<void>("promocodeRedeem", json);
  }

  async get_history(): Promise<string> {
    return invoke<string>("promocodeGetHistory");
  }
}
