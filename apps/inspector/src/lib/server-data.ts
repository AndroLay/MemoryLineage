import { readFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import path from "node:path";

import type {
  FixtureManifest,
  InspectorData,
  LocalEvidence,
  SepoliaEvidence,
  SepoliaReread,
  TransitionRecord,
} from "./types";

function repositoryRoot(): string {
  const candidates = [
    process.env.MEMORYLINEAGE_REPO_ROOT,
    path.resolve(process.cwd(), "../.."),
    path.resolve(process.cwd(), ".."),
    process.cwd(),
  ].filter((candidate): candidate is string => Boolean(candidate));

  const root = candidates.find((candidate) =>
    existsSync(path.join(candidate, "evidence", "local", "memory_lineage_evm_evidence.json")),
  );
  if (!root) throw new Error("MemoryLineage repository evidence was not found");
  return root;
}

async function readJson<T>(root: string, relativePath: string): Promise<T> {
  const contents = await readFile(path.join(root, relativePath), "utf8");
  return JSON.parse(contents) as T;
}

type RawLocalEvidence = Omit<LocalEvidence, "validHistory"> & {
  validHistory:
    | TransitionRecord[]
    | { transitions: TransitionRecord[]; finalHead?: LocalEvidence["validHistory"]["finalHead"] };
};

function normalizeLocalEvidence(raw: RawLocalEvidence): LocalEvidence {
  const transitions = Array.isArray(raw.validHistory)
    ? raw.validHistory
    : raw.validHistory.transitions;
  const last = transitions.at(-1);
  if (!last) throw new Error("Local evidence does not contain a valid transition history");

  return {
    ...raw,
    validHistory: {
      transitions,
      finalHead: {
        transitionId: last.transitionId,
        root: last.nextStateRoot,
        sequence: last.sequence,
      },
    },
  };
}

export async function loadInspectorData(): Promise<InspectorData> {
  const root = repositoryRoot();
  const [rawLocal, sepolia, reread, fixture] = await Promise.all([
    readJson<RawLocalEvidence>(root, "evidence/local/memory_lineage_evm_evidence.json"),
    readJson<SepoliaEvidence>(root, "evidence/sepolia/sepolia_deployment.json"),
    readJson<SepoliaReread>(root, "evidence/sepolia/sepolia_reread.json"),
    readJson<FixtureManifest>(root, "fixtures/silent-rollback/manifest.json"),
  ]);
  return { local: normalizeLocalEvidence(rawLocal), sepolia, reread, fixture };
}

export function resolveRepositoryRoot(): string {
  return repositoryRoot();
}
