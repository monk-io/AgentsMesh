import { useEffect, useState, type ReactNode } from "react";
import { IntlProvider as ReactIntlProvider } from "next-intl";
import { type Locale, defaultLocale, isValidLocale, MESSAGE_NAMESPACES } from "@/lib/i18n/config";

export const LOCALE_STORAGE_KEY = "app_locale";
export const LOCALE_CHANGE_EVENT = "app-locale-change";

function detectSystemLocale(): Locale {
  const lang = navigator.language.split("-")[0];
  return isValidLocale(lang) ? lang : defaultLocale;
}

function getSavedLocale(): Locale {
  const saved = localStorage.getItem(LOCALE_STORAGE_KEY);
  if (saved && isValidLocale(saved)) return saved as Locale;
  return detectSystemLocale();
}

async function loadMessages(locale: Locale): Promise<Record<string, unknown>> {
  const files = await Promise.all(
    MESSAGE_NAMESPACES.map((m) => import(`@/messages/${locale}/${m}.json`).catch(() => ({ default: {} })))
  );
  return Object.assign({}, ...files.map((f) => f.default));
}

export function DesktopIntlProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>(getSavedLocale);
  const [messages, setMessages] = useState<Record<string, unknown> | null>(null);

  useEffect(() => {
    loadMessages(locale).then(setMessages);
  }, [locale]);

  useEffect(() => {
    const handler = (e: Event) => {
      const next = (e as CustomEvent<Locale>).detail;
      if (isValidLocale(next)) setLocaleState(next);
    };
    window.addEventListener(LOCALE_CHANGE_EVENT, handler);
    return () => window.removeEventListener(LOCALE_CHANGE_EVENT, handler);
  }, []);

  if (!messages) return null;

  return (
    <ReactIntlProvider locale={locale} messages={messages}>
      {children}
    </ReactIntlProvider>
  );
}
