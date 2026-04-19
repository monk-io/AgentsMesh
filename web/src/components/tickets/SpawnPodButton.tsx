"use client";

import { useState, useMemo } from "react";
import { Button } from "@/components/ui/button";
import { CreatePodModal } from "@/components/ide/CreatePodModal";
import { Terminal } from "lucide-react";
import { useTranslations } from "next-intl";
import type { Ticket } from "@/stores/ticket";

interface SpawnPodButtonProps {
  ticket: Ticket;
  ticketSlug: string;
  onPodCreated?: () => void;
  size?: "default" | "lg";
  className?: string;
}

/** Prominent CTA matching the design's "Spawn Pod" button on ticket detail header. */
export function SpawnPodButton({
  ticket,
  ticketSlug,
  onPodCreated,
  size = "lg",
  className,
}: SpawnPodButtonProps) {
  const t = useTranslations();
  const [showModal, setShowModal] = useState(false);

  const ticketContext = useMemo(
    () =>
      ticket.id
        ? {
            id: ticket.id,
            slug: ticketSlug,
            title: ticket.title,
            repositoryId: ticket.repository_id,
          }
        : undefined,
    [ticket.id, ticketSlug, ticket.title, ticket.repository_id],
  );

  return (
    <>
      <Button
        size={size}
        className={className}
        onClick={() => setShowModal(true)}
      >
        <Terminal className="h-4 w-4" />
        {t("tickets.detail.spawnPod")}
      </Button>
      <CreatePodModal
        open={showModal}
        onClose={() => setShowModal(false)}
        onCreated={() => {
          setShowModal(false);
          onPodCreated?.();
        }}
        ticketContext={ticketContext}
      />
    </>
  );
}
