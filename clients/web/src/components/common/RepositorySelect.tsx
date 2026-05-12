"use client";

import { useState, useEffect, useCallback } from "react";
import { RepositoryData } from "@/lib/api";
import { listRepositories } from "@/lib/api/repositoryConnect";
import { useCurrentOrg } from "@/stores/auth";

export interface RepositorySelectProps {
  value: number | null;
  onChange: (value: number | null, repository?: RepositoryData) => void;
  disabled?: boolean;
  placeholder?: string;
  className?: string;
  /** Show only active repositories */
  activeOnly?: boolean;
}

export function RepositorySelect({
  value,
  onChange,
  disabled = false,
  placeholder = "Select a repository...",
  className = "",
  activeOnly = true,
}: RepositorySelectProps) {
  const currentOrg = useCurrentOrg();
  const [repositories, setRepositories] = useState<RepositoryData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadRepositories = useCallback(async () => {
    if (!currentOrg) {
      setLoading(false);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const res = await listRepositories(currentOrg.slug);
      let repos: RepositoryData[] = res.items;
      if (activeOnly) {
        repos = repos.filter((r) => r.is_active);
      }
      setRepositories(repos);
    } catch (err) {
      console.error("Failed to load repositories:", err);
      setError("Failed to load repositories");
    } finally {
      setLoading(false);
    }
  }, [activeOnly, currentOrg]);

  useEffect(() => {
    loadRepositories();
  }, [loadRepositories]);

  const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedValue = e.target.value;
    if (!selectedValue) {
      onChange(null);
    } else {
      const selectedId = Number(selectedValue);
      const selectedRepo = repositories.find((r) => r.id === selectedId);
      onChange(selectedId, selectedRepo);
    }
  };

  if (error) {
    return (
      <div className={`text-sm text-destructive ${className}`}>
        {error}
        <button
          type="button"
          onClick={loadRepositories}
          className="ml-2 underline hover:no-underline"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <select
      className={`w-full px-3 py-2 border border-border rounded-md bg-background ${className}`}
      value={value || ""}
      onChange={handleChange}
      disabled={disabled || loading}
    >
      <option value="">
        {loading ? "Loading repositories..." : placeholder}
      </option>
      {repositories.map((repo) => (
        <option key={repo.id} value={repo.id}>
          {repo.slug}
        </option>
      ))}
    </select>
  );
}

export default RepositorySelect;
