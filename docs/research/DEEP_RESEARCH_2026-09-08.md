# Deep research kandidat baru — 8 September 2026

## Putusan riset

Kandidat baru yang paling menjanjikan setelah audit berulang adalah **MemoryLineage Auditor**, sebuah audit lab untuk sejarah state memori agen berbasis ERC-8350. Produk ini tidak menyimpan isi memori di blockchain. Ia menampilkan apakah setiap perubahan state memiliki urutan, predecessor root, authorizer, provenance commitment, profile, dan locator commitment yang konsisten. Demonstrasi utamanya adalah memutar ulang sejarah yang valid lalu memasukkan rollback, sequence gap, parallel history, authorizer berbeda, locator substitution, dan payload tampering.

Riset ini belum membuktikan skor MVP 4,7. Setelah audit pembulatan dan pemeriksaan sumber terbaru, rating riset kandidat ini adalah **4,3/5** (sebelumnya 4,2; rata-rata dimensinya tepat 4,25). Spike lokalnya membuktikan subset kriptografi dan linear-history. Target bersyarat **4,7** hanya boleh dipertahankan bila semua gerbang EVM, independent verifier, testnet, mutation corpus, dan uji pengguna lulus.

## Mengapa masalahnya nyata

OWASP menempatkan Memory & Context Poisoning sebagai ASI06 untuk aplikasi agentic. Risiko utamanya adalah data yang masuk sekali kemudian ikut dipakai lintas sesi, sehingga pengaruh penyerang bertahan lebih lama daripada prompt awal. OWASP juga memiliki Agent Memory Guard yang sudah menangani baseline hash, perubahan protected key, anomaly, snapshot, dan rollback. Ini memvalidasi masalahnya sekaligus memperlihatkan bahwa produk kita harus memberi pembeda onchain yang konkret, bukan hanya menyalin middleware deteksi memori.

Rujukan:

- [OWASP: Memory Is a Feature. It Is Also an Attack Surface](https://genai.owasp.org/2026/05/13/memory-is-a-feature-it-is-also-an-attack-surface/)
- [OWASP ASI06: Memory & Context Poisoning](https://genai.owasp.org/download/52117/?tmstv=1765059207)
- [OWASP Agent Memory Guard](https://owasp.org/www-project-agent-memory-guard/)

## Temuan primer ERC-8350

ERC-8350 mendefinisikan state machine linear untuk Memory Space. Transition memiliki tujuh field tetap: spaceId, sequence, prevStateRoot, deltaCommitment, provenanceCommitment, profileId, dan locatorCommitment. Transition ID memakai satu EIP-712 type string; root berikutnya dihitung kontrak dari predecessor root dan transition ID. Registry menolak sequence yang bukan currentSequence + 1 atau predecessor yang bukan current root. Dengan demikian, kasus rollback, lompatan, dan sejarah paralel menjadi failure case yang dapat dijelaskan, bukan sekadar label risiko.

Proposal juga menetapkan batas yang harus dipertahankan dalam produk: valid transition hanya membuktikan bahwa authority yang dikonfigurasi menyetujui commitment dalam urutan yang benar. Ia tidak membuktikan isi memori benar, tersedia, kepemilikan data, kebenaran inference, atau penghapusan data offchain.

Status dan evidence yang dapat diperiksa:

- [ERC-8350 discussion](https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098): draft, public Sepolia deployment, golden-vector details, dan non-claims.
- [Reference repository](https://github.com/AwareLiquid/ERC-8350): Solidity registry, dua implementasi TypeScript yang dependency-isolated, golden vector, fixture Space, dan perintah pnpm check.
- [Golden vector](https://github.com/AwareLiquid/ERC-8350/blob/main/test-vectors/v1.json): typehash, spaceId, transitionId, dan nextStateRoot yang menjadi fixture spike.
- [Official ERC PR #1910](https://github.com/ethereum/ERCs/pull/1910): proposal menerima editor review dan masih berlabel Draft; halaman mencatat perbaikan immutability dan status checks yang harus diverifikasi ulang sebelum dianggap final.
- [Threat model](https://github.com/AwareLiquid/ERC-8350/blob/main/docs/threat-model.md): memisahkan ancaman namespace, append, relayer locator, upgradeability, dan availability witness.

Bukti ini lebih kuat daripada proposal yang hanya memiliki prose, tetapi belum setara dengan standar final atau audit eksternal independen. Repository sendiri menyatakan bahwa dua implementasi berada dalam satu proyek dan masih membutuhkan implementasi dari tim lain serta audit keamanan.

## Spike lokal yang benar-benar dijalankan

Probe throwaway di [verifier/python/memory_lineage/model.py](../../verifier/python/memory_lineage/model.py:1) dan [verifier/python/memory_lineage/demo.py](../../verifier/python/memory_lineage/demo.py:1) melakukan hal berikut:

- menghitung Keccak-256 tanpa dependency tambahan;
- mencocokkan spaceId, tiga typehash, transitionId, dan nextStateRoot dengan golden vector publik;
- membuat empat transition lokal yang valid;
- menolak enam mutasi: rollback, sequence gap, parallel history, authorizer berbeda, locator substitution, dan payload tampering;
- menulis evidence JSON dan menjalankan verifier replay dari evidence;
- menghasilkan replay_mismatches = 0.

Artefak dapat diperiksa di [memory_lineage_summary.json](../../evidence/generated/memory_lineage_summary.json:1) dan [memory_lineage_evidence.json](../../evidence/generated/research-spikes/memory_lineage_evidence.json:1).

Batas aktualnya tetap tegas: belum ada Solidity, signature EOA/ERC-1271, event indexing, bytecode immutability check, transaksi Sepolia milik proyek, data-availability test, atau semantic proof bahwa payload memori benar.

## Audit overlap dan kandidat pembanding

| Kandidat | Evidence primer | Kelebihan | Blocker yang menahan skor | Rating riset | Putusan |
| --- | --- | --- | --- | ---: | --- |
| MemoryLineage Auditor / ERC-8350 | Draft dengan reference repo, golden vector, Sepolia registry, external recompute report, dan spike lokal | Failure case linear state mudah divisualkan; isi tetap privat; Web3 menjadi enforcement boundary | Proposal masih Draft; dua implementation masih satu tim; pesaing offchain sudah menangani hash-chain/rollback; registry dan verifier kita belum ada | 4,3 | Challenger terkuat; lanjut ke EVM probe |
| PlanSeal Conformance Lab / ERC-8410 | PR resmi dengan schema dan digest vector, tetapi Open/Draft dan belum dijalankan workspace | Alur artifact, relay, wallet, dan broadcast mudah dipahami | Overlap dengan EIP-5792, clear-signing, dan Ekubo; belum ada local artifact, wallet adapter, atau testnet flow | 3,8 | Tetap fallback conformance |
| CAPV PolicyProof Lab / ERC-8354 | EIP resmi Draft, reference claims, policy commitment, dan reported production recompute | Privasi policy dan pre-execution gate sangat inovatif | ZK proving system berat; proof membuktikan interpreter berjalan, bukan policy benar; ruleset membuat external recompute CANNOT_RECOMPUTE; LLM judgment tetap tidak deterministik | 3,9 | Research-only challenger |
| OCP OutcomeProof / ERC-8281 | Reference CLI, Base Sepolia contract, cross-chain path, conformance 11/11 reported | Evidence onchain dan verifier sangat konkret | Primitive sudah generik; overlap langsung dengan ReceiptReplay/ReceiptOS; produk baru perlu use case sempit | 3,8 | Diserap sebagai evidence layer |
| NAVFreshness Guard / ERC-8330 | EIP dalam Review dengan correction, invalidation, dual staleness, aggregation, dan test-case list | Interface RWA lebih matang daripada proposal awal; failure state jelas | Provider, methodology, backing, dan redemption tetap di luar standar; implementasi kita belum ada | 3,7 | RWA fallback |
| CoercionProof Scenario Lab / ERC-8238 | Reference project melaporkan 71/71, fork, deployment, dan demo | Demo serangan dan policy vault mudah dilihat juri | Reference product, Safe limits, Cedar, dan Edge mengurangi uniqueness; belum ada independent implementation kita | 3,8 | Challenger, bukan pilihan utama |
| Execution Evidence Chain / ERC-8301 | Draft aktif, 67 replies, workflow/FSM direction, trustless-ai SDK | Bisa menjadi execution envelope lintas agent verifier | Scope mudah melebar menjadi framework; banyak layer sudah dipisah ERC-8004/8274/8281/8299; tidak ada spike lokal | 3,8 | Ditahan; angka naik hanya karena pembulatan konsisten |

OCP memiliki evidence teknis eksternal paling lengkap. Reference repo menyebut contract Base Sepolia, CLI tanpa dependency, revocation extension, dan conformance suite 11/11. Namun itu juga membuat ruang produk generik lebih sempit: produk kita harus mengalahkan primitive yang sudah hidup, bukan hanya mengulang hash-versus-tamper demo.

CAPV memiliki upside besar, tetapi EIP-nya sendiri menyatakan bahwa verdict hanya membuktikan committed interpreter mengevaluasi action dan mengembalikan ALLOW; ia tidak membuktikan policy benar atau interpreter setia pada maksud operator. Batas ini membuat demo audit fidelity jauh lebih berat daripada MemoryLineage.

Rujukan pembanding:

- [OCP repository dan live reference](https://github.com/damonzwicker/observation-commitment-protocol)
- [ERC-8404 RVR discussion](https://ethereum-magicians.org/t/erc-8404-recomputable-verification-receipts/29521)
- [ERC-8354 official EIP](https://eips.ethereum.org/EIPS/eip-8354)
- [ERC-8330 official EIP](https://eips.ethereum.org/EIPS/eip-8330)
- [ERC-8301 discussion](https://ethereum-magicians.org/t/erc-8301-ai-agent-execution/28785)

## Produk yang sebaiknya dikerjakan

Nama kerja: **MemoryLineage Auditor**.

Pengguna pertama adalah operator atau developer agent yang perlu menjawab: “Apakah state yang digunakan agent sekarang berasal dari sejarah yang sah, atau telah di-rollback, disisipi, dilompati, atau diarahkan ke locator lain?”

MVP yang sempit:

1. Buat satu Memory Space dan tampilkan controller, authorizer, profile, serta batas privasinya.
2. Tulis tiga sampai empat transition ke kontrak yang tidak upgradeable pada satu testnet.
3. Tampilkan timeline state root dan evidence tanpa menampilkan raw memory.
4. Sediakan Attack Lab untuk enam mutasi yang sudah terbukti pada spike lokal.
5. Ekspor evidence JSON; halaman verifier kedua membaca export tersebut tanpa memanggil evaluator utama.
6. Tampilkan batas: “integrity dan urutan commitment terverifikasi; kebenaran isi memori belum terverifikasi.”

Jangan menambahkan vector database, marketplace memory, tokenisasi, LLM judge, ZK proof, multi-chain, atau recovery otomatis sebelum core gate lulus. Fitur tersebut mengubah produk menjadi platform memori dan menghilangkan failure case yang membuatnya menarik.

## Gerbang agar target 4,7 boleh dipertahankan

Target ini adalah proyeksi bersyarat, bukan hasil saat ini. Formula target yang diizinkan: Innovation 4,8 + Technical Feasibility 4,6 + Uniqueness 4,6 + Design 4,8 = 18,8 / 4 = **4,7**.

| Gate | Evidence wajib | Stop condition |
| --- | --- | --- |
| Exact conformance | Implementasi Solidity kita dan verifier kedua menghitung semua golden vector dari type string, bukan konstanta yang disalin | Satu vector berbeda atau verifier kedua memakai fungsi hash utama |
| Contract boundary | Deployment testnet, ABI, event/state replay, dan pemeriksaan enforcing logic tidak dapat diganti melalui proxy/admin | Hanya mock lokal, contract upgradeable tanpa guard, atau history dibaca dari UI saja |
| Authority | EOA serta ERC-1271 fixture; signature mencakup semua field termasuk locator dan config nonce | Locator dapat diganti relayer atau signature tidak mengikat predecessor/sequence |
| Mutation corpus | Minimal 20 mutation: rollback, gap, branch, duplicate, signer, chain, profile, delta, provenance, locator, replay, root, proxy, dan missing witness | Ada mutation yang lolos atau status hanya unsafe tanpa failure code |
| Independent replay | Python dan TypeScript membaca evidence export secara terpisah; hasil dan final root sama | Replay memanggil evaluator utama atau membutuhkan raw memory/server |
| Poisoning demo | Satu fixture memory poisoning dengan before/after dan quarantine result; ukur false reject pada sejarah valid | Demo hanya mengganti teks lalu menyatakan aman tanpa baseline dan batas |
| User/competitor evidence | Dua developer menjalankan flow, satu membandingkan Git/hash-chain/OWASP Guard, dan mismatch, latency, serta tx cost dicatat | Tidak ada pembeda yang dipahami atau produk hanya menjadi timeline |

Jika gate Exact conformance, Contract boundary, atau Independent replay gagal, MemoryLineage turun menjadi spike riset dan keputusan kembali ke PlanSeal atau SlippageTruth sesuai toolchain. Jika gate competitor menunjukkan bahwa pengguna hanya membutuhkan OWASP Agent Memory Guard tanpa public settlement/audit, kandidat tidak boleh dipilih walaupun test kontraknya lulus.

## Keputusan akhir loop

Deep research ini mengubah urutan kerja menjadi:

1. **MemoryLineage Auditor** — upside tertinggi yang sekarang sudah memiliki spike lokal dan problem evidence eksternal; rating riset 4,3; lanjutkan hanya ke EVM conformance probe.
2. **PlanSeal Conformance Lab** — fallback dengan rating riset eksternal 3,8; belum aktual di workspace.
3. **SlippageTruth Lab** — bukti lokal paling matang saat ini, tetapi domainnya tetap arithmetic/local sampai fork EVM lulus.
4. **CAPV PolicyProof Lab** — simpan sebagai research track karena ZK/fidelity terlalu berat untuk milestone pertama.
5. **OCP/ReceiptReplay** — gunakan sebagai pola evidence atau integrasi, bukan produk generik baru.

Tidak ada kandidat yang saat ini boleh disebut skor aktual 4,7 atau hampir 5. Kandidat baru hanya memberi **jalur conditional 4,7 yang lebih terukur** daripada PlanSeal; nilai aktual yang dapat dipertanggungjawabkan masih dibatasi oleh spike lokal dan belum mencakup kontrak/testnet.

## Eleventh-loop adversarial search: external benchmarks and memory rights

The next search sweep checked recent ETHGlobal projects and new agent standards against the MemoryLineage wedge. It found stronger **external benchmarks**, not a new 4.7 candidate for this workspace.

| Candidate or benchmark | Evidence found | Audit result |
| --- | --- | --- |
| FlexGov | Live demo and source page describe deterministic governance reports, Graph-backed pagination, canonical hashes, six DAO adapters, and 34 engine/tooling tests | Strong external implementation, but the core idea is already built; uniqueness for a new submission is capped |
| KSwap-VM | Live demo and public repository describe bytecode-level proofs, a K-semantics layer, negative controls, and 281 properties; the repository also declares 20/52 opcodes modelled and some layer-two theorems `ADMITTED` | High proof discipline, but direct overlap and explicit semantic coverage gaps prevent a 4.7 product claim |
| Doca | Public page reports Base mainnet contracts, 11 Hardhat tests, paired control measurements, and a known single-fill limitation | Measurable DeFi benchmark; it is already an existing project and is not our evidence |
| Assay | Public page reports live ENS, Hedera, Graph queries, challenge/slash flow, and explicit gaps around atomic settlement and amount/memo checking | Strong demonstration benchmark; exact agent-reputation/payment surface is already occupied |
| Commitment Issues | Public page describes a real SSH-agent proxy, World ID selfie-gated commit signing, signal-bound expiry, and a remaining diff-display hole | Strong human-accountability benchmark; dependency and overlap reduce suitability as a fresh 4.7 candidate |
| MemoryRights / ERC-8264 plus Capsule | The draft defines four memory-rights functions, while the reference repository reports 21/21 Foundry tests, EVM testnet deployments, Bitcoin/Solana anchors, and self-tests | Best new challenger at 4.1 research rating; reference implementation is already close to the product, so it is a comparator/integration target |
| ERC-8263, ERC-8273, ERC-8257 | Public drafts and references cover inference anchoring, per-operation attestation, and tool registration/predicate access | Useful composition layers; generic proof/registry products have direct overlap |

The source pages are [FlexGov](https://ethglobal.com/showcase/flexgov-ooe2m), [KSwap-VM](https://ethglobal.com/showcase/kswap-vm-aix5n), [KSwap-VM repository](https://github.com/vovunku/swap-vm-verified), [Doca](https://ethglobal.com/showcase/doca-finance-rjm24), [Assay](https://ethglobal.com/showcase/assay-26egq), [Commitment Issues](https://ethglobal.com/showcase/commitment-issues-y1t2h), [ERC-8264](https://ethereum-magicians.org/t/erc-8264-ai-agent-memory-access-rights/28584), and the [rmem-gateway reference](https://github.com/clavote-boop/rmem-gateway).

The local loop ledger is [AUDIT_LOOP_4_7_2026-09-08.md](AUDIT_LOOP_4_7_2026-09-08.md:1), with machine-readable output at [candidate_audit_loop.json](../../evidence/generated/research-spikes/candidate_audit_loop.json:1). It records **three search rounds**, no actual 4.7 hit, and one conditional 4.7 path for MemoryLineage. The ledger marks `actual_score` as null where the contract, independent verifier, live/testnet flow, or user-impact gate is missing.

This sweep therefore does not change the selected implementation priority. MemoryLineage remains the best bounded next probe because its workspace spike is the only candidate in this group with a local mutation/replay artifact and it has a distinct state-continuity failure story. Its number remains a 4.3 research rating, not an actual product score.

### EVM conformance follow-up

The next probe is now complete locally. Solidity bytecode matches the ERC-8350
vector and domain, four transitions replay, EOA and ERC-1271 authorization plus
rotation pass, and 20/20 mutations reject. A separate Python verifier reports
zero replay mismatches. This closes the local contract and verifier gates while
leaving public testnet deployment, second-RPC/indexer reread, and measured
developer impact open. The research rating remains 4.3; the 4.7 figure remains
conditional and is not an actual score.
