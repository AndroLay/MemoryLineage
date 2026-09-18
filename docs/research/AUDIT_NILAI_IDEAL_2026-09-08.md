# Audit ulang nilai ideal kandidat

Tanggal audit: **8 September 2026**.

Audit ini memeriksa apakah rating terakhir masih sesuai dengan bukti terbaru, apakah rata-ratanya dihitung konsisten, dan apakah ada alasan yang sah untuk menaikkan atau menurunkan kandidat. Rating di bawah adalah **rating evidence/research**, bukan skor MVP atau prediksi juri.

## Putusan

Ranking riset tetap dipimpin oleh **MemoryLineage Auditor / ERC-8350**, tetapi nilainya dikoreksi dari **4,2 menjadi 4,3** karena dimensi yang dipakai sebelumnya berjumlah tepat 4,25. Dengan aturan pembulatan satu angka desimal `ROUND_HALF_UP`, 4,25 menjadi 4,3.

`Execution Evidence Chain / ERC-8301` juga dikoreksi dari **3,7 menjadi 3,8** karena dimensinya berjumlah tepat 3,75. Ini koreksi aritmetika, bukan bukti implementasi baru.

Tidak ada penurunan evidence rating pada loop ini. Kandidat lain tetap karena pemeriksaan sumber terbaru tidak menghasilkan bukti baru yang mengubah dimensi. Namun tidak ada kandidat yang memiliki skor aktual 4,7: semua angka di atas 4 adalah rating riset, dan MVP kita belum berjalan pada EVM/testnet dengan verifier independen.

## Aturan perhitungan yang dipakai

1. Nilai rata-rata dihitung dari empat dimensi: Innovation, Technical Feasibility, Uniqueness, dan Design.
2. Nilai internal disimpan sampai dua desimal agar selisih pembulatan tidak tersembunyi.
3. Tampilan satu desimal memakai pembulatan half-up, bukan pemotongan dan bukan pembulatan yang berubah karena representasi float.
4. Reference implementation, deployment, dan test suite pihak lain memperkuat rating riset, tetapi tidak menjadi MVP workspace ini.
5. Skor aktual 4 atau 5 tetap memerlukan MVP kita, pemeriksaan independen, pembandingan, dan validasi pengguna sesuai rubric audit.

## Pemeriksaan bukti terbaru

| Area | Hasil pemeriksaan | Dampak pada nilai |
| --- | --- | --- |
| ERC-8350 / MemoryLineage | Draft masih Open/Draft, tetapi memiliki registry Sepolia, golden vector, reference Solidity/TypeScript, dan external recomputation atas hash/state chain. Repository sendiri masih meminta implementasi tim lain dan audit eksternal. | Tidak ada kenaikan dimensi. Rata-rata dikoreksi 4,2 → 4,3; target 4,7 tetap bersyarat. |
| ERC-8410 / PlanSeal | PR masih Open/Draft dan memerlukan review editor. Schema, empat vector, cross-check Rust–JavaScript, dan Ekubo reference flow tetap nyata; halaman juga masih memiliki status review/CI yang harus diperlakukan sebagai external evidence. | Tetap 3,8; tidak ada local artifact atau wallet/testnet kita. |
| ERC-8313 / ManifestTruth | PR masih Open/Draft, memerlukan review editor, dan tidak menyediakan conformance lab setara yang dijalankan workspace. | Tetap 3,8. |
| ERC-8238 / CoercionProof | Evidence reference makin konkret: 71/71, fork Sepolia, deployment terverifikasi, dan live demo. Namun reference product dan Safe/Cedar/Edge tetap menekan uniqueness. | Tetap 3,8; external evidence bukan MVP kita. |
| ERC-8354 / CAPV | EIP tetap membatasi proof pada evaluasi committed interpreter; policy correctness dan interpreter fidelity tetap di luar proof. | Tetap 3,9; proving/liveness tetap blocker. |
| OCP/NAV/Execution evidence | OCP tetap primitive evidence generik; NAV tetap interface/status layer; Execution Evidence Chain tetap framework luas dan overlap tinggi. | OCP tetap 3,8; NAV tetap 3,7; Execution Evidence Chain dikoreksi 3,7 → 3,8 karena pembulatan. |
| Spike workspace | Full suite 18/18, MemoryLineage 7 kasus/6 mutation non-pass/replay mismatch 0, SlippageTruth 5 kasus/replay mismatch 0. | Tidak mengubah rating produk; bukti tetap local throwaway, bukan EVM MVP. |

Sumber primer yang diperiksa: [ERC-8350 discussion](https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098), [ERC-8350 repository](https://github.com/AwareLiquid/ERC-8350), [ERC-8350 PR](https://github.com/ethereum/ERCs/pull/1910), [ERC-8410 PR](https://github.com/ethereum/ERCs/pull/1992), [ERC-8313 PR](https://github.com/ethereum/ERCs/pull/1836), [ERC-8238 discussion](https://ethereum-magicians.org/t/erc-8238-coercion-resistant-vault/28130), [ERC-8354](https://eips.ethereum.org/EIPS/eip-8354), [OCP repository](https://github.com/damonzwicker/observation-commitment-protocol), dan [ERC-8330](https://eips.ethereum.org/EIPS/eip-8330).

## Recalibrasi nilai kandidat yang masih relevan

| Kandidat | Dimensi yang diaudit (I/F/U/D) | Rata-rata sebelumnya | Rata-rata audit ulang | Perubahan | Status nilai |
| --- | --- | ---: | ---: | :---: | --- |
| **MemoryLineage Auditor / ERC-8350** | 4,6 / 4,0 / 3,9 / 4,5 = **4,25** | 4,2 | **4,3** | **↑ 0,1** | Rating riset; belum skor MVP |
| **PlanSeal Conformance Lab** | 4,1 / 3,7 / 3,4 / 4,1 = **3,825** | 3,8 | **3,8** | = | Rating riset; belum artefak lokal |
| **ManifestTruth / PIM** | 4,1 / 3,4 / 3,8 / 3,9 = **3,800** | 3,8 | **3,8** | = | Rating riset; belum verifier/workflow lokal |
| **CAPV PolicyProof Lab** | 4,8 / 2,4 / 4,3 / 4,0 = **3,875** | 3,9 | **3,9** | = | Research track; ZK/fidelity blocker |
| **OCP OutcomeProof** | 4,1 / 4,3 / 2,8 / 4,0 = **3,800** | 3,8 | **3,8** | = | Evidence layer; uniqueness rendah |
| **NAVFreshness Guard** | 4,0 / 3,3 / 3,5 / 3,9 = **3,675** | 3,7 | **3,7** | = | RWA fallback; provider truth di luar scope |
| **CoercionProof Scenario Lab** | 4,1 / 3,8 / 3,3 / 4,0 = **3,800** | 3,8 | **3,8** | = | Reference-backed; belum implementasi kita |
| **Execution Evidence Chain** | 4,2 / 3,4 / 3,4 / 4,0 = **3,750** | 3,7 | **3,8** | **↑ 0,1** | Koreksi pembulatan; framework terlalu luas |
| **Warden Compromise Lab** | 3,9 / 3,7 / 3,6 / 3,9 = **3,775** | 3,8 | **3,8** | = | Reference evidence; belum run kita |
| **SlippageTruth Lab** | 3,8 / 3,7 / 3,1 / 3,8 = **3,600** | 3,6 | **3,6** | = | Spike lokal arithmetic; belum EVM/fork |
| **RWA StatusGuard** | 3,8 / 3,4 / 3,7 / 3,8 = **3,675** | 3,7 | **3,7** | = | Draft adapter; belum atomic integration |
| **HiddenRefs Explorer** | 3,9 / 3,2 / 3,8 / 3,9 = **3,700** | 3,7 | **3,7** | = | Uniqueness kuat; belum contract/user proof |
| **ResolverCompat** | 3,0 / 3,5 / 2,8 / 2,8 = **3,025** | 3,0 | **3,0** | = | Spike lokal; external fixture belum ada |
| **RWA AgencyGuard** | prior audit | 3,3 | **3,3** | = | Penurunan 3,4 → 3,3 sudah valid dari loop sebelumnya |
| **BoundedSpend, SettlementLock, SkillPackage, QuoteTruth, dan kandidat historis lain** | Tidak ada bukti baru yang mengubah dimensi | nilai terakhir | nilai terakhir | = | Tetap sampai ada evidence baru |

## Nilai aktual versus rating riset

MemoryLineage sekarang adalah rating riset tertinggi, bukan produk terbaik yang sudah terbukti. Spike lokal hanya memverifikasi commitment, vector, linearity, dan mutation pada model Python. Ia belum memverifikasi Solidity, signature EOA/ERC-1271, event history, proxy immutability, RPC, testnet, data availability, atau semantic truth memori.

Karena itu keputusan yang paling ideal saat ini adalah mempertahankan MemoryLineage sebagai **prioritas riset berikutnya**, tetapi menahan klaim skor aktualnya di bawah 4 sampai contract boundary dan independent replay benar-benar lulus. Bila salah satu gate tersebut gagal, ranking kerja harus kembali membandingkan PlanSeal dengan SlippageTruth berdasarkan toolchain dan bukti yang benar-benar tersedia.

Kesimpulan audit: **naik secara aritmetika** untuk MemoryLineage dan Execution Evidence Chain; **tetap** untuk semua kandidat lain; **tidak ada penurunan baru**. Tidak ada target 4,7 yang berubah menjadi skor aktual.

### Addendum audit EVM

MemoryLineage sekarang juga lulus conformance bytecode lokal dan replay
independen: golden vector cocok, empat transition valid, 20/20 mutation
ditolak, dan mismatch replay nol. Ini mengubah status dua gate teknis dari
belum terbukti menjadi terbukti lokal, tetapi belum mengubah angka: deployment
testnet workspace, reread RPC/indexer kedua, dan user-impact measurement belum
ada. Rating tetap 4,3 dan `actual_score` tetap `null`; 4,7 masih target
bersyarat.

## Audit putaran berikutnya

Loop baru menguji kandidat terhadap benchmark yang memiliki demo, repository, test suite, atau deployment nyata. Hasilnya tidak menghasilkan kenaikan yang sah untuk kandidat workspace:

| Kandidat baru atau pembanding | Nilai audit | Bukti yang membatasi kesimpulan |
| --- | ---: | --- |
| FlexGov | 4,0 | Implementasi dan tests publik sudah ada; ini benchmark eksternal, bukan MVP kita |
| KSwap-VM | 3,9 | Proof/negative control kuat, tetapi overlap langsung dan hanya 20/52 opcode yang dimodelkan menurut repository |
| Doca / Assay / Commitment Issues | 3,8 / 3,8 / 3,8 | Masing-masing memiliki bukti demo atau live flow, tetapi kemampuan inti sudah dibangun pihak lain |
| MemoryRights / ERC-8264 + Capsule | 4,1 | Reference repository melaporkan 21/21 test dan deployments, tetapi tidak independen dari penulis standar dan overlap produk tinggi |
| ERC-8263 / ERC-8273 / ERC-8257 | 3,8 / 3,9 / 3,7 | Draft/reference layer yang memperkecil ruang novelty generic proof, attestation, dan registry |

Rata-rata ini tercatat secara deterministik dalam [candidate_audit_loop.json](../../evidence/generated/research-spikes/candidate_audit_loop.json:1). Tidak ada kandidat dengan `actual_score` karena tidak ada yang sekaligus memenuhi contract/fork, independent verifier, live/testnet flow, dan user-impact gate milik workspace. MemoryLineage tetap 4,3 sebagai rating riset dan tetap satu-satunya jalur conditional 4,7.
