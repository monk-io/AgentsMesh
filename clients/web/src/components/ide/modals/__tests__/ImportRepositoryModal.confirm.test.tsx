import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import { ImportRepositoryModal } from "../ImportRepositoryModal";
import {
  setupProviderMocks,
  mockRepositoryCreate,
  stableRepoSvc,
} from "./ImportRepositoryModal.utils";

describe("ImportRepositoryModal - Confirmation Step", () => {
  const mockOnClose = vi.fn();
  const mockOnImported = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    setupProviderMocks();
  });

  it("should navigate to confirm step after selecting repository", async () => {
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
      expect(screen.getByText("Confirm Import")).toBeInTheDocument();
    });
  });

  it("should show repository details in confirm step", async () => {
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
      expect(screen.getByText("my-project")).toBeInTheDocument();
      expect(screen.getByText(/Ticket Prefix/)).toBeInTheDocument();
    });
  });

  it("should show visibility options in confirm step", async () => {
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
      expect(screen.getByText("Visibility")).toBeInTheDocument();
      expect(screen.getByText("Organization")).toBeInTheDocument();
      expect(screen.getByText("Private (only you)")).toBeInTheDocument();
    });
  });

  it("should allow setting ticket prefix", async () => {
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
      expect(screen.getByPlaceholderText("PROJ")).toBeInTheDocument();
    });

    fireEvent.change(screen.getByPlaceholderText("PROJ"), {
      target: { value: "TEST" },
    });

    fireEvent.click(screen.getByRole("button", { name: "Import Repository" }));

    await waitFor(() => {
      expect(stableRepoSvc.create).toHaveBeenCalledWith(
        expect.stringContaining('"ticket_prefix":"TEST"'),
      );
    });
  });
});
