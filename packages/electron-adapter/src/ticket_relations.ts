import { invoke } from "./invoke";
import type { ITicketRelationsService } from "@agentsmesh/service-interface";

export class ElectronTicketRelationsService implements ITicketRelationsService {
  async list_relations(slug: string): Promise<string> {
    return invoke<string>("ticketRelationsListRelations", slug);
  }

  async create_relation(slug: string, json: string): Promise<string> {
    return invoke<string>("ticketRelationsCreateRelation", slug, json);
  }

  async delete_relation(slug: string, relationId: bigint): Promise<void> {
    await invoke<void>("ticketRelationsDeleteRelation", slug, Number(relationId));
  }

  async list_comments(slug: string, limit?: number | null, offset?: number | null): Promise<string> {
    return invoke<string>("ticketRelationsListComments", slug, limit, offset);
  }

  async create_comment(slug: string, json: string): Promise<string> {
    return invoke<string>("ticketRelationsCreateComment", slug, json);
  }

  async update_comment(slug: string, commentId: bigint, json: string): Promise<string> {
    return invoke<string>("ticketRelationsUpdateComment", slug, Number(commentId), json);
  }

  async delete_comment(slug: string, commentId: bigint): Promise<void> {
    await invoke<void>("ticketRelationsDeleteComment", slug, Number(commentId));
  }

  async list_commits(slug: string): Promise<string> {
    return invoke<string>("ticketRelationsListCommits", slug);
  }

  async link_commit(slug: string, json: string): Promise<string> {
    return invoke<string>("ticketRelationsLinkCommit", slug, json);
  }

  async unlink_commit(slug: string, commitId: bigint): Promise<void> {
    await invoke<void>("ticketRelationsUnlinkCommit", slug, Number(commitId));
  }

  async list_merge_requests(slug: string): Promise<string> {
    return invoke<string>("ticketRelationsListMergeRequests", slug);
  }
}
