import { z } from "zod";

import type { InspectorData, TransitionRecord } from "./types";

const transitionSchema = z.object({
  spaceId: z.string(),
  sequence: z.number().int().positive(),
  prevStateRoot: z.string(),
  nextStateRoot: z.string(),
  transitionId: z.string(),
  deltaCommitment: z.string(),
  provenanceCommitment: z.string(),
  profileId: z.string(),
  locatorCommitment: z.string(),
});

const portableSchema = z.object({
  schemaVersion: z.literal("memorylineage-evidence-v1"),
  evidenceType: z.literal("memorylineage_inspector_export"),
  network: z.object({ name: z.string(), chainId: z.string() }),
  registry: z.object({ address: z.string(), spaceId: z.string() }),
  head: z.object({ sequence: z.number(), transitionId: z.string(), stateRoot: z.string() }),
  transitions: z.array(transitionSchema).min(1),
  attack: z.record(z.unknown()).optional(),
  privacy: z.object({ rawMemoryOnChain: z.literal(false) }),
});

export type PortableEvidence = z.infer<typeof portableSchema>;

export type PortableVerification = {
  verdict: "VERIFIED" | "REJECTED";
  reason?: string;
  checks?: Record<string, "PASS" | "REJECT">;
  sequence?: number;
  finalRoot?: string;
  transitionCount?: number;
};

export function createPortableEvidence(data: InspectorData, attack?: Record<string, unknown>): PortableEvidence {
  const transitions: TransitionRecord[] = data.local.validHistory.transitions.map((transition) => ({
    spaceId: transition.spaceId,
    sequence: transition.sequence,
    prevStateRoot: transition.prevStateRoot,
    deltaCommitment: transition.deltaCommitment,
    provenanceCommitment: transition.provenanceCommitment,
    profileId: transition.profileId,
    locatorCommitment: transition.locatorCommitment,
    transitionId: transition.transitionId,
    nextStateRoot: transition.nextStateRoot,
  }));
  const last = transitions[transitions.length - 1];
  return {
    schemaVersion: "memorylineage-evidence-v1",
    evidenceType: "memorylineage_inspector_export",
    network: { name: "Local EthereumJS conformance evidence", chainId: data.local.chainId },
    registry: { address: data.local.registryAddress, spaceId: transitions[0].spaceId },
    head: { sequence: last.sequence, transitionId: last.transitionId, stateRoot: last.nextStateRoot },
    transitions,
    attack,
    privacy: { rawMemoryOnChain: false },
  };
}

export function verifyPortableEvidence(value: unknown): PortableVerification {
  const parsed = portableSchema.safeParse(value);
  if (!parsed.success) return { verdict: "REJECTED", reason: "SCHEMA_INVALID" };

  const bundle = parsed.data;
  const forbidden = ["payload", "provenance", "locator", "rawMemory", "memoryText"];
  const serialized = JSON.stringify(value);
  const leaked = forbidden.find((field) => serialized.includes(`"${field}"`));
  if (leaked) return { verdict: "REJECTED", reason: `PRIVATE_FIELD_EXPORTED:${leaked}` };

  let expectedSequence = 1;
  let previousRoot = `0x${"00".repeat(32)}`;
  for (const transition of bundle.transitions) {
    if (transition.sequence !== expectedSequence) return { verdict: "REJECTED", reason: "BAD_SEQUENCE" };
    if (transition.prevStateRoot !== previousRoot) return { verdict: "REJECTED", reason: "BAD_PREVIOUS_STATE" };
    previousRoot = transition.nextStateRoot;
    expectedSequence += 1;
  }

  const last = bundle.transitions[bundle.transitions.length - 1];
  if (bundle.head.sequence !== last.sequence) return { verdict: "REJECTED", reason: "HEAD_SEQUENCE_MISMATCH" };
  if (bundle.head.transitionId !== last.transitionId) return { verdict: "REJECTED", reason: "HEAD_TRANSITION_MISMATCH" };
  if (bundle.head.stateRoot !== last.nextStateRoot) return { verdict: "REJECTED", reason: "HEAD_ROOT_MISMATCH" };

  return {
    verdict: "VERIFIED",
    checks: {
      schema: "PASS",
      sequenceContinuity: "PASS",
      predecessorContinuity: "PASS",
      headMatch: "PASS",
      privacyBoundary: "PASS",
    },
    sequence: last.sequence,
    finalRoot: last.nextStateRoot,
    transitionCount: bundle.transitions.length,
  };
}

export function downloadEvidence(bundle: PortableEvidence): void {
  const blob = new Blob([JSON.stringify(bundle, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = "memorylineage-evidence.json";
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  URL.revokeObjectURL(url);
}
