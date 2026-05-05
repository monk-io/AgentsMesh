import { getAuthApiService } from "@/lib/wasm-core";

export const authApi = {
  register: async (data: { name: string; email: string; password: string }) => {
    const json = await getAuthApiService().register(JSON.stringify(data));
    return JSON.parse(json);
  },
  verifyEmail: async (token: string) => {
    const json = await getAuthApiService().verify_email(token);
    return JSON.parse(json);
  },
  resendVerification: async (email: string) => {
    const json = await getAuthApiService().resend_verification(email);
    return JSON.parse(json);
  },
  forgotPassword: async (email: string) => {
    const json = await getAuthApiService().forgot_password(email);
    return JSON.parse(json);
  },
  resetPassword: async (token: string, password: string) => {
    const json = await getAuthApiService().reset_password(JSON.stringify({ token, password }));
    return JSON.parse(json);
  },
};
