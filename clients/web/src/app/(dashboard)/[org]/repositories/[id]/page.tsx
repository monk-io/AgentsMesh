"use client";

import { useEffect } from "react";
import { useRouter, useParams } from "next/navigation";

export default function RepositoryDetailRedirect() {
  const router = useRouter();
  const { org, id } = useParams<{ org: string; id: string }>();

  useEffect(() => {
    if (org && id) router.replace(`/${org}/infra?tab=repositories&id=${id}`);
  }, [org, id, router]);

  return null;
}
