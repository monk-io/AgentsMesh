"use client";

import { useState, useCallback, useEffect } from "react";
import { Ticket } from "@/stores/ticket";
import { TicketRelation, TicketCommit, TicketComment } from "@/lib/api";
import { getSubTickets as getSubTicketsConnect } from "@/lib/api/facade/ticketConnect";
import {
  listRelations,
  listCommits,
  listComments,
  createComment,
  updateComment,
  deleteComment,
} from "@/lib/api/facade/ticketRelations";
import { readCurrentOrg } from "@/stores/auth";

function orgSlug(): string {
  return readCurrentOrg()?.slug ?? "";
}

export interface TicketExtraData {
  subTickets: Ticket[];
  relations: TicketRelation[];
  commits: TicketCommit[];
  comments: TicketComment[];
}

export function useTicketExtraData(slug: string, enabled: boolean) {
  const [subTickets, setSubTickets] = useState<Ticket[]>([]);
  const [relations, setRelations] = useState<TicketRelation[]>([]);
  const [commits, setCommits] = useState<TicketCommit[]>([]);
  const [comments, setComments] = useState<TicketComment[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchExtraData = useCallback(async () => {
    if (!enabled || !slug) return;

    setLoading(true);
    try {
      const org = orgSlug();
      const [subTicketsRes, relationsRes, commitsRes, commentsRes] = await Promise.all([
        getSubTicketsConnect(org, slug).catch(() => [] as Ticket[]),
        listRelations(org, slug).catch(() => ({ relations: [] as TicketRelation[] })),
        listCommits(org, slug).catch(() => ({ commits: [] as TicketCommit[] })),
        listComments(org, slug).catch(() => ({ comments: [] as TicketComment[], total: 0 })),
      ]);

      setSubTickets((subTicketsRes as Ticket[]) ?? []);
      setRelations(relationsRes.relations || []);
      setCommits(commitsRes.commits || []);
      setComments(commentsRes.comments || []);
    } catch (err) {
      console.error("Failed to fetch extra data:", err);
    } finally {
      setLoading(false);
    }
  }, [slug, enabled]);

  useEffect(() => {
    fetchExtraData();
  }, [fetchExtraData]);

  const addComment = useCallback(
    async (
      content: string,
      parentId?: number,
      mentions?: Array<{ user_id: number; username: string }>
    ) => {
      const org = orgSlug();
      await createComment(org, slug, { content, parent_id: parentId, mentions });
      const commentsRes = await listComments(org, slug);
      setComments(commentsRes.comments || []);
    },
    [slug]
  );

  const updateCommentFn = useCallback(
    async (
      commentId: number,
      content: string,
      mentions?: Array<{ user_id: number; username: string }>
    ) => {
      const org = orgSlug();
      await updateComment(org, slug, commentId, { content, mentions });
      const commentsRes = await listComments(org, slug);
      setComments(commentsRes.comments || []);
    },
    [slug]
  );

  const deleteCommentFn = useCallback(
    async (commentId: number) => {
      const org = orgSlug();
      await deleteComment(org, slug, commentId);
      const commentsRes = await listComments(org, slug);
      setComments(commentsRes.comments || []);
    },
    [slug]
  );

  return {
    subTickets,
    relations,
    commits,
    comments,
    loading,
    refetch: fetchExtraData,
    addComment,
    updateComment: updateCommentFn,
    deleteComment: deleteCommentFn,
  };
}

export default useTicketExtraData;
