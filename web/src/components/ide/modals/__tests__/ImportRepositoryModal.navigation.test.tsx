import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import { ImportRepositoryModal } from "../ImportRepositoryModal";
import {
  mockCreatedRepository,
  setupProviderMocks,
  mockRepositoryCreate,
  createRepositoryResponse,
  stableRepoSvc,
} from "./ImportRepositoryModal.utils";

describe("ImportRepositoryModal - Navigation Flow", () => {
  const mockOnClose = vi.fn();
  const mockOnImported = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    setupProviderMocks();
  });

  it("should complete manual import flow successfully", async () => {
    mockRepositoryCreate(createRepositoryResponse({
      ...mockCreatedRepository,
      name: "test-repo",
      slug: "test/repo",
      http_clone_url: "https://github.com/test/repo.git",
    }));

    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("Enter Manually")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("Enter Manually"));

    await waitFor(() => {
      expect(screen.getByPlaceholderText("https://github.com/org/repo.git")).toBeInTheDocument();
    });

    fireEvent.change(screen.getByPlaceholderText("https://github.com/org/repo.git"), {
      target: { value: "https://github.com/test/repo.git" },
    });
    fireEvent.change(screen.getByPlaceholderText("my-project"), {
      target: { value: "test-repo" },
    });
    fireEvent.change(screen.getByPlaceholderText("org/my-project"), {
      target: { value: "test/repo" },
    });

    fireEvent.click(screen.getByText("Continue"));

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Import Repository" })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Repository" }));

    await waitFor(() => {
      expect(stableRepoSvc.create).toHaveBeenCalledWith(
        expect.stringContaining('"provider_type":"github"'),
      );
      expect(mockOnImported).toHaveBeenCalled();
      expect(mockOnClose).toHaveBeenCalled();
    });
  });

  it("should allow changing visibility in confirm step", async () => {
    mockRepositoryCreate(createRepositoryResponse({
      ...mockCreatedRepository,
      visibility: "private",
    }));

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
      expect(screen.getByText("Private (only you)")).toBeInTheDocument();
    });

    const privateRadio = screen.getByText("Private (only you)").previousElementSibling;
    if (privateRadio) {
      fireEvent.click(privateRadio);
    }

    fireEvent.click(screen.getByRole("button", { name: "Import Repository" }));

    await waitFor(() => {
      expect(stableRepoSvc.create).toHaveBeenCalledWith(
        expect.stringContaining('"visibility":"private"'),
      );
    });
  });
});
