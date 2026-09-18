import { Check, GitBranch, ShieldCheck } from "lucide-react";

import type { FixtureSnapshot, TransitionRecord } from "../../../src/lib/types";

function shortHash(value: string) {
  return `${value.slice(0, 8)}…${value.slice(-6)}`;
}

export function CueRail({ transitions, snapshots, attempted = false }: { transitions: TransitionRecord[]; snapshots: FixtureSnapshot[]; attempted?: boolean }) {
  return (
    <div className="cue-rail" aria-label="Committed memory transition timeline">
      <div className="cue-rail-line" />
      {transitions.map((transition, index) => (
        <div className="cue" key={transition.transitionId}>
          <div className="cue-node"><Check size={14} strokeWidth={2.4} /></div>
          <div className="cue-copy">
            <span className="cue-sequence">STATE {String(snapshots[index]?.sequence ?? 17 + index).padStart(2, "0")}</span>
            <strong>{snapshots[index]?.visibleLabel ?? (index === 0 ? "Language preference" : index === 1 ? "Claim policy" : index === 2 ? "Privacy boundary" : "Canonical head")}</strong>
            <code>{shortHash(transition.nextStateRoot)}</code>
          </div>
        </div>
      ))}
      {attempted && (
        <div className="cue cue-attempt">
          <div className="cue-node"><GitBranch size={14} strokeWidth={2.4} /></div>
          <div className="cue-copy">
            <span className="cue-sequence">RESTORED SNAPSHOT</span>
            <strong>Silent rollback rejected</strong>
            <code>BAD_PREVIOUS_STATE</code>
          </div>
        </div>
      )}
      {!attempted && <div className="cue-end"><ShieldCheck size={16} /> head verified</div>}
    </div>
  );
}
