import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";

/** Legacy route kept for bookmark compatibility.
 *  Runners now live under Settings > Organization > Infrastructure. */
export function RunnersPage() {
  const navigate = useNavigate();
  const { org } = useParams<{ org: string }>();

  useEffect(() => {
    if (org) {
      navigate(`/${org}/settings?scope=organization&tab=infra/runners`, { replace: true });
    }
  }, [org, navigate]);

  return null;
}

export default RunnersPage;
