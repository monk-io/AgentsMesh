import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import RepositoryDetailPage from "../page";
import { getRepositoryService, getApiClient } from "@/lib/wasm-core";

// Mock next/navigation
const mockPush = vi.fn();
vi.mock("next/navigation", () => ({
  useParams: () => ({ id: "1" }),
  useRouter: () => ({ push: mockPush }),
}));

// Mock next/link
vi.mock("next/link", () => ({
  default: ({ children, href }: { children: React.ReactNode; href: string }) => (
    <a href={href}>{children}</a>
  ),
}));

// No longer mocking @/lib/api/repository — WebhookSettingsCard now uses WASM services

const mockRepoService = vi.mocked(getRepositoryService);
const mockApiClient = vi.mocked(getApiClient);

describe("RepositoryDetailPage - Webhook, Edit & Variants", () => {
  const mockRepository = {
    id: 1,
    organization_id: 1,
    provider_type: "github",
    provider_base_url: "https://github.com",
    http_clone_url: "https://github.com/org/my-repo.git",
    external_id: "12345",
    name: "my-repo",
    slug: "org/my-repo",
    default_branch: "main",
    ticket_prefix: "PROJ",
    visibility: "organization",
    is_active: true,
    created_at: "2024-01-01T00:00:00Z",
    updated_at: "2024-01-01T00:00:00Z",
  };

  let mockGet: ReturnType<typeof vi.fn>;
  let mockDelete: ReturnType<typeof vi.fn>;
  let mockUpdate: ReturnType<typeof vi.fn>;
  let mockGetWebhookStatus: ReturnType<typeof vi.fn>;
  let mockRegisterWebhook: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    vi.clearAllMocks();

    mockGet = vi.fn().mockResolvedValue(JSON.stringify({ repository: mockRepository }));
    mockDelete = vi.fn().mockResolvedValue(undefined);
    mockUpdate = vi.fn().mockResolvedValue(JSON.stringify({ repository: mockRepository }));
    mockGetWebhookStatus = vi.fn().mockResolvedValue(JSON.stringify({
      webhook_status: { registered: false, is_active: false, needs_manual_setup: false },
    }));
    mockRegisterWebhook = vi.fn().mockResolvedValue(undefined);

    mockRepoService.mockReturnValue({
      ...getRepositoryService(),
      get: mockGet,
      delete: mockDelete,
      update: mockUpdate,
      get_webhook_status: mockGetWebhookStatus,
      get_webhook_secret: vi.fn().mockResolvedValue(JSON.stringify({
        webhook_url: "https://example.com/webhook", webhook_secret: "secret", events: [],
      })),
      delete_webhook: vi.fn().mockResolvedValue(undefined),
      register_webhook: mockRegisterWebhook,
      mark_webhook_configured: vi.fn().mockResolvedValue(undefined),
    } as unknown as ReturnType<typeof getRepositoryService>);

    mockApiClient.mockReturnValue({
      ...getApiClient(),
      post: vi.fn().mockResolvedValue('{}'),
      org_path: (p: string) => `/api/v1/orgs/test-org${p}`,
    } as unknown as ReturnType<typeof getApiClient>);
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("webhook setup", () => {
    it("should call registerWebhook API when button clicked", async () => {
      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("Register Webhook")).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText("Register Webhook"));

      await waitFor(() => {
        expect(mockRegisterWebhook).toHaveBeenCalled();
      });
    });

    it("should refresh webhook status after successful registration", async () => {
      mockGetWebhookStatus.mockResolvedValueOnce(JSON.stringify({
        webhook_status: {
          registered: false,
          is_active: false,
          needs_manual_setup: false,
        },
      }));

      mockGetWebhookStatus.mockResolvedValueOnce(JSON.stringify({
        webhook_status: {
          registered: true,
          is_active: true,
          needs_manual_setup: false,
          webhook_id: "wh_123",
        },
      }));

      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("Register Webhook")).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText("Register Webhook"));

      await waitFor(() => {
        expect(mockGetWebhookStatus).toHaveBeenCalledTimes(2);
      });
    });
  });

  describe("edit modal", () => {
    it("should open edit modal when Edit clicked", async () => {
      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("Edit")).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText("Edit"));

      expect(screen.getByText("Edit Repository")).toBeInTheDocument();
    });

    it("should close edit modal when Cancel clicked", async () => {
      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("Edit")).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText("Edit"));
      fireEvent.click(screen.getByText("Cancel"));

      await waitFor(() => {
        expect(screen.queryByText("Edit Repository")).not.toBeInTheDocument();
      });
    });

    it("should call update API when save clicked", async () => {
      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("Edit")).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText("Edit"));

      const nameInput = screen.getByDisplayValue("my-repo");
      fireEvent.change(nameInput, { target: { value: "updated-repo" } });

      fireEvent.click(screen.getByText("Save Changes"));

      await waitFor(() => {
        expect(mockUpdate).toHaveBeenCalledWith(
          BigInt(1),
          expect.stringContaining('"name":"updated-repo"'),
        );
      });
    });
  });

  describe("inactive repository", () => {
    it("should show Inactive badge for inactive repository", async () => {
      mockGet.mockResolvedValue(
        JSON.stringify({ repository: { ...mockRepository, is_active: false } }),
      );

      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getAllByText("Inactive").length).toBeGreaterThanOrEqual(1);
      });
    });
  });

  describe("private visibility repository", () => {
    it("should show Private badge for private visibility repository", async () => {
      mockGet.mockResolvedValue(
        JSON.stringify({ repository: { ...mockRepository, visibility: "private" } }),
      );

      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("Private")).toBeInTheDocument();
      });
    });
  });

  describe("different providers", () => {
    it("should show GitLab provider type", async () => {
      mockGet.mockResolvedValue(
        JSON.stringify({
          repository: {
            ...mockRepository,
            provider_type: "gitlab",
            provider_base_url: "https://gitlab.com",
          },
        }),
      );

      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("gitlab")).toBeInTheDocument();
        expect(screen.getByText("https://gitlab.com")).toBeInTheDocument();
      });
    });

    it("should show Gitee provider type", async () => {
      mockGet.mockResolvedValue(
        JSON.stringify({
          repository: {
            ...mockRepository,
            provider_type: "gitee",
            provider_base_url: "https://gitee.com",
          },
        }),
      );

      render(<RepositoryDetailPage />);

      await waitFor(() => {
        expect(screen.getByText("gitee")).toBeInTheDocument();
        expect(screen.getByText("https://gitee.com")).toBeInTheDocument();
      });
    });
  });
});
