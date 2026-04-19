import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";

/** Legacy route kept for bookmark compatibility.
 *  Repositories now live under Settings > Organization > Infrastructure. */
export function RepositoriesPage() {
  const navigate = useNavigate();
  const { org } = useParams<{ org: string }>();

  useEffect(() => {
    if (org) {
      navigate(`/${org}/settings?scope=organization&tab=infra/repositories`, { replace: true });
    }
  }, [org, navigate]);

  return null;
}

export default RepositoriesPage;
