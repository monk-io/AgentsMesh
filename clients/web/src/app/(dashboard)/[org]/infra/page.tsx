"use client";

import { useSearchParams, useRouter, useParams } from "next/navigation";
import { useEffect, useCallback, useState } from "react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { CenteredSpinner } from "@/components/ui/spinner";
import { FolderGit2, Server, Plus } from "lucide-react";
import { getRepositoryService } from "@/lib/wasm-core";
import type { RepositoryData } from "@/lib/api/repositoryTypes";
import { useRunners, useRunnerStore } from "@/stores/runner";
import { InfraRepositoryDetail } from "@/components/infra/InfraRepositoryDetail";
import { InfraRunnerDetail } from "@/components/infra/InfraRunnerDetail";

type InfraTab = "repositories" | "runners";

export default function InfraPage() {
  const router = useRouter();
  const params = useParams<{ org: string }>();
  const searchParams = useSearchParams();
  const t = useTranslations();

  const tab = (searchParams.get("tab") as InfraTab) ?? "runners";
  const idParam = searchParams.get("id");
  const selectedId = idParam ? Number(idParam) : NaN;

  useEffect(() => {
    if (!searchParams.get("tab")) {
      router.replace(`/${params.org}/infra?tab=runners`);
    }
  }, [searchParams, router, params.org]);

  const handleBack = useCallback(() => {
    router.push(`/${params.org}/infra?tab=${tab}`);
  }, [router, params.org, tab]);

  return (
    <div className="h-full w-full overflow-auto">
      <div className="mx-auto max-w-[1400px] px-6 py-6">
        {tab === "runners" ? (
          <RunnerSection
            orgSlug={params.org}
            selectedId={selectedId}
            idMissing={!idParam}
            onBack={handleBack}
            t={t}
          />
        ) : (
          <RepoSection
            orgSlug={params.org}
            selectedId={selectedId}
            idMissing={!idParam}
            onBack={handleBack}
            t={t}
          />
        )}
      </div>
    </div>
  );
}

function RepoSection({
  orgSlug,
  selectedId,
  idMissing,
  onBack,
  t,
}: {
  orgSlug: string;
  selectedId: number;
  idMissing: boolean;
  onBack: () => void;
  t: (k: string) => string;
}) {
  const router = useRouter();
  const [loading, setLoading] = useState(true);
  const [firstId, setFirstId] = useState<number | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const raw = await getRepositoryService().list();
        const parsed = JSON.parse(raw) as { repositories?: RepositoryData[] };
        setFirstId(parsed.repositories?.[0]?.id ?? null);
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  useEffect(() => {
    if (!idMissing || loading || firstId == null) return;
    router.replace(`/${orgSlug}/infra?tab=repositories&id=${firstId}`);
  }, [idMissing, loading, firstId, router, orgSlug]);

  if (loading) return <CenteredSpinner className="h-64" />;

  if (idMissing && firstId == null) {
    return (
      <EmptyState
        size="full"
        icon={<FolderGit2 className="h-12 w-12" />}
        title={t("repositories.emptyTitle")}
        description={t("repositories.emptyDescription")}
        actions={
          <Button onClick={() => router.push(`/${orgSlug}/infra?tab=repositories&import=1`)}>
            <Plus className="mr-1 h-4 w-4" />
            {t("repositories.import")}
          </Button>
        }
      />
    );
  }

  if (Number.isNaN(selectedId)) return null;
  return <InfraRepositoryDetail repositoryId={selectedId} onBack={onBack} />;
}

function RunnerSection({
  orgSlug,
  selectedId,
  idMissing,
  onBack,
  t,
}: {
  orgSlug: string;
  selectedId: number;
  idMissing: boolean;
  onBack: () => void;
  t: (k: string) => string;
}) {
  const router = useRouter();
  const runners = useRunners();
  const loading = useRunnerStore((s) => s.loading);
  const fetchRunners = useRunnerStore((s) => s.fetchRunners);

  useEffect(() => {
    fetchRunners();
  }, [fetchRunners]);

  const firstId = runners[0]?.id ?? null;

  useEffect(() => {
    if (!idMissing || loading || firstId == null) return;
    router.replace(`/${orgSlug}/infra?tab=runners&id=${firstId}`);
  }, [idMissing, loading, firstId, router, orgSlug]);

  if (loading && runners.length === 0) return <CenteredSpinner className="h-64" />;

  if (idMissing && firstId == null) {
    return (
      <EmptyState
        size="full"
        icon={<Server className="h-12 w-12" />}
        title={t("runners.emptyState.title")}
        description={t("runners.emptyState.description")}
        actions={
          <Button onClick={() => router.push(`/${orgSlug}/infra?tab=runners&add=1`)}>
            <Plus className="mr-1 h-4 w-4" />
            {t("runners.addRunner")}
          </Button>
        }
      />
    );
  }

  if (Number.isNaN(selectedId)) return null;
  return <InfraRunnerDetail runnerId={selectedId} onBack={onBack} />;
}
