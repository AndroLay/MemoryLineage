# MemoryLineage developer and impact validation

Status saat ini: **PASS untuk dua sesi AI verifier independen**. Artefak
`evidence/generated/research-spikes/impact_validation.json` menyimpan dua disposable-copy
walkthrough, waktu replay, mismatch, jawaban protokol, dan label identitasnya.
Validasi developer manusia tetap dicatat terpisah sebagai `PENDING`.

## Protokol

Dua developer atau verifier independen yang tidak menulis kontrak atau verifier menjalankan perintah dari
root workspace berikut pada checkout yang sama:

```bash
npm run test:python
node evm/scripts/audit_local.mjs
PYTHONPATH=verifier/python python3 -m memory_lineage.replay
PYTHONPATH=verifier/python python3 -m memory_lineage.impact --repeats 5
```

Sebelum melihat hasil mutation matrix, masing-masing developer menjawab:

1. Apa verdict untuk transition valid dengan `sequence` berikutnya dan
   `prevStateRoot` yang sama dengan head?
2. Apa verdict untuk `sequence` yang melompati head?
3. Apa verdict jika `locatorCommitment` diubah tetapi signature lama dipakai?
4. Apakah raw payload, provenance, atau locator tersimpan di evidence publik?
5. Apa perbedaan bukti yang diberikan oleh commitment continuity dan semantic
   truth dari isi memori?

Catat untuk setiap run dengan field `developerAlias`, `timestamp`,
`checkoutHash`, `answers`, `observedVerdicts`, `replayMilliseconds`, dan
`mismatches`. Jangan mencatat private key atau raw memory. Dua alias harus
berbeda dan checkout hash harus dapat ditelusuri.

## Kriteria lulus

Validasi developer baru dapat diubah menjadi `PASS` bila dua run terpisah:

- mengenali transition valid, sequence gap, signature binding, dan batas
  privacy dengan benar;
- memperoleh hasil yang sama dengan verifier independen;
- tidak menemukan false reject pada corpus valid;
- menyimpan waktu replay dan mismatch yang dapat diperiksa ulang.

Metrik mesin saat paket ini dibuat: corpus valid **4/4 diterima**, mutation
**20/20 ditolak**, false reject pada corpus valid **0%**, replay mismatch **0%**,
gas transition pertama **176265**, gas transition lanjutan rata-rata
**125362**, dan benchmark replay lokal lima kali pada run terbaru memiliki
median sekitar **127.348 ms**. Angka gas dan waktu tersebut adalah ukuran lokal, bukan klaim
biaya jaringan publik atau validasi manusia.

Setelah dua developer selesai, tambahkan hasil mereka ke artefak terpisah dan
ubah gate `independentDeveloperValidation` hanya berdasarkan catatan tersebut.
Untuk sesi AI, gunakan alias yang eksplisit seperti `ai-verifier-session-A`
dan `ai-verifier-session-B`; jangan menyebutnya validasi manusia. Jika
penilaian mensyaratkan pengguna manusia, isi `humanDeveloperValidationStatus`
hanya setelah dua developer manusia menjalankan protokol ini sendiri.
