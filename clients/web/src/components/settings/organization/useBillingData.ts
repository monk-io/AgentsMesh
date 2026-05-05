"use client";

import { useState, useEffect, useCallback } from "react";
import { useSearchParams } from "next/navigation";
import type { BillingOverview, SubscriptionPlan, DeploymentInfo } from "@/lib/api/billing-types";
import { getBillingService } from "@/lib/wasm-core";
import { getLocalizedErrorMessage } from "@/lib/api/errors";
import type { TranslationFn } from "./GeneralSettings";

export function useBillingData(t: TranslationFn) {
  const searchParams = useSearchParams();
  const [loading, setLoading] = useState(true);
  const [overview, setOverview] = useState<BillingOverview | null>(null);
  const [plans, setPlans] = useState<SubscriptionPlan[]>([]);
  const [deploymentInfo, setDeploymentInfo] = useState<DeploymentInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [upgrading, setUpgrading] = useState(false);
  const [reactivating, setReactivating] = useState(false);
  const [paymentMessage, setPaymentMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  useEffect(() => {
    const payment = searchParams.get("payment");
    if (payment === "success") {
      setPaymentMessage({ type: "success", text: t("settings.billingPage.paymentSuccess") });
      window.history.replaceState({}, "", window.location.pathname);
    } else if (payment === "cancelled") {
      setPaymentMessage({ type: "error", text: t("settings.billingPage.paymentCancelled") });
      window.history.replaceState({}, "", window.location.pathname);
    }
  }, [searchParams, t]);

  const loadBillingData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const svc = getBillingService();
      const [overviewJson, plansJson, deploymentJson] = await Promise.all([
        svc.get_overview().catch(() => null),
        svc.list_plans().catch(() => '{"plans":[]}'),
        svc.get_deployment_info().catch(() => null),
      ]);
      if (overviewJson) {
        const overviewRes = JSON.parse(overviewJson);
        if (overviewRes?.overview) setOverview(overviewRes.overview);
      }
      const plansRes = JSON.parse(plansJson as string);
      setPlans(plansRes.plans || []);
      if (deploymentJson) setDeploymentInfo(JSON.parse(deploymentJson));
    } catch (err) {
      setError(getLocalizedErrorMessage(err, t, t("settings.billingPage.loadFailed") || "Failed to load billing data"));
    } finally { setLoading(false); }
  }, [t]);

  useEffect(() => { loadBillingData(); }, [loadBillingData]);

  const handleFreePlanSelect = async (planName: string) => {
    setUpgrading(true); setError(null);
    try {
      const svc = getBillingService();
      if (overview) await svc.update_subscription(JSON.stringify({ plan_name: planName }));
      else await svc.create_subscription(JSON.stringify({ plan_name: planName, billing_cycle: "monthly" }));
      await loadBillingData();
    } catch (err) {
      setError(getLocalizedErrorMessage(err, t, t("settings.billingPage.selectPlanFailed") || "Failed to select plan"));
    } finally { setUpgrading(false); }
  };

  const handleSelectPlan = async (planName: string, callbacks: {
    onShowCheckout: (plan: SubscriptionPlan) => void;
    onCloseDialog: () => void;
  }) => {
    const plan = plans.find((p) => p.name === planName);
    if (!plan) return;
    callbacks.onCloseDialog();
    if (plan.price_per_seat_monthly === 0) { handleFreePlanSelect(planName); return; }
    if (overview) {
      const currentPrice = overview.plan?.price_per_seat_monthly || 0;
      if (plan.price_per_seat_monthly > currentPrice) {
        setUpgrading(true); setError(null);
        try {
          await getBillingService().upgrade(JSON.stringify({ plan_name: planName }));
          setPaymentMessage({ type: "success", text: t("settings.billingPage.upgradeSuccess") || "Plan upgraded successfully" });
          await loadBillingData();
        } catch (err) {
          setError(getLocalizedErrorMessage(err, t, t("settings.billingPage.upgradeFailed") || "Failed to upgrade plan"));
        } finally { setUpgrading(false); }
        return;
      }
    }
    callbacks.onShowCheckout(plan);
  };

  const handleReactivateSubscription = async () => {
    setReactivating(true);
    try {
      await getBillingService().reactivate();
      await loadBillingData();
      setPaymentMessage({ type: "success", text: t("settings.billingPage.reactivateSuccess") });
    } catch (err) {
      setError(getLocalizedErrorMessage(err, t, t("settings.billingPage.reactivateFailed") || "Failed to reactivate subscription"));
    } finally { setReactivating(false); }
  };

  return {
    loading, overview, plans, deploymentInfo, error, setError,
    upgrading, reactivating, paymentMessage, setPaymentMessage,
    loadBillingData, handleSelectPlan, handleReactivateSubscription,
  };
}
