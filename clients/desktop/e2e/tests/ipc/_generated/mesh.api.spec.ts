// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · mesh", () => {
  test("meshBatchGetTicketPodsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "meshBatchGetTicketPodsConnect", returnType: "Uint8Array" }, []);
  });

  test("meshCreatePodForTicketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "meshCreatePodForTicketConnect", returnType: "Uint8Array" }, []);
  });

  test("meshFetchTopology", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "meshFetchTopology", returnType: "Uint8Array" });
  });

  test("meshGetMeshTopologyConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "meshGetMeshTopologyConnect", returnType: "Uint8Array" }, []);
  });

  test("meshGetTicketPodsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "meshGetTicketPodsConnect", returnType: "Uint8Array" }, []);
  });
});
