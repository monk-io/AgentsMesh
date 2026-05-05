import { invoke } from "./invoke";
import type { IAuthApiService } from "@agentsmesh/service-interface";

export class ElectronAuthApiService implements IAuthApiService {
  async register(json: string): Promise<string> {
    return invoke<string>("authApiRegister", json);
  }

  async verify_email(token: string): Promise<string> {
    return invoke<string>("authApiVerifyEmail", token);
  }

  async resend_verification(email: string): Promise<string> {
    return invoke<string>("authApiResendVerification", email);
  }

  async forgot_password(email: string): Promise<string> {
    return invoke<string>("authApiForgotPassword", email);
  }

  async reset_password(json: string): Promise<string> {
    return invoke<string>("authApiResetPassword", json);
  }
}
