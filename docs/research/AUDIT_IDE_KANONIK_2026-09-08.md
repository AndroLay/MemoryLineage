# Audit kandidat ide, loop ketujuh

Tanggal audit: **8 September 2026**.

Dokumen ini menggabungkan semua kandidat unik yang pernah masuk ranking pada bagian 12–21 `REKOMENDASI_PROYEK_3RD_WEB_HACK.md`. Nama yang kemudian diserap ke kandidat lain tetap dicatat agar kenaikan skor tidak dibuat dengan mengganti nama. `Skor terakhir` adalah angka terbaru sebelum loop ketujuh; `audit terbaru` adalah nilai setelah bukti loop ketujuh diperiksa.

Skala yang dipakai tetap sama:

- **1**: gagasan atau klaim belum punya bukti yang relevan.
- **2**: primitive atau produk pembanding sudah kuat, atau blocker utama belum selesai.
- **3**: masalah dan scope didukung sumber, tetapi MVP kita belum memiliki bukti independen.
- **4**: MVP kita berjalan, test deterministik dan pemeriksaan independen lulus, serta pembeda terlihat terhadap pembanding.
- **5**: MVP, pemeriksaan independen, validasi pengguna, pembandingan, dan dampak terukur tersedia tanpa gap material.

Angka ini adalah rating evidence, bukan prediksi nilai juri. Reference implementation, deployment, atau test suite milik pihak lain tidak dihitung sebagai MVP kita.

## Hasil audit seluruh kandidat

| Kandidat kanonik | Skor terakhir | Audit terbaru | Perubahan | Putusan audit |
| --- | ---: | ---: | :---: | --- |
| GrantTrail | 2,5 | 2,5 | = | Tetap fallback; overlap grant/escrow belum berubah |
| SkillProof | 2,2 | 2,2 | = | Tetap gugur; penerbitan attestation sudah memiliki primitive kuat |
| AllowanceLens | 2,2 | 2,2 | = | Tetap gugur; inti masalah sudah ditangani Revoke.cash dan wallet |
| QuietVote | 2,1 | 2,1 | = | Tetap gugur; primitive anonim Semaphore masih menutup inti capability |
| IntentWatch | 2,4 | 2,4 | = | Tetap komponen monitoring, bukan produk utama |
| AttestScope | 2,4 | 2,4 | = | Tetap fallback security-first; overlap Clear Signing tetap material |
| Generic OutcomeGuard | 2,3 | 2,3 | = | Tetap gugur; terlalu generik terhadap simulation/assertion |
| EIP-8025 execution proof | 1,9 | 1,9 | = | Tetap terlalu protocol-level untuk MVP hackathon |
| Generic agent trust/escrow | 2,1 | 2,1 | = | Tetap gugur; identity, attestation, dan escrow sudah padat |
| 7702InitLock | 2,3 | 2,3 | = | Tetap fixture pembanding; capability inti sudah beririsan dengan tooling |
| 7702DelegateDiff | 2,6 | 2,6 | = | Tetap candidate security dengan uniqueness rendah |
| 7702Rescue | 2,3 | 2,3 | = | Tetap gugur; revoke flow sudah tersedia |
| PermissionScope Inspector | 2,4 | 2,4 | = | Tetap prototype layak dengan overlap MetaMask Permissions |
| PermissionRevoke Calendar | 2,3 | 2,3 | = | Tetap gugur sementara; expiry/revoke bukan capability baru |
| WalletCapsProbe | 2,3 | 2,3 | = | Tetap conformance tester generik |
| BatchReceipt | 2,3 | 2,3 | = | Tetap feasible, kebaruan rendah |
| IntentAssumptionLens | 2,6 | 2,6 | = | Diserap ke `ResolverCompat`; tidak diberi skor terpisah baru |
| SolverRiskReceipt | 2,5 | 2,5 | = | Tetap memerlukan model lintas protokol yang belum ada |
| PostconditionLab | 2,4 | 2,4 | = | Tetap overlap simulation/assertion |
| TypedSignFirewall | 2,5 | 2,5 | = | Tetap berdekatan dengan Clear Signing dan wallet renderer |
| ModuleScopeAudit | 2,5 | 2,5 | = | Tetap scope scanner yang terlalu ramai |
| RecoveryDrill | 2,5 | 2,5 | = | Tetap butuh integrasi account nyata sebelum bisa naik |
| GovernancePayloadDiff | 2,4 | 2,4 | = | Tetap beririsan dengan Tally dan OpenZeppelin |
| VerifiedRPCLens | 2,3 | 2,3 | = | Tetap terlalu besar untuk MVP sempit |
| ReceiptProofPack | 2,2 | 2,2 | = | Tetap komponen dasar dengan format proof yang ramai |
| FrontendIntegrityStamp | 2,3 | 2,3 | = | Tetap overlap verifikasi frontend dan asset commitment |
| AttestationFreshnessInbox | 2,3 | 2,3 | = | Tetap overlap registry/workflow attestation |
| PrivateCredentialGate | 2,2 | 2,2 | = | Tetap memakai primitive Semaphore/EAS yang sudah matang |
| AgentActionPermit | 2,3 | 2,3 | = | Tetap overlap Proof of Claw, ERC-8273, dan ERC-8004 |
| GrantEvidenceEscrow | 2,2 | 2,2 | = | Tetap gugur; ERC-8183, Allo, dan Milestack dekat |
| ERC8203 SettlementProbe | 2,5 | 2,5 | = | Tetap gugur sebagai produk mandiri karena reference surface sudah ada |
| OnchainUI Safety Lens | 2,4 | 2,4 | = | Tetap ditahan; sandbox dan onchain UI security belum stabil |
| 7702History Forensics | 2,5 | 2,5 | = | Tetap gugur; WalletCheck dan explorer menutup alur inti |
| ResolverCompat / FillerSafety Lab | 3,0 | 3,0 | = | Spike lokal tetap valid, tetapi belum ada fixture external dan user evidence |
| SlippageTruth Lab | 3,6 | 3,6 | = | Local arithmetic evidence tetap sama; EVM/fork belum dijalankan |
| RWA StatusGuard | 3,7 | 3,7 | = | Draft/status adapter dan atomic re-check belum menghasilkan bukti MVP baru |
| HiddenRefs Explorer | 3,7 | 3,7 | = | Commitment/reveal wedge tetap menarik, tetapi belum ada contract/user proof |
| SafeReceive | 3,5 | 3,5 | = | Pain kuat, overlap recipient protection tetap menahan uniqueness |
| ConsentMesh | 3,5 | 3,5 | = | Blocker PII, usage accounting, dan caller authentication tetap terbuka |
| PrivateSpendPolicy | 3,4 | 3,4 | = | Groth16 reference tidak menjadi implementasi kita; payment products tetap dekat |
| LaunchGuard | 3,3 | 3,3 | = | Reference fixes belum menggantikan labeled corpus dan false-positive test |
| LeaseState Lens | 3,4 | 3,4 | = | Reference tests memperjelas feasibility, tetapi product wedge belum naik |
| MandateConservation Auditor | 3,2 | 3,2 | = | Bounded mandate overlap dan fork/property test belum ada |
| ReceiptReplay | 3,3 | 3,3 | = | Receipt standard dan evidence layer tetap terlalu generik |
| PlanSeal / PlanSeal Conformance Lab | 3,3 | **3,8** | **↑ 0,5** | Naik karena ERC-8410 PR memberi schema, empat digest vector, Rust/JS cross-check, dan reference use; tetap bukan MVP kita |
| Deactivation Exit Radar | 3,4 | 3,4 | = | Semantics tetap niche dan belum ada fork fixture |
| CloneReplay Guard | 3,2 | 3,2 | = | Reference/adversarial evidence tidak menyelesaikan latency/race/clone limits |
| FAT Investor Console | 3,2 | 3,2 | = | Economics, redemption, dan soundness tetap terlalu terbuka |
| Warden Compromise Lab | 3,8 | 3,8 | = | Bukti ERC-8238 memperkuat feasibility, tetapi direct reference demo menekan uniqueness; belum ada run kita |
| RWA DisclosureLens | 3,7 | 3,7 | = | Proposal ABI/conformance belum final dan analytics overlap tetap ada |
| WYRIWE Agent Provenance Gate | 3,7 | 3,7 | = | Commitment stack berdekatan; belum ada consumer workflow kita |
| AgentBinding OrphanRadar | 3,5 | 3,5 | = | Standard dan adapters sudah menutup sebagian besar capability |
| SOAC PrivilegeOps Drill | 3,4 | 3,4 | = | Reference implementation belum siap dan control-plane tooling sudah ramai |
| PrivacyToken Interop Lab | 3,3 | 3,3 | = | Cryptography/circuit/wallet dependency tetap load-bearing |
| MemoryRights Capsule | 3,4 | 3,4 | = | PII, deletion, availability, dan legal semantics belum diuji |
| RWA AgencyGuard | 3,4 | **3,3** | **↓ 0,1** | Turun; ERC-8312 dan ERC-8226 memperjelas bahwa metering/mandate control sudah berdekatan dan scope terpisah menggandakan control plane |
| AgentRisk Passport | 3,2 | 3,2 | = | Discovery/reputation/risk products tetap overlap langsung |
| ManifestTruth / PIM Conformance Lab | baru | **3,8** | baru | Candidate baru; PIM memberi problem surface jelas, tetapi belum ada public reference implementation setara |
| CoercionProof Scenario Lab | baru | **3,8** | baru | Candidate baru; external 71/71, fork, deployment, dan demo kuat, tetapi reference product/Safe/Cedar/Edge menekan uniqueness |
| BoundedSpend Cursor Lab | baru | **3,5** | baru | Candidate baru; proposal sendiri memisahkan counting dari enforcement dan ERC-8226 overlap langsung |
| SettlementLock Explorer | baru | **3,4** | baru | Candidate baru; lifecycle lock jelas, tetapi draft dan semantics implementer-defined |
| SkillPackage Integrity Lab | baru | **3,6** | baru | Candidate baru; ERC-8239/asrpm punya registry, checksum, dan Sepolia evidence, tetapi behavior quality tetap di luar integrity |
| QuoteTruth / Signed Quote Audit | baru | **3,2** | baru | Candidate baru; ERC-8409 masih draft sangat baru tanpa reference evidence yang sebanding |

## Kenapa hanya satu kandidat naik rata-ratanya

`PlanSeal` menerima bukti baru yang mengubah dua hal sekaligus: feasibility eksternal menjadi lebih nyata dan design artifact menjadi lebih spesifik. Pull request ERC-8410 menyebut schema JSON, aturan canonicalization, empat test vector, satu vector yang dicross-check antara Rust dan JavaScript, serta reference use di Ekubo Wallet. Itu cukup untuk menaikkan rating evidence kita dari 3,3 menjadi 3,8, tetapi tidak cukup untuk memberi rating 4 karena artefak tersebut belum dibuat dan dijalankan oleh workspace ini.

`Warden Compromise Lab` tidak dinaikkan meskipun ERC-8238 lebih kuat daripada source yang dipakai sebelumnya. Bukti 71/71, fork Sepolia, deployment, dan demo adalah bukti reference project. Pada saat yang sama, reference project tersebut sudah melakukan showcase capability yang sangat dekat dengan ide kita, dan Safe, Cedar, serta Edge menempati ruang wallet protection yang berdekatan. Kenaikan feasibility tertutup oleh penurunan uniqueness; rata-ratanya tetap 3,8.

`RWA AgencyGuard` diturunkan karena loop baru menegaskan bahwa metering bounded-agent dan regulated mandate sudah memiliki permukaan yang berdekatan. Menambah satu dashboard policy di atas dua primitive tersebut tidak cukup untuk mempertahankan nilai 3,4 sebagai produk terpisah. Fungsinya lebih baik diserap ke `RWA StatusGuard` bila suatu saat adapter atomic benar-benar dibangun.

## Target bersyarat yang memenuhi ambang 4,7

Target berikut hanya berlaku setelah semua gerbang bukti lulus. Nilainya dihitung dari target dimensi, bukan dinaikkan secara naratif:

| Kandidat | I target | F target | U target | D target | Rata-rata target | Syarat utama |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| **PlanSeal Conformance Lab** | 4,7 | 4,8 | 4,6 | 4,8 | **4,725 → 4,7** | MVP sendiri, vector resmi, verifier kedua lintas implementasi, 30 mutation, dua wallet/provider, testnet, competitor matrix, user test, dan metrik false reject/overhead |
| **ManifestTruth / PIM Conformance Lab** | 4,7 | 4,7 | 4,6 | 4,8 | **4,700 → 4,7** | Dua manifest protokol, simulator/ABI trace, mutation corpus, independent verifier, wallet/testnet flow, comparison dengan ERC-7730/EIP-5792, dan developer test |

Kedua baris di atas adalah **target bersyarat**, bukan skor sekarang. Bila satu gerbang wajib gagal, target harus diturunkan atau kandidat dihentikan. Tidak ada kandidat yang saat ini terbukti memiliki rata-rata 4,7.

## Loop lanjutan: audit kandidat MemoryLineage / ERC-8350

| Kandidat | Skor evidence saat ini | Target bersyarat | Status audit aktual | Bukti yang dapat diperiksa |
| --- | ---: | ---: | --- | --- |
| **MemoryLineage Auditor / ERC-8350** | **4,3** | **4,7** | Kandidat penantang terkuat dari riset baru; target 4,7 belum aktual; 4,3 adalah koreksi pembulatan research rating | ERC-8350 draft/repository/vector, sumber OWASP, spike lokal 18/18, 7 kasus, 6 mutation non-pass, replay mismatch 0 |
| PlanSeal / PlanSeal Conformance Lab | 3,8 | 4,7 | Research rating; belum ada MVP lokal | ERC-8410 PR, vector eksternal, cross-check Rust/JS dan Ekubo Wallet; artefak bukan milik workspace |
| ManifestTruth / PIM Conformance Lab | 3,8 | 4,7 | Research rating; belum ada MVP lokal | ERC-8313 PR/discussion dan sumber pendukung; belum ada implementation, verifier, atau testnet kita |

### Audit klaim versus aktual untuk MemoryLineage

| Klaim | Yang benar-benar terbukti | Yang belum terbukti |
| --- | --- | --- |
| “ERC-8350 dapat diaudit secara deterministik” | Model lokal mencocokkan vector yang dipublikasikan dan mendeteksi enam kelas mutasi | Kesetaraan dengan Solidity, ABI/event, dan verifier eksternal independen |
| “Memory poisoning dapat didemokan” | Spike menolak rollback, sequence gap, fork, authorizer salah, locator substitution, dan payload tamper pada commitment/history model | Serangan pada agent memory nyata, kualitas deteksi semantik, dan pemulihan operasional |
| “Skornya 4,7” | 4,7 hanya target bersyarat setelah semua gate | Tidak ada score aktual 4,7; skor riset sekarang 4,3 |
| “Blockchain menyelesaikan keamanan memori” | Commitment dan predecessor/sequence checks memberi audit continuity | Tidak membuktikan isi memori benar, tersedia, privat, atau bebas poisoning |

Skor 4,3 dihitung sebagai skor riset berbasis bukti yang sudah tersedia, bukan prediksi juri. Agar naik ke 4,7, semua gate dalam `DEEP_RESEARCH_2026-09-08.md` dan `REKOMENDASI_PROYEK_3RD_WEB_HACK.md` harus lulus dengan output yang disimpan. Jika salah satu gate wajib gagal, rating harus tetap 4,3 atau turun; tidak boleh dinaikkan dari narasi.

## Audit nilai ideal terbaru

Angka 4,2 pada snapshot sebelumnya dikoreksi setelah nilai dimensi disimpan sampai dua desimal dan dibulatkan dengan aturan half-up. Rincian ada di [AUDIT_NILAI_IDEAL_2026-09-08.md](<AUDIT_NILAI_IDEAL_2026-09-08.md:1>).

| Kandidat | Sebelumnya | Audit terbaru | Perubahan | Keterangan |
| --- | ---: | ---: | :---: | --- |
| MemoryLineage Auditor / ERC-8350 | 4,2 | **4,3** | **↑ 0,1** | Koreksi 4,25 → 4,3; bukti tetap research-backed, belum MVP |
| Execution Evidence Chain / ERC-8301 | 3,7 | **3,8** | **↑ 0,1** | Koreksi 3,75 → 3,8; tidak ada implementasi baru |
| Semua kandidat lain | nilai terakhir | nilai terakhir | = | Tidak ada sumber atau artefak baru yang mengubah dimensi |

Batasnya tetap sama: tidak ada score aktual 4,7.

## Tambahan audit: loop benchmark dan memory rights

Loop berikutnya tidak mengubah ranking kanonik. FlexGov, KSwap-VM, Doca, Assay, dan Commitment Issues ditambahkan sebagai benchmark existing-project karena halaman publiknya mengungkapkan demo, source, tests, atau live flow yang relevan. MemoryRights / ERC-8264 + Capsule ditambahkan sebagai challenger source-backed dengan rating riset 4,1; reference repository melaporkan 21/21 Foundry tests dan deployment testnet, tetapi bukti itu tetap eksternal dan sangat dekat dengan produk yang akan dibangun.

MemoryLineage tetap 4,3. Ia tetap memiliki bukti lokal paling relevan: golden-vector compatibility, enam mutation non-pass, dan replay mismatch nol. Tidak ada angka yang dinaikkan akibat deployment atau test count pihak lain. Ledger dan perhitungan dapat diaudit di [AUDIT_LOOP_4_7_2026-09-08.md](<AUDIT_LOOP_4_7_2026-09-08.md:1>) dan [candidate_audit_loop.json](<../../evidence/generated/research-spikes/candidate_audit_loop.json:1>).

### Addendum setelah conformance EVM

Audit lokal terbaru menambahkan bytecode Solidity yang benar-benar dijalankan,
EOA/ERC-1271 authorization, rotasi authorizer, 20 mutation, dan verifier Python
independen. Ringkasan mesin kini mencatat gate `contract_or_fork=true` dan
`independent_verifier=true`; gate testnet dan user-impact masih `false`.
Karena itu rating riset tetap 4,3, target bersyarat tetap 4,7, dan
`actual_score` tetap `null`.
