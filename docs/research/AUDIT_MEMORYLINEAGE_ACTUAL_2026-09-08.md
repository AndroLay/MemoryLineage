# Audit aktual MemoryLineage / ERC-8350

Tanggal: **8 September 2026**. Dokumen ini memperbarui rating riset sebelumnya setelah kontrak lokal diselaraskan dengan batas normatif ERC-8350 v1. Ia memisahkan bukti yang benar-benar dijalankan di workspace dari bukti live upstream dan gate yang membutuhkan otorisasi eksternal.

> Arsip historis. Dokumen ini menyimpan keadaan riset pada 8 September dan bukan
> score submission saat ini. Artefak generated yang disebut di versi lama telah
> dikeluarkan dari paket publik; gunakan [`docs/testing.md`](../testing.md) dan
> `evidence/local/` sebagai sumber verifikasi saat ini.

## Hasil yang sudah terbukti

Kontrak [MemoryLineageRegistry.sol](<../../contracts/solidity/MemoryLineageRegistry.sol:1>) sekarang mengikuti batas utama reference v1: `registerSpace`, derivasi `spaceId` dari controller dan salt, `commitTransition` dengan tujuh field `ExperienceDelta`, domain EIP-712 `AgentMemoryState`, `updateSpaceAuthorization` dengan `configNonce`, EOA canonical ECDSA, dan ERC-1271 dengan fallback ECDSA. `nextStateRoot` dihitung kontrak dari `prevStateRoot` dan `transitionId`; caller tidak mengirim root hasil.

Artefak lokal yang dapat direplay:

| Gate | Bukti | Hasil |
| --- | --- | --- |
| Compiler/EVM | solc `0.8.36`, target EVM Shanghai, EthereumJS VM chain `31337` | **PASS** |
| Golden vector | space ID, tiga typehash, transition ID, dan next root cocok dengan vector ERC-8350 | **PASS** |
| State machine | empat transition berurutan, head akhir sequence 4, root direplay | **PASS** |
| Authorization | EOA, direct caller path, ERC-1271 fixture, wrong signer, wrong chain/domain, malformed signature | **PASS** |
| Rotation | controller/authorizer rotation dan `configNonce=1`; signature authorizer lama ditolak | **PASS** |
| Adversarial mutation | 20 kasus bernama ditolak dengan alasan kontrak | **20/20 PASS** |
| Independent replay | verifier Python mandiri membaca JSON publik dan menghitung final root serta alasan mutation | **PASS**, mismatch 0 |
| Privacy boundary | evidence hanya menyimpan fixed-size commitments; payload/provenance/locator mentah tidak diekspor | **PASS** |

Ringkasan mesin historis berada di generated output yang tidak termasuk paket
publik. Bundle publik yang masih tersedia adalah
[memory_lineage_evm_evidence.json](<../../evidence/local/memory_lineage_evm_evidence.json:1>);
jalankan `npm run verify` untuk meregenerasi ringkasan lokal.

Gate agregat historis dapat diaudit ulang dengan
[gates.py](<../../verifier/python/memory_lineage/gates.py:1>). Release saat ini
tidak mengeluarkan `actualScore`; ia melaporkan kondisi pass/fail dan batas
bukti yang tersedia.

Verifier independen dijalankan melalui [run_memory_lineage_evm_audit.py](<../../verifier/python/memory_lineage/replay.py:1>) dan mengembalikan `verdict=PASS`, empat transition valid, final sequence 4, serta 20 mutation rejected. Ia tidak mengimpor [memory_lineage.py](<../../verifier/python/memory_lineage/model.py:1>) atau wrapper JavaScript.

Pengukuran impact mesin tersedia di [memory_lineage_impact.json](<../../evidence/generated/memory_lineage_impact.json:1>): corpus valid 4/4 diterima, false reject 0%, mutation rejection 20/20, replay mismatch 0%, gas transition pertama 176265, dan gas transition lanjutan rata-rata 125362. Benchmark replay lima kali pada run terbaru menghasilkan median 143,45 ms pada environment ini. Dua sesi AI verifier independen kemudian menjalankan disposable copy: replay 65 ms dan 61 ms, keduanya mismatch 0 dan mengenali batas semantic truth.

Deployment publik workspace sekarang sudah terbukti di Ethereum Sepolia. Registry berada di `0x36fE9FA585565615Adcfe8680a126F770931E160`; deployment, registrasi, dan commit valid mined dengan status 1, sedangkan commit sequence gap yang diharapkan mined dengan status 0. [sepolia_deployment.json](<../../evidence/sepolia/sepolia_deployment.json:1>) menyimpan transaction hashes dan bytecode hash. [sepolia_reread.json](<../../evidence/sepolia/sepolia_reread.json:1>) membaca ulang melalui `https://rpc.sepolia.ethpandaops.io`, mencocokkan code hash dan head, serta mengembalikan `verdict=PASS`.

## Audit terhadap klaim 4,7

| Syarat aktual | Status | Alasan |
| --- | --- | --- |
| Contract conformance | **Lulus lokal** | API, domain, derivasi namespace, state root, dan vector telah dijalankan di bytecode lokal |
| Independent verifier | **Lulus lokal** | Implementasi Python kedua mereplay evidence tanpa publisher |
| Mutation/adversarial | **Lulus lokal** | 20/20 ditolak, termasuk rollback, gap, predecessor, zero commitment, unknown space, domain, signature, dan parallel history |
| Public testnet milik workspace | **Lulus** | Registry workspace live di Sepolia dengan deployment, register, valid commit, dan expected rejected commit yang terverifikasi |
| Second RPC/indexer untuk deployment workspace | **Lulus** | RPC kedua mencocokkan chain ID, code hash, seluruh receipt, head, state root, dan sequence |
| Independent developer/agent validation | **AI-only evidence** | Dua sesi AI verifier terpisah menjalankan disposable copy; ini bukan validasi developer atau pengguna manusia |
| Human developer/UX validation | **Belum dilakukan** | Artefak membedakan status manusia dari gate agent; tidak ada klaim pemahaman pengguna manusia |

Probe read-only sebelumnya menemukan deployment referensi upstream dan tiga typehash yang cocok. Artefak itu tetap diberi label `external_reference_live_read_only`; ia tidak digunakan untuk menaikkan skor produk workspace. Deployment workspace sendiri kini memiliki bukti terpisah melalui dua RPC publik.

Gate bukti teknis dan validasi AI historis pernah memenuhi jalur internal
bersyarat, tetapi artefak score tersebut sudah dipensiunkan. `actualScore=4.7`
dan `actual47=true` bukan score juri, bukan validasi manusia, dan bukan klaim
release saat ini.

## Cara menutup gate yang tersisa

Helper [deploy_sepolia.mjs](<../../evm/scripts/deploy_sepolia.mjs:1>) hanya menerima `DEPLOYER_PRIVATE_KEY` melalui environment dan mewajibkan flag `--confirm-public-testnet`; ia tidak membaca atau menulis key ke file, memeriksa chain ID `11155111`, deployment code, registrasi space, satu transition valid, dan satu transaction dengan sequence salah yang harus mined sebagai revert. Deployment sudah dijalankan dengan signer sementara yang hanya hidup di memori. Reread melalui [reread_sepolia.mjs](<../../evm/scripts/reread_sepolia.mjs:1>) telah lulus melalui RPC kedua.

Gate independent-agent sudah menyimpan hasil dua sesi: waktu replay, mismatch, observed verdicts, batas privacy, dan keterbatasan semantic truth. Jika penilaian yang diminta harus berasal dari manusia, dua developer tetap dapat menjalankan protokol yang sama dan mengganti status `humanDeveloperValidationStatus` hanya berdasarkan catatan nyata mereka; langkah itu akan menghasilkan `strictHumanActualScore=4.7`.

Ledger gate historis ada di [IMPACT_VALIDATION.md](IMPACT_VALIDATION.md) dengan
`validationMode=independent_ai_verifier_sessions`; status tersebut tetap bukan
validasi manusia. Current release tidak memakai ledger itu untuk menaikkan
score.
