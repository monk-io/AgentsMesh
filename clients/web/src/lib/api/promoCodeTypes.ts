export type PromoCodeType = "media" | "partner" | "campaign" | "internal" | "referral";

export interface ValidatePromoCodeResponse {
  valid: boolean;
  code: string;
  plan_name?: string;
  plan_display_name?: string;
  duration_months?: number;
  expires_at?: string;
  message_code?: string;
}

export interface RedeemPromoCodeResponse {
  success: boolean;
  plan_name?: string;
  duration_months?: number;
  new_period_end?: string;
  message_code?: string;
}

export interface PromoCodeRedemption {
  id: number;
  promo_code_id: number;
  organization_id: number;
  user_id: number;
  plan_name: string;
  duration_months: number;
  previous_plan_name?: string;
  previous_period_end?: string;
  new_period_end: string;
  created_at: string;
}
