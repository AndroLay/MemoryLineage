"use client";

import { useMemo, useState } from "react";
import {
  Activity,
  AlertTriangle,
  ArrowDownRight,
  ArrowRight,
  Check,
  CheckCircle2,
  ClipboardCheck,
  Database,
  Download,
  ExternalLink,
  FileCheck2,
  GitBranch,
  KeyRound,
  LockKeyhole,
  Radio,
  RefreshCw,
  RotateCcw,
  ScanLine,
  ShieldCheck,
  Upload,
  XCircle,
} from "lucide-react";
import type { Address, Hex } from "viem";

import { inspectSepolia, type LiveInspection } from "../../src/lib/chain";
import {
  createPortableEvidence,
  downloadEvidence,
  verifyPortableEvidence,
  type PortableEvidence,
  type PortableVerification,
} from "../../src/lib/portable-evidence";
import type { InspectorData, TransitionRecord } from "../../src/lib/types";
import { Button } from "./ui/button";
import { CueRail } from "./ui/cue-rail";
import { StatusBadge } from "./ui/status-badge";

type Tab = "inspect" | "history" | "lab" | "verify";
type RollbackResult = {
  status: "REJECTED" | "UNEXPECTED_PASS" | "ERROR";
  reason: string | null;
  canonicalHead?: { sequence: number; root: string; transitionId: string };
  restoredSnapshotSequence?: number;
  attemptedSequence?: number;
  expectedReason?: string;
  execution?: string;
};

const tabs: { id: Tab; label: string; icon: typeof Activity }[] = [
  { id: "inspect", label: "Inspect", icon: ScanLine },
  { id: "history", label: "History", icon: Activity },
  { id: "lab", label: "Tampering Lab", icon: GitBranch },
  { id: "verify", label: "Verify", icon: FileCheck2 },
];

const attackCorpus = [
  ["sequence_gap", "BAD_SEQUENCE"],
  ["rollback_predecessor", "BAD_PREVIOUS_STATE"],
  ["parallel_history", "BAD_SEQUENCE"],
  ["wrong_eoa_signer", "INVALID_AUTHORIZATION"],
  ["locatorCommitment_signature_binding", "INVALID_AUTHORIZATION"],
  ["wrong_chain_domain", "INVALID_AUTHORIZATION"],
] as const;

function shortHash(value: string, head = 10, tail = 8) {
  if (!value) return "—";
  return `${value.slice(0, head)}…${value.slice(-tail)}`;
}

function shortAddress(value: string) {
  return shortHash(value, 7, 5);
}

function stateTitle(index: number, fixture: InspectorData["fixture"]) {
  return fixture.snapshots[index]?.visibleLabel ?? "Local conformance continuation";
}

function transitionFields(transition: TransitionRecord) {
  return [
    ["transition", transition.transitionId],
    ["previous root", transition.prevStateRoot],
    ["next root", transition.nextStateRoot],
    ["delta commitment", transition.deltaCommitment],
    ["provenance commitment", transition.provenanceCommitment],
    ["locator commitment", transition.locatorCommitment],
  ];
}

export default function InspectorShell({ initialData }: { initialData: InspectorData }) {
  const [activeTab, setActiveTab] = useState<Tab>("inspect");
  const [rollback, setRollback] = useState<RollbackResult | null>(null);
  const [rollbackBusy, setRollbackBusy] = useState(false);
  const [liveInspection, setLiveInspection] = useState<LiveInspection | null>(null);
  const [liveBusy, setLiveBusy] = useState(false);
  const [selectedAttack, setSelectedAttack] = useState("rollback_predecessor");
  const [verification, setVerification] = useState<PortableVerification | null>(null);
  const [importedName, setImportedName] = useState<string | null>(null);
  const [announcement, setAnnouncement] = useState("History loaded from published local evidence.");

  const transitions = initialData.local.validHistory.transitions;
  const mutationCases = initialData.local.mutationMatrix?.filter((mutation) => mutation.expected === "REJECT") ?? [];
  const mutationRejected = mutationCases.filter((mutation) => mutation.observed?.status === "REJECT").length;
  const localHead = transitions[transitions.length - 1];
  const authorityHistory = initialData.local.authorityHistory ?? [];
  const portableEvidence = useMemo(
    () => createPortableEvidence(initialData, rollback ? {
      name: "silent-rollback",
      status: rollback.status,
      reason: rollback.reason,
      restoredSnapshotSequence: rollback.restoredSnapshotSequence,
      attemptedSequence: rollback.attemptedSequence,
    } : undefined),
    [initialData, rollback],
  );

  const liveHead = liveInspection?.status === "LIVE"
    ? liveInspection.head
    : initialData.sepolia.onchainHead;
  const liveAuthority = liveInspection?.status === "LIVE"
    ? liveInspection.authority
    : { controller: "published deployment signer", authorizer: "published deployment signer", configNonce: 0 };

  async function runRollback() {
    setActiveTab("lab");
    setRollbackBusy(true);
    setAnnouncement("Running the local EthereumJS registry simulation…");
    try {
      const response = await fetch("/api/simulate/rollback", { method: "POST" });
      const result = (await response.json()) as RollbackResult;
      setRollback(result);
      setAnnouncement(result.status === "REJECTED" ? `Rollback rejected: ${result.reason}` : `Simulation returned ${result.status}`);
    } catch (error) {
      const result: RollbackResult = { status: "ERROR", reason: error instanceof Error ? error.message : "Simulation failed" };
      setRollback(result);
      setAnnouncement("The simulation could not run.");
    } finally {
      setRollbackBusy(false);
    }
  }

  async function inspectLiveRegistry() {
    setLiveBusy(true);
    setAnnouncement("Reading the Sepolia registry from the public RPC…");
    const result = await inspectSepolia(initialData.sepolia.registryAddress as Address, initialData.sepolia.spaceId as Hex);
    setLiveInspection(result);
    setLiveBusy(false);
    setAnnouncement(result.status === "LIVE" ? "Live Sepolia head matches the registry read." : result.message);
  }

  function exportEvidence() {
    downloadEvidence(portableEvidence);
    setVerification(verifyPortableEvidence(portableEvidence));
    setAnnouncement("Portable evidence downloaded and verified in the browser.");
  }

  function runTamperCheck() {
    const tampered: PortableEvidence = structuredClone(portableEvidence);
    tampered.head.stateRoot = `0x${"ff".repeat(32)}`;
    const result = verifyPortableEvidence(tampered);
    setVerification(result);
    setAnnouncement(`Tamper check: ${result.verdict}${result.reason ? ` / ${result.reason}` : ""}`);
  }

  async function importEvidence(file: File | undefined) {
    if (!file) return;
    setImportedName(file.name);
    try {
      const value: unknown = JSON.parse(await file.text());
      const result = verifyPortableEvidence(value);
      setVerification(result);
      setAnnouncement(`Imported ${file.name}: ${result.verdict}${result.reason ? ` / ${result.reason}` : ""}`);
    } catch {
      setVerification({ verdict: "REJECTED", reason: "JSON_INVALID" });
      setAnnouncement(`Imported ${file.name}: invalid JSON`);
    }
  }

  return (
    <div className="inspector-shell">
      <header className="topbar">
        <a className="brand" href="#top" aria-label="MemoryLineage Inspector home">
          <span className="brand-mark">ML</span>
          <span><strong>MEMORYLINEAGE</strong><small>INSPECTOR</small></span>
        </a>
        <div className="topbar-readout">
          <span className="topbar-live"><span />SEPOLIA / OBSERVED</span>
          <code>{shortAddress(initialData.sepolia.registryAddress)}</code>
        </div>
      </header>

      <main id="top">
        <section className="hero-section">
          <div className="hero-copy">
            <div className="cue-label"><span className="cue-label-line" /> CUE 04 / HISTORY INTEGRITY</div>
            <h1>Verify the history,<br /><em>not the memory.</em></h1>
            <p className="hero-lede">
              An independent audit surface for private agent state. Follow the
              committed sequence, try the stale restore, and take the evidence
              with you.
            </p>
            <div className="hero-actions">
              <Button variant="primary" onClick={runRollback} disabled={rollbackBusy}>
                {rollbackBusy ? <RefreshCw className="spin" size={16} /> : <RotateCcw size={16} />}
                {rollbackBusy ? "Running simulation" : "Run Silent Rollback"}
                {!rollbackBusy && <ArrowRight size={16} />}
              </Button>
              <Button variant="quiet" onClick={() => { setActiveTab("verify"); document.getElementById("workbench")?.scrollIntoView({ behavior: "smooth" }); }}>
                <FileCheck2 size={16} /> Verify evidence
              </Button>
            </div>
            <div className="hero-proof">
              <LockKeyhole size={17} />
              <span><strong>RAW MEMORY OFF-CHAIN</strong> / fixed-size commitments only</span>
            </div>
          </div>

          <div className="hero-board" aria-label="Canonical history evidence">
            <div className="board-header">
              <div><span className="board-kicker">LIVE CASE BOARD</span><strong>Private agent memory / canonical cue rail</strong></div>
              <StatusBadge tone="verified">VERIFIED</StatusBadge>
            </div>
            <CueRail transitions={transitions} snapshots={initialData.fixture.snapshots} attempted={rollback?.status === "REJECTED"} />
            <div className="board-footer">
              <span><span className="board-key sage" /> ordered history</span>
              <span><span className="board-key rose" /> rejected attempt</span>
              <code>HEAD {shortHash(localHead.nextStateRoot)}</code>
            </div>
          </div>
        </section>

        <section className="workbench" id="workbench">
          <nav className="tab-bar" aria-label="Inspector views" role="tablist">
            {tabs.map(({ id, label, icon: Icon }) => (
              <button
                className={`tab ${activeTab === id ? "tab-active" : ""}`}
                key={id}
                onClick={() => setActiveTab(id)}
                role="tab"
                aria-selected={activeTab === id}
              >
                <Icon size={16} /> {label}
              </button>
            ))}
            <span className="tab-spacer" />
            <span className="tab-announce" aria-live="polite">{announcement}</span>
          </nav>

          {activeTab === "inspect" && (
            <section className="tab-panel" aria-labelledby="inspect-title">
              <div className="section-heading">
                <div><h2 id="inspect-title">Registry inspection</h2><p>What the public registry can prove at this moment.</p></div>
                <Button variant="quiet" size="small" onClick={inspectLiveRegistry} disabled={liveBusy}>
                  {liveBusy ? <RefreshCw className="spin" size={15} /> : <Radio size={15} />}
                  {liveBusy ? "Reading RPC" : "Read Sepolia now"}
                </Button>
              </div>
              <div className="inspection-layout">
                <div className="ledger-panel">
                  <div className="ledger-title"><span>PUBLIC REGISTRY / READOUT</span><StatusBadge tone={liveInspection?.status === "LIVE" ? "verified" : "neutral"}>{liveInspection?.status === "LIVE" ? "LIVE" : "PUBLISHED EVIDENCE"}</StatusBadge></div>
                  <dl className="readout-list">
                    <div><dt>Network</dt><dd>Ethereum Sepolia <code>11155111</code></dd></div>
                    <div><dt>Registry</dt><dd><code>{shortAddress(initialData.sepolia.registryAddress)}</code><a href={`https://sepolia.etherscan.io/address/${initialData.sepolia.registryAddress}`} target="_blank" rel="noreferrer"><ExternalLink size={13} /></a></dd></div>
                    <div><dt>Memory space</dt><dd><code>{shortHash(initialData.sepolia.spaceId)}</code></dd></div>
                    <div><dt>Current sequence</dt><dd><strong>{liveHead.sequence}</strong> / root <code>{shortHash(liveHead.stateRoot)}</code></dd></div>
                    <div><dt>Read source</dt><dd>{liveInspection?.status === "LIVE" ? `RPC block ${liveInspection.blockNumber}` : `second RPC / block ${initialData.reread.blockNumber}`}</dd></div>
                  </dl>
                </div>
                <aside className="boundary-panel">
                  <div className="panel-icon"><ShieldCheck size={20} /></div>
                  <h3>Proof boundary</h3>
                  <p>Continuity and configured authority are visible. The private memory that produced each commitment is not.</p>
                  <ul className="check-list">
                    <li><CheckCircle2 size={15} /> ordered transition history</li>
                    <li><CheckCircle2 size={15} /> predecessor root continuity</li>
                    <li><CheckCircle2 size={15} /> authorization binding</li>
                    <li className="muted"><ArrowDownRight size={15} /> semantic truth is out of scope</li>
                  </ul>
                </aside>
              </div>
              <div className="authority-strip">
                <div className="authority-symbol"><KeyRound size={17} /></div>
                <div><span className="label">AUTHORITY CONFIGURATION</span><strong>Controller {typeof liveAuthority.controller === "string" && liveAuthority.controller.startsWith("0x") ? shortAddress(liveAuthority.controller) : liveAuthority.controller}</strong></div>
                <div><span className="label">AUTHORIZER</span><strong>{typeof liveAuthority.authorizer === "string" && liveAuthority.authorizer.startsWith("0x") ? shortAddress(liveAuthority.authorizer) : liveAuthority.authorizer}</strong></div>
                <div><span className="label">CONFIG NONCE</span><strong>{liveAuthority.configNonce}</strong></div>
                <span className="authority-note">Rotation rules exercised in the local contract suite.</span>
              </div>
              {authorityHistory.length > 1 && (
                <div className="authority-trace" aria-label="Local authority rotation evidence">
                  <div className="authority-trace-heading"><span className="label">LOCAL AUTHORITY TRACE</span><span>config nonce is part of the signed rotation</span></div>
                  <div className="authority-trace-steps">
                    {authorityHistory.map((entry, index) => (
                      <div className="authority-trace-step" key={`${entry.configNonce}-${entry.authorizer}`}>
                        <span className="trace-step-number">0{entry.configNonce}</span>
                        <div><strong>{entry.label}</strong><code>authorizer {shortAddress(entry.authorizer)}</code></div>
                        {index < authorityHistory.length - 1 && <ArrowRight size={15} />}
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </section>
          )}

          {activeTab === "history" && (
            <section className="tab-panel" aria-labelledby="history-title">
              <div className="section-heading"><div><h2 id="history-title">Canonical history</h2><p>Four transitions from the local EVM evidence bundle. Raw memory remains private.</p></div><StatusBadge tone="verified">{transitions.length} / {transitions.length} REPLAYED</StatusBadge></div>
              <div className="history-list">
                {transitions.map((transition, index) => (
                  <article className="history-entry" key={transition.transitionId}>
                    <div className="history-marker"><span>{String(index + 1).padStart(2, "0")}</span><div /></div>
                    <div className="history-content">
                      <div className="history-topline"><span className="cue-sequence">STATE {17 + index}</span><StatusBadge tone="verified">COMMITTED</StatusBadge><code>SEQ {transition.sequence}</code></div>
                      <h3>{stateTitle(index, initialData.fixture)}</h3>
                      <p>Authorized transition extends the previous committed root.</p>
                      <div className="hash-table">{transitionFields(transition).map(([label, value]) => <div key={label}><span>{label}</span><code>{shortHash(value, 16, 12)}</code></div>)}</div>
                    </div>
                  </article>
                ))}
              </div>
              <div className="history-footnote"><Database size={16} /><span>Evidence source: <code>evidence/local/memory_lineage_evm_evidence.json</code>. Payload, provenance, and locator values are intentionally absent.</span></div>
            </section>
          )}

          {activeTab === "lab" && (
            <section className="tab-panel" aria-labelledby="lab-title">
              <div className="section-heading"><div><h2 id="lab-title">Lineage tampering lab</h2><p>Rehearse a failure against the actual local registry harness.</p></div><span className="lab-count"><strong>{mutationCases.length}</strong> corpus cases / <strong>{mutationRejected}</strong> rejected</span></div>
              <div className="lab-layout">
                <div className="attack-list" aria-label="Attack corpus">
                  <button className={`attack-row ${selectedAttack === "silent-rollback" ? "attack-active" : ""}`} onClick={() => setSelectedAttack("silent-rollback")}>
                    <span className="attack-index">LIVE</span><span><strong>Silent rollback</strong><small>restore state 17, continue after head 19</small></span><ArrowRight size={15} />
                  </button>
                  {attackCorpus.map(([name, reason]) => (
                    <button className={`attack-row ${selectedAttack === name ? "attack-active" : ""}`} key={name} onClick={() => setSelectedAttack(name)}>
                      <span className="attack-index">CORPUS</span><span><strong>{name.replaceAll("_", " ")}</strong><small>{reason}</small></span><Check size={15} />
                    </button>
                  ))}
                </div>
                <div className={`lab-result ${rollback?.status === "REJECTED" ? "lab-result-rejected" : ""}`}>
                  {selectedAttack === "silent-rollback" && rollback?.status === "REJECTED" ? (
                    <>
                      <div className="result-stamp"><XCircle size={20} /> REJECTED</div>
                      <h3>The restored snapshot cannot replace the committed history.</h3>
                      <div className="attempt-chain"><span>STATE {transitions.length + 16} / HEAD</span><ArrowRight size={15} /><span className="attempt-old">RESTORE {initialData.fixture.attack.restoredSequence}</span><ArrowRight size={15} /><span className="attempt-bad">STATE {initialData.fixture.attack.attemptedSequence}</span></div>
                      <dl className="result-readout"><div><dt>contract result</dt><dd><code>{rollback.reason}</code></dd></div><div><dt>execution</dt><dd>{rollback.execution ?? "local EthereumJS"}</dd></div><div><dt>raw memory on-chain</dt><dd>NO</dd></div></dl>
                      <p className="result-explanation">The local memory snapshot was restored, but it cannot silently replace the previously committed history.</p>
                      <Button variant="quiet" size="small" onClick={runRollback}><RefreshCw size={15} /> Run again</Button>
                    </>
                  ) : (
                    <>
                      <div className="result-stamp result-stamp-ready"><Radio size={20} /> READY TO REHEARSE</div>
                      <h3>{selectedAttack === "silent-rollback" ? "Try the stale predecessor against the live local harness." : "This mutation is recorded in the published corpus."}</h3>
                      <p>{selectedAttack === "silent-rollback" ? "The simulation commits three canonical transitions, restores snapshot 17, then submits a correctly sequenced transition with the wrong predecessor root." : "The browser view keeps the exact contract reason visible while the full 20-case corpus remains replayable offline."}</p>
                      {selectedAttack === "silent-rollback" ? <Button variant="danger" onClick={runRollback} disabled={rollbackBusy}>{rollbackBusy ? <RefreshCw className="spin" size={16} /> : <RotateCcw size={16} />}{rollbackBusy ? "Running" : "Run Silent Rollback"}</Button> : <div className="corpus-note"><ClipboardCheck size={16} /> Recorded: {attackCorpus.find(([name]) => name === selectedAttack)?.[1]}</div>}
                    </>
                  )}
                </div>
              </div>
            </section>
          )}

          {activeTab === "verify" && (
            <section className="tab-panel" aria-labelledby="verify-title">
              <div className="section-heading"><div><h2 id="verify-title">Portable evidence</h2><p>Export the public commitments. Verify them without trusting this dashboard.</p></div><StatusBadge tone={verification?.verdict === "VERIFIED" ? "verified" : verification?.verdict === "REJECTED" ? "danger" : "neutral"}>{verification?.verdict ?? "NOT RUN"}</StatusBadge></div>
              <div className="verify-layout">
                <div className="export-panel">
                  <div className="export-mark"><Download size={20} /></div>
                  <h3>Take the evidence with you</h3>
                  <p>The export contains sequence, predecessor roots, commitments, and the observed head. It never contains raw memory.</p>
                  <div className="export-lines"><div><span>transitions</span><strong>{portableEvidence.transitions.length}</strong></div><div><span>privacy boundary</span><strong>OFF-CHAIN</strong></div><div><span>schema</span><code>v1</code></div></div>
                  <div className="export-actions"><Button variant="primary" onClick={exportEvidence}><Download size={16} /> Export evidence.json</Button><Button variant="quiet" onClick={runTamperCheck}><AlertTriangle size={16} /> Tamper one field</Button></div>
                  <p className="export-note">The independent CLI is <code>python3 verifier/verify.py memorylineage-evidence.json</code>.</p>
                </div>
                <div className="import-panel">
                  <div className="import-heading"><Upload size={18} /><div><h3>Verify a bundle</h3><p>{importedName ?? "Drop an exported JSON file here."}</p></div></div>
                  <label className="file-drop"><Upload size={20} /><strong>Import evidence JSON</strong><span>Runs schema, sequence, predecessor, head, and privacy checks.</span><input type="file" accept="application/json,.json" onChange={(event) => importEvidence(event.target.files?.[0])} /></label>
                  {verification ? <div className={`verification-result ${verification.verdict === "VERIFIED" ? "verification-pass" : "verification-fail"}`}><div className="verification-title">{verification.verdict === "VERIFIED" ? <CheckCircle2 size={18} /> : <XCircle size={18} />} {verification.verdict}{verification.reason ? ` / ${verification.reason}` : ""}</div>{verification.checks && Object.entries(verification.checks).map(([key, value]) => <div className="verification-row" key={key}><span>{key.replaceAll(/([A-Z])/g, " $1")}</span><strong>{value}</strong></div>)}{verification.finalRoot && <code className="verification-root">final root {shortHash(verification.finalRoot, 16, 12)}</code>}</div> : <div className="verification-empty"><FileCheck2 size={22} /><span>Verification results will appear here.</span></div>}
                </div>
              </div>
            </section>
          )}
        </section>
      </main>

      <footer className="footer"><span>MEMORYLINEAGE / INSPECTOR</span><span>We prove history integrity. We do not claim semantic truth.</span><a href="#workbench" aria-label="Return to the Inspector workbench"><ArrowRight size={14} /> WORKBENCH</a></footer>
    </div>
  );
}
