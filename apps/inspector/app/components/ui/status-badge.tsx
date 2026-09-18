import type { ReactNode } from "react";

export function StatusBadge({ tone, children }: { tone: "verified" | "danger" | "warning" | "neutral"; children: ReactNode }) {
  return <span className={`status-badge status-${tone}`}><span className="status-dot" />{children}</span>;
}
