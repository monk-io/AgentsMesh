import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

vi.mock("next-intl", () => ({
  useTranslations: () => (key: string) => {
    const translations: Record<string, string> = {
      "title": "Webhook Settings",
      "loading": "Loading...",
      "status.notRegistered": "Not Registered",
      "notRegisteredDescription": "No webhook registered. Register a webhook to receive PR/Pipeline status updates.",
      "register": "Register Webhook",
      "error.register": "Failed to register webhook",
    };
    return translations[key] || key;
  },
}));

import {
  mockGetWebhookStatus,
  mockRegisterWebhook,
} from "./testSetup";

import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { WebhookSettings } from "../../webhook";
import {
  mockRepository,
  notRegisteredStatus,
  registeredStatus,
  manualSetupStatus,
  resetAllMocks,
} from "./testSetup";

describe("WebhookSettings - Not Registered State", () => {
  const mockOnUpdate = vi.fn();

  beforeEach(() => {
    resetAllMocks();
    mockGetWebhookStatus.mockResolvedValue(JSON.stringify({ webhook_status: notRegisteredStatus }));
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it("should display not registered status", async () => {
    render(<WebhookSettings repository={mockRepository} onUpdate={mockOnUpdate} />);

    await waitFor(() => {
      expect(screen.getByText("Not Registered")).toBeInTheDocument();
    });
  });

  it("should display description text", async () => {
    render(<WebhookSettings repository={mockRepository} onUpdate={mockOnUpdate} />);

    await waitFor(() => {
      expect(screen.getByText("No webhook registered. Register a webhook to receive PR/Pipeline status updates.")).toBeInTheDocument();
    });
  });

  it("should show register button", async () => {
    render(<WebhookSettings repository={mockRepository} onUpdate={mockOnUpdate} />);

    await waitFor(() => {
      expect(screen.getByText("Register Webhook")).toBeInTheDocument();
    });
  });

  it("should handle register click - successful auto registration", async () => {
    mockRegisterWebhook.mockResolvedValue(JSON.stringify({
      result: { repo_id: 1, registered: true, webhook_id: "wh_new" },
    }));
    mockGetWebhookStatus
      .mockResolvedValueOnce(JSON.stringify({ webhook_status: notRegisteredStatus }))
      .mockResolvedValue(JSON.stringify({ webhook_status: registeredStatus }));

    render(<WebhookSettings repository={mockRepository} onUpdate={mockOnUpdate} />);

    await waitFor(() => {
      expect(screen.getByText("Register Webhook")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("Register Webhook"));

    await waitFor(() => {
      expect(mockRegisterWebhook).toHaveBeenCalled();
    });

    expect(mockOnUpdate).toHaveBeenCalled();
  });

  it("should handle register click - needs manual setup", async () => {
    mockRegisterWebhook.mockResolvedValue(JSON.stringify({
      result: {
        repo_id: 1,
        registered: false,
        needs_manual_setup: true,
        manual_webhook_url: "https://example.com/webhooks/org/gitlab/1",
        manual_webhook_secret: "new_secret",
        error: "OAuth token not available",
      },
    }));

    mockGetWebhookStatus
      .mockResolvedValueOnce(JSON.stringify({ webhook_status: notRegisteredStatus }))
      .mockResolvedValue(JSON.stringify({ webhook_status: manualSetupStatus }));

    render(<WebhookSettings repository={mockRepository} onUpdate={mockOnUpdate} />);

    await waitFor(() => {
      expect(screen.getByText("Register Webhook")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("Register Webhook"));

    await waitFor(() => {
      expect(mockRegisterWebhook).toHaveBeenCalled();
    });
  });

  it("should show error when register fails", async () => {
    mockRegisterWebhook.mockRejectedValue(new Error("Registration failed"));

    render(<WebhookSettings repository={mockRepository} onUpdate={mockOnUpdate} />);

    await waitFor(() => {
      expect(screen.getByText("Register Webhook")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("Register Webhook"));

    await waitFor(() => {
      expect(screen.getByText("Failed to register webhook")).toBeInTheDocument();
    });
  });
});
