# ResolverCompat spike (throwaway)

Ini adalah probe feasibility yang sengaja kecil dan tidak dianggap produk final.
Ia memodelkan bentuk output resolver ERC-7683 dengan fixture JSON, menerapkan
policy lokal yang eksplisit, menghasilkan evidence hash, dan menjalankan verifier
kedua yang hanya membaca evidence.

Jalankan test:

```bash
python3 -m unittest discover -s . -p 'test_*.py' -v
```

Jalankan runner dan tulis artefak:

```bash
python3 research/spikes/run_spike.py --output-dir evidence/generated/research-spikes
```

Gate spike:

- 3 fixture dasar: valid, payment recipient mismatch, dan unknown assumption;
- 12 mutasi payload;
- setiap mutasi harus `REJECT` atau `UNVERIFIED`;
- verifier kedua harus menghasilkan verdict yang sama;
- `PASS` berarti hanya lolos policy yang dipilih, bukan jaminan keamanan ekonomi.

Artefak di `artifacts/` dapat dihapus setelah hasilnya dicatat. Belum ada
integrasi dengan kontrak atau resolver yang dideploy, fixture open-source yang
dijalankan di fork, uji pengguna, maupun pembandingan performa.

## SlippageTruth spike

Spike kedua memodelkan proposal `ERC-8377` secara deterministik. Ia membandingkan
minimum output statis dengan floor yang dihitung dari reference output, known
route cost, adverse deviation, freshness, dan saldo output yang benar-benar
diterima recipient. Ia bukan simulator MEV atau kontrak router produksi.

Jalankan test dan tulis evidence:

```bash
python3 -m unittest discover -s . -p 'test_*.py' -v
python3 research/spikes/run_slippage_truth.py --output-dir evidence/generated/research-spikes
python3 -m json.tool evidence/generated/research-spikes/slippage_truth_summary.json >/dev/null
```

Gate lokal saat ini terdiri dari lima kasus: output yang melewati floor statis
tetapi gagal floor live, output jujur yang diterima, route yang melaporkan
output tanpa saldo recipient, oracle stale, dan intent kedaluwarsa. Verifier
kedua menghitung ulang verdict dari evidence dan hash evidence harus cocok.
Kasus ini masih model lokal; belum ada router EVM, fork mainnet, pengukuran
nilai MEV, atau uji pengguna.

## MemoryLineage Auditor spike

Spike ketiga menguji kandidat `MemoryLineage Auditor / ERC-8350`: isi memori
tetap menjadi witness privat, sementara sequence, predecessor root, delta,
provenance, profile, dan locator dipresentasikan sebagai commitment. Probe
ini memverifikasi golden vector publik, membuat empat transition valid, lalu
mencoba rollback, sequence gap, parallel history, authorizer berbeda, locator
substitution, dan payload tampering. Verifier replay membaca JSON evidence dan
tidak membutuhkan memori mentah.

Jalankan:

```bash
python3 -m unittest discover -s . -p 'test_*.py' -v
PYTHONPATH=verifier/python python3 -m memory_lineage.demo --output-dir evidence/generated
python3 -m json.tool evidence/generated/memory_lineage_summary.json >/dev/null
```

Gate ini hanya membuktikan model hash dan linear-history lokal. Ia belum
membuktikan signature EOA/ERC-1271, bytecode kontrak, event indexing, transaksi
Sepolia, data availability, atau kebenaran isi memori.

## Candidate 4.7 audit loop

Ledger pencarian kandidat menyimpan rating riset, evidence class, gate aktual,
dan target conditional secara terpisah. Jalankan:

```bash
python3 research/spikes/candidate_audit_loop.py --output-dir evidence/generated/research-spikes
python3 -m json.tool evidence/generated/research-spikes/candidate_audit_loop.json >/dev/null
```

`actual_score` hanya dapat terisi jika contract/fork, verifier independen,
testnet/live flow, dan user-impact gate lulus. Hasil saat ini mencatat tidak ada
actual 4,7; MemoryLineage memiliki satu jalur conditional 4,7.

## MemoryLineage EVM conformance audit

Audit berikutnya sudah menjalankan kontrak Solidity lokal yang mengikuti batas
normatif ERC-8350 v1. Perintahnya:

```bash
node --test evm/test/memory_lineage_registry.test.mjs
node evm/scripts/audit_local.mjs
PYTHONPATH=verifier/python python3 -m memory_lineage.replay
PYTHONPATH=verifier/python python3 -m memory_lineage.impact --repeats 5
PYTHONPATH=verifier/python python3 -m memory_lineage.gates
node evm/scripts/deploy_sepolia.mjs --dry-run
# after explicit approval and local DEPLOYER_PRIVATE_KEY setup:
node evm/scripts/deploy_sepolia.mjs --confirm-public-testnet
# after an approved deployment, from a second RPC:
node evm/scripts/reread_sepolia.mjs
```

Gate lokal yang terbukti sekarang: compiler Shanghai, golden vector, empat
transition valid, EOA, ERC-1271, rotasi `configNonce`, dan 20/20 mutation
ditolak. Verifier Python menghitung ulang evidence tanpa mengimpor evaluator
JavaScript dan menghasilkan mismatch nol. Artefaknya ada di
`evidence/generated/`, `evidence/local/memory_lineage_evm_evidence.json`, dan
`evidence/generated/memory_lineage_evidence_gates.json`.

Probe dua RPC Sepolia hanya membaca deployment referensi upstream. Itu bukan
deployment workspace. `actual_score` tetap `null` sampai ada deployment
workspace yang dapat dibaca ulang melalui RPC kedua atau indexer, serta uji
developer/impact yang menyimpan waktu replay, gas, false reject, mismatch, dan
pemahaman verdict. Helper deployment fail-closed dan tidak membaca key dari
file; mode `--dry-run` tidak mengirim transaksi.
