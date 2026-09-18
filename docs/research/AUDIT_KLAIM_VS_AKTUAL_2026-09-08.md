# Audit klaim versus bukti aktual

Tanggal audit: **8 September 2026**.

Audit ini menjawab pertanyaan yang berbeda dari ranking ide: apakah suatu angka atau hasil benar-benar sudah dijalankan oleh workspace ini, hanya dibuktikan oleh sumber eksternal, atau masih merupakan target bersyarat.

## Putusan

**Target bersyarat 4,7 belum aktual.** Tidak ada MVP PlanSeal atau ManifestTruth di workspace, tidak ada vector ERC-8410 yang dijalankan, tidak ada adapter wallet, transaksi testnet, verifier kedua untuk dua kandidat tersebut, atau user test.

`PlanSeal 3,8` dan `ManifestTruth 3,8` harus dibaca sebagai **rating riset source-backed**, bukan skor produk yang sudah terbukti. `CoercionProof` dan Warden juga hanya memiliki bukti reference project eksternal. Angka-angka tersebut tidak boleh dipakai dalam submission sebagai “hasil pengujian kita”.

## Matriks status bukti

| Klaim/hasil | Status audit | Bukti yang benar-benar ada | Batas yang masih berlaku |
| --- | --- | --- | --- |
| `PlanSeal` memiliki schema, digest vector, cross-check Rust–JavaScript, dan reference use | **Eksternal, sebagian author-reported** | PR ERC-8410 terbuka menjelaskan schema, empat vector, cross-check, dan Ekubo Wallet; halaman PR berstatus Open/Draft, meminta review, dan mencatat commit dengan error CI | File vector/schema belum dijalankan oleh workspace; raw file tidak berhasil diambil ulang dalam audit ini; tidak ada implementasi PlanSeal lokal |
| `PlanSeal` aktual 3,8 | **Tidak terbukti sebagai skor produk** | 3,8 adalah rating analitis yang ditulis di dokumen berdasarkan PR eksternal | Harus ditulis sebagai `research rating 3,8`, bukan `MVP score 3,8` |
| `PlanSeal` target 4,7 | **Target saja** | Perhitungan target 4,725 dari empat dimensi dan daftar gate | Tidak satu pun gate MVP, 30 mutation, wallet/testnet, user test, dan impact metric lulus |
| `ManifestTruth` aktual 3,8 | **Tidak terbukti** | PR ERC-8313 mengonfirmasi proposal PIM, layout JSON, dan scope; PR masih Open/Draft, meminta review, dan mencatat commit dengan error | Tidak ada conformance lab lokal, dua manifest runnable, simulator, atau verifier independen |
| `ManifestTruth` target 4,7 | **Target saja** | Perhitungan target 4,700 dari empat dimensi | Belum ada artefak yang membuat target bisa diuji |
| `CoercionProof` reference 71/71, fork, deployment, demo | **Klaim eksternal yang dapat ditelusuri** | Thread ERC-8238 memuat laporan 71/71, fork Sepolia, deployment, live execution, dan demo | Workspace tidak menjalankan repository, tidak mengulang test suite, dan tidak melakukan audit independen; bukti itu tidak menjadi MVP kita |
| `Warden 3,8` | **Rating eksternal, bukan hasil lokal** | Reference repository dan PR standar menjadi dasar desain | Tidak ada Forge/Hardhat run, fuzz, fork, deployment, atau verifier di workspace |
| `ResolverCompat` 3,0 | **Aktual secara lokal, scope terbatas** | 7 test ResolverCompat lulus sebagai bagian dari total suite `18/18`; 3 fixture; 12 mutation; 15 evidence replay; `replay_mismatches=0`; tamper check tersedia | Semua fixture lokal/mock; belum ada resolver open-source di fork, user test, deployment, atau independent external implementation |
| `SlippageTruth` 3,6 | **Aktual sebagai spike lokal** | 5 kasus dijalankan; `replay_mismatches=0`; static floor menerima fixture adverse sementara live floor menolak; stale oracle dan expired intent ditolak | Ini arithmetic/local model; belum EVM, router, fork, oracle nyata, MEV measurement, deployment, atau user test |
| Audit daftar kandidat | **Aktual sebagai audit dokumen** | 57 baris kandidat historis tercatat, lalu MemoryLineage ditambahkan sebagai addendum dengan status evidence dan gate terpisah | Coverage pencarian web tidak membuktikan semua kandidat ekosistem telah ditemukan |

## Pemeriksaan lokal yang dijalankan

| Pemeriksaan | Hasil |
| --- | --- |
| Pencarian artefak PlanSeal/ManifestTruth di `spike/` | Tidak ada source, test, vector, schema, atau runner untuk kedua kandidat |
| Toolchain project | Tidak ditemukan `package.json`, `foundry.toml`, `hardhat.config.*`, `pnpm-lock.yaml`, atau `Cargo.toml` pada workspace |
| Test suite | `python3 -m unittest discover -s . -p 'test_*.py' -v` → **18/18 lulus** |
| SlippageTruth runner | **5 kasus**, `replay_mismatches=0` |
| ResolverCompat artifacts | **3 fixture**, **12 mutation**, `replay_mismatches=0`, seluruh mutation tidak `PASS` |
| JSON artifacts | `summary.json`, `slippage_truth_summary.json`, dan `memory_lineage_summary.json` lulus `json.tool` |

Artefak aktual berada di [spike/](<spike-README.md:1>), [ResolverCompat summary](<../../evidence/generated/research-spikes/summary.json:1>), [SlippageTruth summary](<../../evidence/generated/research-spikes/slippage_truth_summary.json:1>), dan [MemoryLineage summary](<../../evidence/generated/memory_lineage_summary.json:1>).

## Reclassifikasi yang wajib dipakai

Mulai audit ini, dokumen harus memakai istilah berikut:

1. **Aktual lokal:** hanya hasil yang dapat dijalankan ulang dari source dan artifact workspace.
2. **Source-backed eksternal:** fakta atau laporan dari repository/PR pihak lain; belum menjadi hasil kita.
3. **Target bersyarat:** angka setelah semua gate masa depan lulus; bukan score sekarang.
4. **Klaim tidak terverifikasi:** pernyataan yang belum memiliki source, artifact, atau replay yang dapat diperiksa.

Dengan klasifikasi ini, `MemoryLineage Auditor` menjadi challenger dengan rating riset tertinggi yang sudah memiliki spike lokal, tetapi bukti aktualnya masih sebatas model commitment dan linear-history. `SlippageTruth` dan `ResolverCompat` tetap menjadi spike lokal yang dapat direplay. Keputusan implementasi tidak boleh menyebut MemoryLineage, PlanSeal, atau ManifestTruth telah mencapai 4,7 sebelum gate mereka benar-benar dijalankan.

## Addendum conformance EVM

Setelah bagian di atas ditulis, MemoryLineage sudah memiliki bukti bytecode
lokal: vector ERC-8350 cocok, empat transition valid, authorization EOA dan
ERC-1271 lulus, rotasi authorizer diuji, dan 20/20 mutation ditolak. Replay
independen Python menghasilkan mismatch nol. Jadi deskripsi lama yang menyebut
belum ada kontrak lokal harus dibaca sebagai status sebelum addendum ini.

Klasifikasi tetap ketat: bukti tersebut adalah **aktual lokal**, bukan aktual
testnet. Tidak ada transaksi deployment workspace, reread RPC/indexer kedua,
atau user-impact evidence. Maka tidak ada kandidat yang boleh diberi
`actual_score=4.7`; MemoryLineage hanya memiliki jalur target bersyarat 4,7.
Rincian dan artefak terbaru ada di [AUDIT_MEMORYLINEAGE_ACTUAL_2026-09-08.md](<AUDIT_MEMORYLINEAGE_ACTUAL_2026-09-08.md:1>).

## Audit deep research: MemoryLineage versus aktual

Deep research menemukan fakta primer baru berikut:

| Klaim | Status audit | Bukti | Batas |
| --- | --- | --- | --- |
| ERC-8350 memiliki state machine linear, golden vector, reference repo, dan public Sepolia deployment | **Source-backed eksternal** | Discussion thread, repository, vector JSON, dan official PR #1910 | Draft; dua implementation masih berada dalam satu repo; external security review dan implementasi tim kedua belum terbukti selesai |
| MemoryLineage spike cocok dengan vector publik | **Aktual lokal** | `memory_lineage_summary.json` menunjukkan seluruh vector checks true | Probe tidak menguji Solidity bytecode, EIP-712 signature, event history, proxy, atau RPC |
| Enam mutasi MemoryLineage ditolak dan replay mismatch nol | **Aktual lokal** | Runner menghasilkan 7 kasus, 6 mutation non-pass, dan replay mismatch 0 | Mutasi berjalan pada model Python; belum ada testnet transaction atau independent TypeScript verifier |
| MemoryLineage rating 4,3 | **Rating analitis** | Audit ulang menyimpan dimensi 4,6/4,0/3,9/4,5 = 4,25 dan memakai pembulatan half-up | Bukan score MVP, score juri, atau hasil user research |
| MemoryLineage target 4,7 | **Target bersyarat** | Formula 4,8 + 4,6 + 4,6 + 4,8 dan gate EVM/authority/replay/user | Belum satu pun gerbang produk lulus; target harus dicabut bila contract boundary atau independent replay gagal |
| OWASP ASI06 membuktikan problem memory poisoning relevan | **Source-backed eksternal** | OWASP ASI06 dan Agent Memory Guard | Ini memvalidasi problem space, bukan validasi bahwa solusi kita mencegah semantic poisoning |

Audit final untuk loop ini: MemoryLineage adalah kandidat yang lebih berpotensi
daripada PlanSeal berdasarkan kombinasi problem evidence, failure case, dan
spike lokal. Ia belum lebih terbukti pada boundary EVM. Klaim “lebih berpotensi”
harus dibaca sebagai keputusan prioritas spike berikutnya, bukan klaim produk
sudah benar atau aman.

## Audit ulang nilai ideal

Audit numerik terbaru berada di [AUDIT_NILAI_IDEAL_2026-09-08.md](<AUDIT_NILAI_IDEAL_2026-09-08.md:1>). Hasilnya:

| Kandidat | Nilai sebelumnya | Audit ulang | Perubahan | Alasan |
| --- | ---: | ---: | :---: | --- |
| MemoryLineage Auditor | 4,2 | **4,3** | **↑ 0,1** | 4,25 dibulatkan half-up menjadi 4,3; bukan kenaikan skor MVP |
| Execution Evidence Chain | 3,7 | **3,8** | **↑ 0,1** | 3,75 dibulatkan half-up menjadi 3,8 |
| Kandidat lain | nilai terakhir | nilai terakhir | = | Tidak ada evidence baru yang mengubah dimensi |

Koreksi ini tidak mengubah klasifikasi aktual: tidak ada kandidat yang telah memiliki MVP EVM, independent replay, testnet, dan user evidence lengkap. Karena itu tidak ada kandidat yang boleh disebut memiliki skor aktual 4,7.

## Audit loop terbaru

Pencarian tambahan menemukan beberapa proyek yang benar-benar lebih matang secara eksternal: FlexGov mencantumkan 34 tests dan report hash; KSwap-VM menyediakan negative controls dan proof repository; Doca mencantumkan deployment Base serta paired measurements; Assay mencantumkan challenge/slash flow lintas jaringan; dan rmem-gateway mencantumkan 21/21 Foundry tests serta deployment testnet untuk memory rights. Bukti tersebut tetap **external benchmark**, karena kode, transaksi, dan test suite itu bukan dijalankan oleh workspace ini dan beberapa produknya sudah memiliki fungsi inti yang sama.

Ledger [AUDIT_LOOP_4_7_2026-09-08.md](<AUDIT_LOOP_4_7_2026-09-08.md:1>) memeriksa tiga putaran tambahan. Hasil mesin: `actual_4_7_hits = []`, `conditional_4_7_hits = [MemoryLineage]`, dan `actual_score = null` untuk semua kandidat. Dengan demikian target bersyarat 4,7 masih merupakan rencana pembuktian, bukan klaim aktual.

MemoryRights / ERC-8264 + Capsule dicatat sebagai challenger 4,1 berbasis reference repository. 21/21 tests dan deployment testnet tersebut tetap merupakan bukti eksternal; tidak ada implementasi atau transaksi MemoryRights milik workspace yang dapat dipakai untuk skor aktual.

### Addendum paling baru

Audit terbaru telah menutup dua gate teknis yang sebelumnya kosong untuk
MemoryLineage: contract conformance lokal dan independent replay. Artefak
mesin menunjukkan `contract_or_fork=true`, `independent_verifier=true`, empat
transition valid, dan 20/20 mutation ditolak. Gate public testnet workspace,
reread RPC/indexer kedua, dan user-impact tetap `false`; `actual_score` tetap
`null` dan tidak ada aktual 4,7.
