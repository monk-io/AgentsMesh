// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · ticket", () => {
  test("ticketAddAssigneeConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketAddAssigneeConnect", returnType: "Array<number>" }, []);
  });

  test("ticketAddLabelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketAddLabelConnect", returnType: "Array<number>" }, []);
  });

  test("ticketCreateLabelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketCreateLabelConnect", returnType: "Array<number>" }, []);
  });

  test("ticketCreateTicketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketCreateTicketConnect", returnType: "Array<number>" }, []);
  });

  test("ticketDeleteLabelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketDeleteLabelConnect", returnType: "Array<number>" }, []);
  });

  test("ticketDeleteTicketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketDeleteTicketConnect", returnType: "Array<number>" }, []);
  });

  test("ticketGetActiveTicketsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketGetActiveTicketsConnect", returnType: "Array<number>" }, []);
  });

  test("ticketGetBoardConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketGetBoardConnect", returnType: "Array<number>" }, []);
  });

  test("ticketGetSubTicketsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketGetSubTicketsConnect", returnType: "Array<number>" }, []);
  });

  test("ticketGetTicketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketGetTicketConnect", returnType: "Array<number>" }, []);
  });

  test("ticketGetTicketPods", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketGetTicketPods", returnType: "string" }, "", false);
  });

  test("ticketListLabelsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketListLabelsConnect", returnType: "Array<number>" }, []);
  });

  test("ticketListTicketsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketListTicketsConnect", returnType: "Array<number>" }, []);
  });

  test("ticketRemoveAssigneeConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketRemoveAssigneeConnect", returnType: "Array<number>" }, []);
  });

  test("ticketRemoveLabelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketRemoveLabelConnect", returnType: "Array<number>" }, []);
  });

  test("ticketTicketPodsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketTicketPodsJson", returnType: "string" }, "");
  });

  test("ticketUpdateLabelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketUpdateLabelConnect", returnType: "Array<number>" }, []);
  });

  test("ticketUpdateTicketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketUpdateTicketConnect", returnType: "Array<number>" }, []);
  });

  test("ticketUpdateTicketStatusConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "ticketUpdateTicketStatusConnect", returnType: "Array<number>" }, []);
  });
});
