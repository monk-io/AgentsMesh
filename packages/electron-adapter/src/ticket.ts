import { invoke } from "./invoke";
import type { ITicketService } from "@agentsmesh/service-interface";

export class ElectronTicketService implements ITicketService {
  private _ticketsCache = "[]";
  private _labelsCache = "[]";
  private _boardColumnsCache = "[]";
  private _currentTicketCache: string | null = null;

  tickets_json(): string { return this._ticketsCache; }
  labels_json(): string { return this._labelsCache; }
  board_columns_json(): string { return this._boardColumnsCache; }
  current_ticket_json(): unknown { return this._currentTicketCache; }

  get_ticket_by_slug_json(slug: string): unknown {
    const tickets = JSON.parse(this._ticketsCache) as { slug: string }[];
    const t = tickets.find(x => x.slug === slug);
    return t ? JSON.stringify(t) : null;
  }

  filter_tickets_json(search: string, statusesJson: string, prioritiesJson: string, repoIdsJson: string): string {
    return this._ticketsCache;
  }

  set_tickets(json: string): void { this._ticketsCache = json; }
  set_labels(json: string): void { this._labelsCache = json; }
  set_board_columns(json: string): void { this._boardColumnsCache = json; }
  set_current_ticket(json: string): void { this._currentTicketCache = json || null; }

  add_ticket(json: string): void {
    const tickets = JSON.parse(this._ticketsCache) as unknown[];
    tickets.push(JSON.parse(json));
    this._ticketsCache = JSON.stringify(tickets);
  }

  add_label(json: string): void {
    const labels = JSON.parse(this._labelsCache) as unknown[];
    labels.push(JSON.parse(json));
    this._labelsCache = JSON.stringify(labels);
  }

  remove_ticket(slug: string): void {
    const tickets = JSON.parse(this._ticketsCache) as { slug: string }[];
    this._ticketsCache = JSON.stringify(tickets.filter(x => x.slug !== slug));
  }

  remove_label(id: number): void {
    const labels = JSON.parse(this._labelsCache) as { id: number }[];
    this._labelsCache = JSON.stringify(labels.filter(x => x.id !== id));
  }

  update_ticket_local(slug: string, json: string): void {
    const tickets = JSON.parse(this._ticketsCache) as { slug: string }[];
    const idx = tickets.findIndex(x => x.slug === slug);
    if (idx >= 0) tickets[idx] = { ...tickets[idx], ...JSON.parse(json) };
    this._ticketsCache = JSON.stringify(tickets);
  }

  update_ticket_status_local(slug: string, status: string): void {
    const tickets = JSON.parse(this._ticketsCache) as { slug: string; status?: string }[];
    const t = tickets.find(x => x.slug === slug);
    if (t) t.status = status;
    this._ticketsCache = JSON.stringify(tickets);
  }

  append_column_tickets(status: string, json: string): void {
    const cols = JSON.parse(this._boardColumnsCache) as { status: string; tickets: unknown[] }[];
    const col = cols.find(c => c.status === status);
    if (col) col.tickets.push(...(JSON.parse(json) as unknown[]));
    this._boardColumnsCache = JSON.stringify(cols);
  }

  async fetch_tickets(status?: string | null, limit?: number | null, offset?: number | null): Promise<string> {
    const result = await invoke<string>("ticketFetchTickets", status, limit, offset);
    const parsed = JSON.parse(result);
    this._ticketsCache = JSON.stringify(parsed.tickets ?? []);
    return result;
  }

  async fetch_ticket(slug: string): Promise<string> {
    const result = await invoke<string>("ticketFetchTicket", slug);
    this._currentTicketCache = result;
    return result;
  }

  async fetch_board(repositoryId?: bigint | null): Promise<string> {
    const result = await invoke<string>("ticketFetchBoard", repositoryId ? Number(repositoryId) : null);
    const parsed = JSON.parse(result) as { columns?: Array<{ tickets?: unknown[]; total_count?: number; count?: number }> };
    const columns = (parsed.columns ?? []).map((c) => ({
      ...c,
      count: c.count ?? c.total_count ?? (c.tickets?.length ?? 0),
    }));
    this._boardColumnsCache = JSON.stringify(columns);
    this._ticketsCache = JSON.stringify(columns.flatMap((c) => c.tickets ?? []));
    return JSON.stringify({ ...parsed, columns });
  }

  async fetch_labels(repositoryId?: bigint | null): Promise<string> {
    const result = await invoke<string>("ticketFetchLabels", repositoryId ? Number(repositoryId) : null);
    const parsed = JSON.parse(result);
    this._labelsCache = JSON.stringify(parsed.labels ?? []);
    return result;
  }

  async create_ticket(json: string): Promise<string> {
    const result = await invoke<string>("ticketCreateTicket", json);
    this.add_ticket(result);
    this._currentTicketCache = result;
    return result;
  }

  async update_ticket(slug: string, json: string): Promise<string> {
    const result = await invoke<string>("ticketUpdateTicket", slug, json);
    this.update_ticket_local(slug, result);
    this._currentTicketCache = result;
    return result;
  }

  async update_ticket_status(slug: string, status: string): Promise<string> {
    const result = await invoke<string>("ticketUpdateTicketStatus", slug, status);
    this.update_ticket_status_local(slug, status);
    return result;
  }

  async delete_ticket(slug: string): Promise<void> {
    await invoke<void>("ticketDeleteTicket", slug);
    this.remove_ticket(slug);
  }

  async create_label(name: string, color: string, repositoryId?: bigint | null): Promise<string> {
    const result = await invoke<string>("ticketCreateLabel", name, color, repositoryId ? Number(repositoryId) : null);
    this.add_label(result);
    return result;
  }

  async delete_label(id: number): Promise<void> {
    await invoke<void>("ticketDeleteLabel", id);
    this.remove_label(id);
  }

  async get_sub_tickets(slug: string): Promise<string> {
    return invoke<string>("ticketGetSubTickets", slug);
  }

  async get_ticket_pods(slug: string, activeOnly?: boolean | null): Promise<string> {
    return invoke<string>("ticketGetTicketPods", slug, activeOnly);
  }

  async load_more_column(status: string, offset: number, limit: number): Promise<string> {
    return invoke<string>("ticketLoadMoreColumn", status, offset, limit);
  }
}
