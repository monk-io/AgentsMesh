"use client";

import { useEffect } from "react";
import { useRouter, useParams } from "next/navigation";

export default function RunnersRedirectPage() {
  const router = useRouter();
  const { org } = useParams<{ org: string }>();

  useEffect(() => {
    if (org) router.replace(`/${org}/infra?tab=runners`);
  }, [org, router]);

  return null;
}
