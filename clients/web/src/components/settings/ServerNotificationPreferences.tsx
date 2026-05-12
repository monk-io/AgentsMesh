"use client";

import React, { useEffect, useState, useCallback } from "react";
import { BellOff, Loader2 } from "lucide-react";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { useTranslations } from "next-intl";
import { useCurrentOrg } from "@/stores/auth";
import type { NotificationPreference } from "@/lib/api";
import { listPreferencesConnect, setPreferenceConnect } from "@/lib/api/notificationConnect";

// Available notification sources with i18n keys
const NOTIFICATION_SOURCES = [
  { source: "channel:message", labelKey: "settings.notifications.channelMessage", descKey: "settings.notifications.channelMessageDesc" },
  { source: "channel:mention", labelKey: "settings.notifications.channelMention", descKey: "settings.notifications.channelMentionDesc" },
  { source: "terminal:osc", labelKey: "settings.notifications.terminalOsc", descKey: "settings.notifications.terminalOscDesc" },
  { source: "task:completed", labelKey: "settings.notifications.taskCompleted", descKey: "settings.notifications.taskCompletedDesc" },
] as const;

// Channel label mapping for known delivery channels
const CHANNEL_LABELS: Record<string, string> = {
  toast: "Toast",
  browser: "Browser",
  apns: "Push (Mobile)",
  email: "Email",
};

/**
 * Server-synced notification preferences: mute / channels per source.
 *
 * Uses the Connect-RPC lane (proto.notification.v1) via the
 * `notificationConnect.ts` adapter. The legacy
 * `getNotificationService().get_preferences()` / `.set_preference()`
 * JSON path stays available during the dual-track window for non-migrated
 * callers (currently none — this is the only consumer).
 */
export function ServerNotificationPreferences() {
  const t = useTranslations();
  const currentOrg = useCurrentOrg();
  const [prefs, setPrefs] = useState<NotificationPreference[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchPrefs = useCallback(async () => {
    if (!currentOrg) {
      setLoading(false);
      return;
    }
    try {
      const list = await listPreferencesConnect(currentOrg.slug);
      setPrefs(list);
    } catch {
      // Silently fail - user might not have org context yet
    } finally {
      setLoading(false);
    }
  }, [currentOrg]);

  useEffect(() => { fetchPrefs(); }, [fetchPrefs]);

  const getPref = (source: string): NotificationPreference => {
    const found = prefs.find((p) => p.source === source && !p.entity_id);
    return found ?? { source, is_muted: false, channels: { toast: true, browser: true } };
  };

  const updatePref = (source: string, updated: NotificationPreference) => {
    setPrefs((prev) => {
      const idx = prev.findIndex((p) => p.source === source && !p.entity_id);
      if (idx >= 0) { const next = [...prev]; next[idx] = updated; return next; }
      return [...prev, updated];
    });
  };

  const handleMuteToggle = async (source: string, muted: boolean) => {
    if (!currentOrg) return;
    const updated = { ...getPref(source), is_muted: muted };
    updatePref(source, updated);
    try { await setPreferenceConnect(currentOrg.slug, updated); } catch { fetchPrefs(); }
  };

  const handleChannelToggle = async (source: string, channel: string, value: boolean) => {
    if (!currentOrg) return;
    const current = getPref(source);
    const updated = { ...current, channels: { ...current.channels, [channel]: value } };
    updatePref(source, updated);
    try { await setPreferenceConnect(currentOrg.slug, updated); } catch { fetchPrefs(); }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-4">
        <Loader2 className="w-4 h-4 animate-spin text-muted-foreground" />
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <h4 className="font-medium text-sm text-muted-foreground">
        {t("settings.notifications.deliveryPreferences")}
      </h4>
      <div className="space-y-3">
        {NOTIFICATION_SOURCES.map(({ source, labelKey, descKey }) => {
          const pref = getPref(source);
          return (
            <div key={source} className="p-3 rounded-lg border space-y-2">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label className="cursor-pointer font-medium">{t(labelKey)}</Label>
                  <p className="text-xs text-muted-foreground">{t(descKey)}</p>
                </div>
                <div className="flex items-center gap-1">
                  {pref.is_muted && <BellOff className="w-3.5 h-3.5 text-muted-foreground" />}
                  <Switch checked={!pref.is_muted} onCheckedChange={(checked) => handleMuteToggle(source, !checked)} />
                </div>
              </div>
              {!pref.is_muted && pref.channels && (
                <div className="flex items-center gap-4 pl-1">
                  {Object.entries(pref.channels).map(([ch, enabled]) => (
                    <label key={ch} className="flex items-center gap-1.5 text-xs text-muted-foreground cursor-pointer">
                      <Switch className="scale-75" checked={enabled as boolean} onCheckedChange={(v) => handleChannelToggle(source, ch, v)} />
                      {CHANNEL_LABELS[ch] ?? ch}
                    </label>
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
