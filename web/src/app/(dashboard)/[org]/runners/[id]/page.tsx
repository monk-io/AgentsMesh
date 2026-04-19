"use client";

import { useEffect } from "react";
import { useRouter, useParams } from "next/navigation";

export default function RunnerDetailRedirect() {
  const router = useRouter();
  const { org, id } = useParams<{ org: string; id: string }>();

  useEffect(() => {
    if (org && id) router.replace(`/${org}/infra?tab=runners&id=${id}`);
  }, [org, id, router]);

  return null;
}
