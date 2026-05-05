import { getPromoCodeService } from "@/lib/wasm-core";
export type { ValidatePromoCodeResponse, RedeemPromoCodeResponse, PromoCodeRedemption } from "./promoCodeTypes";

export const promoCodeApi = {
  validate: async (code: string) => {
    const json = await getPromoCodeService().validate(JSON.stringify({ code }));
    return JSON.parse(json);
  },
  redeem: async (code: string) => {
    await getPromoCodeService().redeem(JSON.stringify({ code }));
  },
};
