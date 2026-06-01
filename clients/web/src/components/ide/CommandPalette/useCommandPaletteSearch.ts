"use client";

import { useState, useEffect } from "react";
import { listPods as listPodsConnect } from "@/lib/api/facade/podConnect";
import { listTickets as listTicketsConnect } from "@/lib/api/facade/ticketConnect";
import { readCurrentOrg } from "@/stores/auth";
import { useRepositories, useRepositoryStore } from "@/stores/repository";
import type { SearchResults, PodSearchResult, TicketSearchResult, RepositorySearchResult } from "./types";

function orgSlug(): string {
  return readCurrentOrg()?.slug ?? "";
}

export function useCommandPaletteSearch(search: string): SearchResults {
  const [pods, setPods] = useState<PodSearchResult[]>([]);
  const [tickets, setTickets] = useState<TicketSearchResult[]>([]);
  const [repositories, setRepositories] = useState<RepositorySearchResult[]>([]);
  const [loading, setLoading] = useState(false);

  const allRepos = useRepositories();
  const fetchRepositories = useRepositoryStore((s) => s.fetchRepositories);
  useEffect(() => { fetchRepositories(); }, [fetchRepositories]);

  useEffect(() => {
    if (!search || search.length < 2) {
      setPods([]);
      setTickets([]);
      setRepositories([]);
      return;
    }

    const loadSearchResults = async () => {
      setLoading(true);
      try {
        const slug = orgSlug();
        const [podsRes, ticketsRes] = await Promise.all([
          listPodsConnect(slug, {}).catch(() => ({ items: [] as { pod_key: string }[] })),
          listTicketsConnect(slug, { limit: 500 }).catch(
            () => ({ items: [] as { slug: string; title: string }[] }),
          ),
        ]);

        const searchLower = search.toLowerCase();
        setPods(
          (podsRes.items as PodSearchResult[])
            .filter((p) => p.pod_key.toLowerCase().includes(searchLower))
            .slice(0, 5)
        );
        setTickets(
          (ticketsRes.items as TicketSearchResult[])
            .filter(
              (ticket) =>
                ticket.slug.toLowerCase().includes(searchLower) ||
                ticket.title.toLowerCase().includes(searchLower)
            )
            .slice(0, 5)
        );
        setRepositories(
          allRepos
            .filter((r) => r.slug.toLowerCase().includes(searchLower))
            .slice(0, 5)
        );
      } catch (error) {
        console.error("Search error:", error);
      } finally {
        setLoading(false);
      }
    };

    const debounce = setTimeout(loadSearchResults, 300);
    return () => clearTimeout(debounce);
  }, [search, allRepos]);

  return { pods, tickets, repositories, loading };
}
