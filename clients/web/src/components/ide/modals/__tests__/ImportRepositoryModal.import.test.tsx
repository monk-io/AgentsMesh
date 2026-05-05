import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import { ImportRepositoryModal } from "../ImportRepositoryModal";
import {
  setupProviderMocks,
  mockRepositoryCreate,
  stableRepoSvc,
} from "./ImportRepositoryModal.utils";

describe("ImportRepositoryModal - Import Actions", () => {
  const mockOnClose = vi.fn();
  const mockOnImported = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    setupProviderMocks();
  });

  it("should call repositoryApi.create when import is clicked", async () => {
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
      expect(stableRepoSvc.create).toHaveBeenCalledWith(
        expect.stringContaining('"provider_type":"github"'),
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
