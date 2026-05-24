// Facade re-export of the ticket Connect-RPC adapter. Business code imports
// from here (or from the `@/lib/api` barrel) so the wire-shape layer stays
// internal to the facade boundary. Tests mock this path.

export {
  fromProtoTicket,
  fromProtoLabel,
  listTickets,
  getTicket,
  createTicket,
  updateTicket,
  deleteTicket,
  updateTicketStatus,
  getActiveTickets,
  getBoard,
  getSubTickets,
  addAssignee,
  removeAssignee,
  listLabels,
  createLabel,
  updateLabel,
  deleteLabel,
  addLabel,
  removeLabel,
  type CreateTicketInput,
  type UpdateTicketInput,
} from "../connect/ticketConnect";
