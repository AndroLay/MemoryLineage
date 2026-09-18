import { execFile } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";
import { promisify } from "node:util";

import { NextResponse } from "next/server";

export const runtime = "nodejs";

const execFileAsync = promisify(execFile);

function repositoryRoot() {
  const candidates = [
    process.env.MEMORYLINEAGE_REPO_ROOT,
    path.resolve(process.cwd(), "../.."),
    path.resolve(process.cwd(), ".."),
    process.cwd(),
  ].filter((candidate): candidate is string => Boolean(candidate));
  const root = candidates.find((candidate) => existsSync(path.join(candidate, "evm", "scripts", "simulate_silent_rollback.mjs")));
  if (!root) throw new Error("Simulation script was not found");
  return root;
}

export async function POST() {
  try {
    const root = repositoryRoot();
    const { stdout } = await execFileAsync(process.execPath, [path.join(root, "evm", "scripts", "simulate_silent_rollback.mjs")], {
      cwd: root,
      maxBuffer: 2 * 1024 * 1024,
      env: process.env,
    });
    return NextResponse.json(JSON.parse(stdout));
  } catch (error) {
    return NextResponse.json(
      { status: "ERROR", reason: error instanceof Error ? error.message : "Simulation failed" },
      { status: 500 },
    );
  }
}
