import {
  useNavigate,
  useLocation,
  useSearchParams as useRRSearchParams,
  useParams as useRRParams,
} from "react-router-dom";
import { useMemo } from "react";

interface NavigationOptions {
  scroll?: boolean;
}

export function useRouter() {
  const navigate = useNavigate();

  return useMemo(
    () => ({
      push: (url: string, _options?: NavigationOptions) => navigate(url),
      replace: (url: string, _options?: NavigationOptions) => navigate(url, { replace: true }),
      back: () => navigate(-1),
      forward: () => navigate(1),
      refresh: () => navigate(0),
      prefetch: (_url: string) => {},
    }),
    [navigate],
  );
}

export function usePathname(): string {
  const location = useLocation();
  return location.pathname;
}

export function useSearchParams(): URLSearchParams {
  const [searchParams] = useRRSearchParams();
  return searchParams;
}

export function useParams<T extends Record<string, string> = Record<string, string>>(): T {
  return useRRParams() as T;
}

export { useLocation } from "react-router-dom";

export function redirect(url: string): never {
  window.location.href = url;
  throw new Error("redirect");
}

export function useSelectedLayoutSegment(): string | null {
  const location = useLocation();
  const segments = location.pathname.split("/").filter(Boolean);
  return segments[0] ?? null;
}
