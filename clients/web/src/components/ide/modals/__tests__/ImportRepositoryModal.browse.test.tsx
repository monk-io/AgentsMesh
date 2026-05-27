import { describe, it, expect, vi, beforeEach } from "vitest";
import { fromBinary } from "@bufbuild/protobuf";
import { ListProviderRepositoriesRequestSchema } from "@proto/user_credential/v1/user_credential_pb";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import { ImportRepositoryModal } from "../ImportRepositoryModal";
import {
  setupProviderMocks,
  stableCredSvc,
} from "./ImportRepositoryModal.utils";

// Decode the last `listProviderRepositoriesConnect` call. Production
// passes proto-encoded Uint8Array, so the test inspects field-level
// values via fromBinary rather than positional args.
function lastListProviderReposCall() {
  const calls = stableCredSvc.listProviderRepositoriesConnect.mock.calls;
  if (calls.length === 0) throw new Error("listProviderRepositoriesConnect not called");
  return fromBinary(ListProviderRepositoriesRequestSchema, calls[calls.length - 1][0] as Uint8Array);
}

describe("ImportRepositoryModal - Provider Selection and Browse", () => {
  const mockOnClose = vi.fn();
  const mockOnImported = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    setupProviderMocks();
  });

  it("should navigate to browse step when provider is selected", async () => {
    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("My GitHub"));

    await waitFor(() => {
      expect(stableCredSvc.listProviderRepositoriesConnect).toHaveBeenCalled();
      const req = lastListProviderReposCall();
      expect(req.id).toBe(BigInt(1));
      expect(req.page).toBe(1);
      expect(req.perPage).toBe(20);
    });
  });

  it("should display repositories after selecting provider", async () => {
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
  });

  it("should allow going back to source selection", async () => {
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

    const backButtons = document.querySelectorAll("button");
    const backButton = Array.from(backButtons).find(
      (btn) => btn.querySelector('svg path[d*="M15 19l-7-7 7-7"]')
    );
    expect(backButton).toBeTruthy();
    fireEvent.click(backButton!);

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
      expect(screen.getByText("My GitLab")).toBeInTheDocument();
    });
  });

  it("should show search input in browse step", async () => {
    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("My GitHub"));

    await waitFor(() => {
      expect(screen.getByPlaceholderText("Search repositories...")).toBeInTheDocument();
    });
  });

  it("should handle search form submission", async () => {
    render(
      <ImportRepositoryModal open={true} onClose={mockOnClose} onImported={mockOnImported} />
    );

    await waitFor(() => {
      expect(screen.getByText("My GitHub")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("My GitHub"));

    await waitFor(() => {
      expect(screen.getByPlaceholderText("Search repositories...")).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText("Search repositories...");
    fireEvent.change(searchInput, { target: { value: "test-search" } });

    const searchButton = screen.getByText("Search");
    fireEvent.click(searchButton);

    await waitFor(() => {
      const req = lastListProviderReposCall();
      expect(req.id).toBe(BigInt(1));
      expect(req.search).toBe("test-search");
    });
  });
});
