import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import {
  setupProviderMocks,
  mockRepositoryCreate,
  stableRepoSvc,
  lastCreateRepoCall,
} from "./ImportRepositoryModal.utils";

const stable = vi.hoisted(() => ({
  org: { id: 1, name: "TestOrg", slug: "test-org" },
  user: { id: 1, email: "u@e.com", username: "u" },
}));

vi.mock("@/stores/auth", () => ({
  useCurrentOrg: () => stable.org,
  useCurrentUser: () => stable.user,
  useAuthOrganizations: () => [],
  useAuthStore: () => ({ currentOrg: stable.org }),
  useIsAuthenticated: () => true,
  readCurrentUser: () => stable.user,
  readCurrentOrg: () => stable.org,
  readOrganizations: () => [],
}));

import { ImportRepositoryModal } from "../ImportRepositoryModal";

describe("ImportRepositoryModal - Import Actions", () => {
  const mockOnClose = vi.fn();
  const mockOnImported = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    setupProviderMocks();
    mockRepositoryCreate();
  });

  it("should call createRepository (Connect) when import is clicked", async () => {
    mockRepositoryCreate();

    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("My GitHub"));

    await waitFor(() => {
      expect(screen.getByText("org/my-project")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("org/my-project"));

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Import Repository" })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Repository" }));

    // Production calls createRepositoryConnect with proto-encoded bytes
    // over the wasm bridge. Assert on the decoded request body.
    await waitFor(() => {
      expect(stableRepoSvc.createRepositoryConnect).toHaveBeenCalled();
      expect(lastCreateRepoCall()).toEqual(
        expect.objectContaining({ provider_type: "github" }),
      );
    });
  });

  it("should call onImported and onClose after successful import", async () => {
    mockRepositoryCreate();

    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("My GitHub"));

    await waitFor(() => {
      expect(screen.getByText("org/my-project")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("org/my-project"));

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Import Repository" })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Repository" }));

    await waitFor(() => {
      expect(mockOnImported).toHaveBeenCalled();
      expect(mockOnClose).toHaveBeenCalled();
    });
  });

  it("should handle import error", async () => {
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    stableRepoSvc.create.mockRejectedValue(new Error("Import failed"));

    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("My GitHub"));

    await waitFor(() => {
      expect(screen.getByText("org/my-project")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("org/my-project"));

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Import Repository" })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Repository" }));

    await waitFor(() => {
      expect(screen.getByText(/Failed to import repository/)).toBeInTheDocument();
    });

    consoleSpy.mockRestore();
  });
});
