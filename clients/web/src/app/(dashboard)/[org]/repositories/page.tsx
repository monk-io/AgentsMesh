"use client";

import { useEffect } from "react";
import { useRouter, useParams } from "next/navigation";

export default function RepositoriesRedirectPage() {
  const router = useRouter();
  const { org } = useParams<{ org: string }>();

  useEffect(() => {
    if (org) router.replace(`/${org}/infra?tab=repositories`);
  }, [org, router]);

  return null;
}
