# Audit Submission Publik 3rd-Web-Hack

Tanggal audit: **18 September 2026**.

Dokumen ini mencatat submission yang dapat ditemukan dan diaudit melalui sumber
publik. Ini bukan daftar lengkap seluruh peserta dan bukan hasil penilaian resmi
juri.

## Batas cakupan

Gallery resmi masih menyatakan belum dipublikasikan:

- [Project gallery](https://3rd-web-hack.devpost.com/project-gallery)
- [Daftar peserta](https://3rd-web-hack.devpost.com/participants) meminta login
- [Halaman resmi hackathon](https://3rd-web-hack.devpost.com/) menampilkan jumlah peserta, bukan jumlah submission

Pencarian lintas Devpost, GitHub, live demo, dan YouTube menemukan lima halaman
project yang terhubung secara publik dengan 3rd-Web-Hack:

1. [DEDSEC Shadow NET](https://devpost.com/software/dedsec-shadow-net)
2. [Kinetic](https://devpost.com/software/kinetic-m9i5gv)
3. [ArcLight AI](https://devpost.com/software/arclight-ai)
4. [Forkline: Rehearse the Rollback](https://devpost.com/software/forkline-rehearse-the-rollback)
5. [FinalityDesk](https://devpost.com/software/finalitydesk)

Pencarian lanjutan juga menemukan beberapa halaman project publik milik peserta
atau akun yang berkaitan dengan blockchain, AI, dan security. Halaman-halaman itu
tidak dimasukkan ke skor kandidat karena tidak menampilkan `Submitted to
3rd-Web-Hack`, atau secara eksplisit menunjuk ke hackathon lain. Daftarnya dicatat
di bagian [lead yang tidak dihitung](#6-hasil-pencarian-lanjutan-lead-yang-tidak-dihitung)
agar kandidat palsu tidak masuk ke perbandingan.

Untuk ArcLight dan Kinetic, hasil indeks Devpost menunjukkan 3rd-Web-Hack pada
daftar submission, tetapi beberapa pengambilan halaman memberikan metadata yang
tidak konsisten. Keduanya dicatat sebagai **terhubung secara publik dengan
confidence sedang**, bukan sebagai bukti gallery resmi yang lengkap.

## Rubrik audit

Skor di bawah menggunakan skala 1–5 dengan bobot sama pada Innovation,
Technical Feasibility, Uniqueness, dan Design. Skor ini adalah **evidence-adjusted
rating**, bukan skor Devpost.

- **Klaim:** apa yang dijanjikan halaman project.
- **Bukti:** apa yang dapat diperiksa dari source, live page, metadata video, atau artefak publik.
- **Tidak terbukti:** tidak ditemukan bukti publik yang cukup; ini bukan bukti bahwa fitur tersebut tidak ada.
- **Replay lokal:** source dan test dijalankan ulang oleh audit ini. Tidak ada replay code competitor yang berhasil dilakukan pada putaran ini.

## Ringkasan kandidat

| Project | Creator | GitHub | Video/live | Web3 fit | Evidence-adjusted rating |
| --- | --- | --- | --- | ---: | ---: |
| [Kinetic](https://devpost.com/software/kinetic-m9i5gv) | Shivani1105 Kinagi | Tidak ditemukan repository publik yang terhubung | Live tercantum, tidak dapat diambil; video publik tidak ditemukan | Tinggi | **3,0–3,6** |
| [ArcLight AI](https://devpost.com/software/arclight-ai) | Deven Goyal, Rishabh Verma | [Fork publik](https://github.com/Devengoyal885/Arclight-ai) dari [repository parent](https://github.com/Rishabhv16/arclight-ai) | [Live page](https://arclight-ai.vercel.app/), [video](https://www.youtube.com/watch?v=thGccIaJ3Bk) | Rendah/belum terbukti | **3,0–3,4** |
| [DEDSEC Shadow NET](https://devpost.com/software/dedsec-shadow-net) | Gaurav W | Tidak ditemukan repository publik yang terhubung | Live dan YouTube tercantum di Devpost; endpoint/video tidak dapat diaudit frame-by-frame | Sangat rendah/belum terbukti | **2,6–3,1** |
| [Forkline](https://devpost.com/software/forkline-rehearse-the-rollback) | Joseph Ayanda | [Source branch](https://github.com/josepha-mayo/Joseph-Portfolio/tree/forkline-outbox-20260908) | [Delivery Lab](https://6aa089c0cc86050008977842--josephm.netlify.app/delivery.html), [video](https://www.youtube.com/watch?v=ZNnNR-yUtdg), [MP4](https://6aa089c0cc86050008977842--josephm.netlify.app/demo.mp4) | Tinggi | **4,0–4,4** |
| [FinalityDesk](https://devpost.com/software/finalitydesk) | Hyunsik Park | [GitHub](https://github.com/HyunsikParker/finalitydesk) | [YouTube](https://www.youtube.com/watch?v=M4Iow0HCL2U), [MP4](https://github.com/HyunsikParker/finalitydesk/blob/main/deliverables/FinalityDesk-demo.mp4) | Tinggi | **3,6–4,1** |

## 1. Kinetic

### Tautan dan identitas

- Devpost: [Kinetic](https://devpost.com/software/kinetic-m9i5gv)
- Creator: **Shivani1105 Kinagi**
- GitHub: **tidak ditemukan link repository publik pada halaman yang diaudit**
- Live demo yang tercantum: `https://kinetic-c9x74pgi3-shivanikinagi-gmailcoms-projects.vercel.app`
- Video: **tidak ditemukan link video publik pada halaman yang diaudit**

### Klaim ide

Kinetic memosisikan diri sebagai marketplace peer-to-peer untuk compute GPU/CPU
berbasis Algorand. Fitur yang diklaim meliputi ProviderRegistry, escrow,
BadgeMinter, proof-of-compute sembilan langkah, X-402, SSE, Pera Wallet, dan
autonomous consumer agent.

Problem dan kebutuhan Web3-nya paling jelas di antara kandidat publik yang
ditemukan. Marketplace compute memang memerlukan escrow, pembayaran, dan
mekanisme verifikasi yang dapat dipercaya oleh dua pihak yang tidak saling
mengenal.

### Audit klaim versus bukti

- Halaman Devpost menjelaskan arsitektur dan stack secara rinci, tetapi itu masih
  merupakan **klaim dari submission**.
- Tidak ada repository, contract address, transaction hash, atau test artifact
  publik yang berhasil saya hubungkan dari halaman tersebut.
- Live demo tidak tersedia bagi crawler pada saat audit.
- Roadmap menyebut fase berikutnya adalah mengganti **simulated compute** dengan
  eksekusi GPU nyata. Artinya, halaman tersebut belum membuktikan bahwa proof
  chain saat ini membuktikan pekerjaan GPU benar-benar dilakukan.
- SHA-256 chain dapat membuktikan bahwa laporan langkah saling terhubung. Ia
  belum otomatis membuktikan kebenaran komputasi tanpa execution witness,
  independent rerun yang bermakna, attestation, atau mekanisme verifikasi lain.

### Penilaian

| Dimensi | Audit |
| --- | --- |
| Innovation | Tinggi secara konsep; decentralized compute dan escrow relevan |
| Technical Feasibility | Potensial tinggi, tetapi source dan deployment belum terbukti |
| Uniqueness | Sedang; compute marketplace dan proof-of-work/payment sudah memiliki banyak preseden |
| Design | Belum dapat dinilai secara jujur tanpa live page atau video |
| Status bukti | **Devpost narrative only** |

Kinetic adalah **kompetitor ide Web3 paling serius**, tetapi bukan kompetitor
dengan bukti publik paling lengkap.

Tanggal project yang terlihat di Devpost adalah 28 April 2026. Periode submission
resmi dimulai 22 Agustus 2026 menurut [dates resmi](https://3rd-web-hack.devpost.com/details/dates).
Karena [rules](https://3rd-web-hack.devpost.com/rules) menyebut project harus
original dan dikembangkan untuk hackathon, timestamp ini perlu dijelaskan oleh
peserta. Timestamp tersebut bukan bukti otomatis bahwa submission tidak sah.

## 2. ArcLight AI

### Tautan dan identitas

- Devpost: [ArcLight AI](https://devpost.com/software/arclight-ai)
- Creator di Devpost: **Deven Goyal** dan **Rishabh Verma**
- GitHub submission: [Devengoyal885/Arclight-ai](https://github.com/Devengoyal885/Arclight-ai)
- GitHub parent: [Rishabhv16/arclight-ai](https://github.com/Rishabhv16/arclight-ai)
- Live demo: [arclight-ai.vercel.app](https://arclight-ai.vercel.app/)
- Video: [ArcLight AI demo](https://www.youtube.com/watch?v=thGccIaJ3Bk)

### Klaim ide

ArcLight AI adalah Urban Intelligence OS dengan dashboard smart waste, smart
energy, air quality, digital twin, citizen layer, dan AI copilot. Ia memiliki
presentasi visual paling siap di antara kandidat yang berhasil dibuka.

### Audit GitHub

- Repository publik adalah **fork** dari repository Rishabhv16. [Metadata GitHub](https://api.github.com/repos/Devengoyal885/Arclight-ai)
  menunjukkan `fork: true`, parent repository, dan tanggal pembuatan 6 Juni
  2026.
- [package.json](https://github.com/Devengoyal885/Arclight-ai/blob/main/package.json)
  berisi `dev`, `build`, `start`, dan `lint`, tetapi tidak memiliki script test
  atau typecheck.
- Struktur publik berisi Next.js/TypeScript, `app`, `components`, dan `lib`.
  Tidak terlihat contract directory atau dependency Web3.
- [lib/api.ts](https://github.com/Devengoyal885/Arclight-ai/blob/main/lib/api.ts)
  mengambil data Open-Meteo dan WAQI, tetapi memiliki fallback simulasi.
- [lib/simulators/index.ts](https://github.com/Devengoyal885/Arclight-ai/blob/main/lib/simulators/index.ts)
  menghasilkan data sintetis dengan noise dan `Math.random()`.
- `lib/api.ts` juga memuat token API secara langsung di source publik. Nilainya
  tidak disalin ke dokumen ini. Token tersebut sebaiknya dirotasi atau dibatasi.
- Klaim YOLOv11, PyTorch, LSTM, XGBoost, PostgreSQL, Kafka, dan MQTT belum
  terbukti dari source publik yang berhasil diaudit. Yang terlihat jelas adalah
  aplikasi Next.js/TypeScript dengan API eksternal dan fallback simulator.

### Audit live dan video

- Teks landing page live berhasil diambil dan menampilkan dashboard, enam modul,
  angka sensor, dan sustainability score.
- Metadata video berhasil dikonfirmasi melalui link YouTube dan deskripsinya.
  Audit ini tidak mengklaim telah menonton seluruh video frame-by-frame.
- Tidak ada build, test, atau browser walkthrough lokal yang dijalankan terhadap
  repository competitor.

### Penilaian

| Dimensi | Audit |
| --- | --- |
| Innovation | Sedang; smart-city AI dashboard adalah ruang yang ramai |
| Technical Feasibility | Sedang; source dan live shell ada, tetapi test dan klaim backend belum terbukti |
| Uniqueness | Rendah–sedang; banyak modul membuat breadth tinggi, tetapi diferensiasi Web3 tidak terlihat |
| Design | Paling kuat dari kandidat publik yang ditemukan |
| Status bukti | **GitHub static review + live landing page + video metadata; belum replay** |

ArcLight adalah **benchmark presentation dan design**, bukan benchmark Web3.

Project Devpost dimulai 10 Juli 2026 dan parent repository GitHub dibuat 6 Juni
2026. Ini menimbulkan pertanyaan timeline terhadap [rules hackathon](https://3rd-web-hack.devpost.com/rules),
meskipun tidak cukup untuk menyimpulkan pelanggaran.

## 3. DEDSEC Shadow NET

### Tautan dan identitas

- Devpost: [DEDSEC Shadow NET](https://devpost.com/software/dedsec-shadow-net)
- Creator: **Gaurav W**
- GitHub: **tidak ditemukan repository publik yang terhubung**
- Live demo tercantum: `https://dedsec-shadow-net-14425316810.asia-southeast1.run.app`
- Video: Devpost hanya mengekspos link pendek `youtu.be`; URL target exact tidak
  berhasil diambil pada audit ini.

### Klaim ide

DEDSEC Shadow NET adalah messaging sementara berbasis WebSocket dengan RAM-only
storage, anonymous session ID, batas dua pengguna, shredding manual, dan expiry
15 menit. Problem-nya sangat mudah dipahami dan tema cyberpunk-nya berpotensi
memberi demo yang memorable.

### Audit klaim versus bukti

- Built With pada Devpost berisi React, Node.js, Express, Vite, WebSockets, dan
  Google Cloud. Tidak terlihat blockchain, wallet, smart contract, atau
  decentralized protocol.
- Halaman menyebut “encrypted-style”, bukan bukti true end-to-end encryption.
- Roadmap mereka sendiri menempatkan true E2EE dan decentralized communication
  sebagai fitur masa depan.
- RAM-only mengurangi persistent database storage, tetapi tidak membuktikan
  secure erasure dari log, memory dump, crash dump, atau lingkungan hosting.
- Live endpoint mengalami timeout pada pengambilan publik; video tidak dapat
  diaudit frame-by-frame; source code tidak tersedia untuk replay.

### Penilaian

| Dimensi | Audit |
| --- | --- |
| Innovation | Sedang; ephemeral messaging menarik tetapi bukan primitive baru |
| Technical Feasibility | Sedang untuk WebSocket app, rendah untuk security claim yang lebih luas |
| Uniqueness | Sedang pada branding dan UI, rendah pada kategori ephemeral chat |
| Design | Berpotensi kuat, tetapi belum diverifikasi secara visual |
| Status bukti | **Devpost narrative + link live/video; no source replay** |

DEDSEC adalah **benchmark storytelling**, tetapi kecocokan terhadap requirement
Blockchain/Web3 masih sangat lemah.

Project ditandai mulai 1 Agustus 2026, sebelum submission window dimulai. Ini
dicatat sebagai timeline risk yang perlu dijelaskan, bukan sebagai putusan
eligibility.

## 4. Forkline: Rehearse the Rollback

### Tautan dan identitas

- Devpost: [Forkline: Rehearse the Rollback](https://devpost.com/software/forkline-rehearse-the-rollback)
- Creator/source owner: **Joseph Ayanda**
- GitHub: [Joseph-Portfolio, branch `forkline-outbox-20260908`](https://github.com/josepha-mayo/Joseph-Portfolio/tree/forkline-outbox-20260908)
- Live Delivery Lab: [delivery.html](https://6aa089c0cc86050008977842--josephm.netlify.app/delivery.html)
- Pitch: [pitch.html](https://6aa089c0cc86050008977842--josephm.netlify.app/pitch.html)
- Source package: [source.zip](https://6aa089c0cc86050008977842--josephm.netlify.app/source.zip)
- Current video: [Forkline DeliveryLab demo](https://www.youtube.com/watch?v=ZNnNR-yUtdg)
- Direct MP4: [demo.mp4](https://6aa089c0cc86050008977842--josephm.netlify.app/demo.mp4)
- Historical engine-workbench video: [older recording](https://www.youtube.com/watch?v=KzO3ZU10gOU)
- Submission readback: [SUBMISSION_CONFIRMED.json](https://raw.githubusercontent.com/josepha-mayo/Joseph-Portfolio/forkline-outbox-20260908/SUBMISSION_CONFIRMED.json)

### Klaim ide

Forkline menguji delivery queue terhadap blockchain reorganization, kehilangan
acknowledgement, dan restart worker tanpa mengulang efek eksternal. Skenarionya
jelas: event blockchain dapat hilang setelah order masuk queue, tetapi tiket,
email, atau tindakan eksternal yang sudah terjadi tidak dapat dibatalkan dengan
rollback database biasa.

Delivery Lab menghubungkan reorg checker dengan local EVM, SQLite outbox, dan
HTTP receiver yang mempunyai ledger/idempotency sendiri. Forkline membandingkan
consumer yang hanya memeriksa eligibility saat enqueue dengan worker yang
memeriksa observed head lagi sebelum dispatch.

### Audit klaim versus bukti

- Branch publik berisi `contracts`, `src`, `tests`, `evidence`, `public`, dan
  script rebuild/deployment lokal.
- `package.json` menyediakan test, local EVM, outbox recording, dan lab server.
- Release evidence publik melaporkan 65 Node tests, 9 local-EVM checks, 8
  independent Python SQLite checks, serta 39 browser workflows. Angka tersebut
  adalah evidence yang diterbitkan project, bukan hasil replay oleh workspace ini.
- `DELIVERY_ENTRY_STATUS.md` membedakan dengan jelas video Delivery Lab terbaru
  dari video workbench lama. Ini mencegah tautan video lama dibaca sebagai demo
  terbaru.
- Demo dan live page adalah replay/static presentation untuk public viewer; live
  SQLite/HTTP controls dijalankan secara lokal menurut source.
- Batasan dijelaskan dengan baik: tidak ada mainnet consensus, cryptographic
  inclusion proof, canonicality proof, exactly-once external effect, atau
  security certification.

### Penilaian

| Dimensi | Audit |
| --- | --- |
| Innovation | Tinggi; menghubungkan reorg observation dengan irreversible external delivery |
| Technical Feasibility | Tinggi; source, local EVM, SQLite receiver, evidence, dan browser paths tersedia |
| Uniqueness | Sedang–tinggi; primitive-nya bukan baru, tetapi integration/rehearsal scope cukup tajam |
| Design | Tinggi; live replay, pitch, video, dan failure story tersusun jelas |
| Status bukti | **Source-backed public release; belum direplay oleh workspace** |

Forkline adalah **kompetitor publik terkuat secara keseluruhan** yang ditemukan
sejauh ini. Kelemahannya adalah scope masih single-worker/local-EVM dan tidak
membuktikan canonicality blockchain secara mandiri. Submission readback publik
mencatat status Submitted pada 8 September 2026.

## 5. FinalityDesk

### Tautan dan identitas

- Devpost: [FinalityDesk](https://devpost.com/software/finalitydesk)
- Creator: [Hyunsik Park](https://devpost.com/skaiea13)
- GitHub: [HyunsikParker/finalitydesk](https://github.com/HyunsikParker/finalitydesk)
- Video YouTube: [FinalityDesk — ERC-20 transfer verification demo](https://www.youtube.com/watch?v=M4Iow0HCL2U)
- Video repository: [FinalityDesk-demo.mp4](https://github.com/HyunsikParker/finalitydesk/blob/main/deliverables/FinalityDesk-demo.mp4)
- Presentation: [FinalityDesk-presentation.pptx](https://github.com/HyunsikParker/finalitydesk/blob/main/deliverables/FinalityDesk-presentation.pptx)

### Klaim ide

FinalityDesk memeriksa apakah transfer ERC-20 yang berhasil benar-benar
memenuhi network, token contract, recipient, dan exact amount yang diharapkan.
Browser membaca receipt dan block melalui read-only JSON-RPC, memeriksa
canonical block membership dua kali, lalu membandingkan block transfer dengan
provider finalized head.

Problem-nya sempit dan konkret: transaksi yang sukses tetap dapat mengirim token
yang salah, kepada penerima yang salah, atau dengan jumlah yang salah.

### Audit GitHub dan test

- Repository dibuat 10 September 2026, tidak terlihat sebagai fork, dan memiliki
  README, package-lock, source, test, dan deliverables.
- [package.json](https://github.com/HyunsikParker/finalitydesk/blob/main/package.json)
  menjalankan `npm ci`, `npm test`, dan Vite build/dev.
- Test suite publik memiliki satu positive finalized case, 22 mutation cases,
  validasi amount, exact integer di atas batas presisi floating point, larangan
  write RPC, invalid RPC envelopes, dan credential-bearing URL rejection.
- Source menggunakan exact base-unit integer, Transfer event signature,
  receipt status, canonical block lookup sebelum/sesudah finality lookup, dan
  finalized head.
- Kode secara eksplisit membatasi klaim: single-provider observation, bukan
  light client atau cryptographic consensus proof.

### Audit video dan batasan

- Metadata YouTube menunjukkan video berjudul `FinalityDesk — ERC-20 transfer
  verification demo`.
- Repository juga menyimpan MP4 dan presentation, sehingga video tidak hanya
  bergantung pada link YouTube.
- Demo dan source belum dijalankan ulang oleh workspace ini.
- Tiga example di browser adalah synthetic dan tidak melakukan network request.
- Project belum membuktikan invoice ownership, replay prevention antar-invoice,
  token behavior, current balance, fiat receipt, atau independent RPC comparison.

### Penilaian

| Dimensi | Audit |
| --- | --- |
| Innovation | Sedang–tinggi; problem pembayaran sangat konkret, tetapi transfer verification memiliki prior art |
| Technical Feasibility | Tinggi untuk scope read-only; source dan test tersedia |
| Uniqueness | Sedang; diferensiasi berada pada evidence report dan finality check |
| Design | Sedang–tinggi; demo, presentation, synthetic cases, dan export tersedia |
| Status bukti | **Public source + test/video artifacts; belum replay oleh workspace** |

FinalityDesk adalah **kompetitor paling dekat pada pola evidence-based Web3
verification**, tetapi scope-nya tidak sama dengan private AI-memory lineage.

## 6. Hasil pencarian lanjutan: lead yang tidak dihitung

Bagian ini penting untuk audit negatif. Project berikut benar-benar ada dan
memiliki ide yang dapat dibandingkan, tetapi belum ada bukti publik yang cukup
untuk menyebutnya submission 3rd-Web-Hack. Saya tidak memberi skor hackathon
kepada mereka.

| Project | Tautan publik | Temuan | Status audit |
| --- | --- | --- | --- |
| Unsubscribely | [Devpost](https://devpost.com/software/unsubscribely), [GitHub](https://github.com/devndesigner6/unsubly) | Subscription vault dan autonomous payment berbasis Algorand; README menyebut AlgoBharat Hack Series 3.0. Halaman Devpost yang dibaca tidak menampilkan `Submitted to` 3rd-Web-Hack. | **Lead Web3, tidak terverifikasi sebagai submission** |
| C402 | [Devpost](https://devpost.com/software/c402) | HTTP 402 gateway untuk pembayaran API per-request dengan Cardano ADA. Halaman publik tidak menampilkan entri submission 3rd-Web-Hack. | **Lead Web3, tidak terverifikasi sebagai submission** |
| EcoLoop | [Devpost](https://devpost.com/software/ecoloop-crij8n) | Waste reporting dan volunteer cleanup dengan reward token W2E di Polygon Mumbai; halaman mencatat project mulai 29 April 2026 dan tidak menampilkan entri submission 3rd-Web-Hack. | **Lead Web3, timeline dan submission tidak terbukti** |
| INTAG | [Devpost](https://devpost.com/software/intag-5idlj8) | URL/Web3 security scanner yang menggabungkan URLScan dan PhishTank; project dimulai 15 Agustus 2026, tetapi halaman publik tidak menampilkan `Submitted to` 3rd-Web-Hack. | **Lead security, tidak terverifikasi sebagai submission** |
| AAGO | [Devpost](https://devpost.com/software/aago-ai-anti-scam-shopping-agent) | AI anti-scam shopping assistant dengan escrow; halaman menyebut dibangun untuk Shipaton dan bagian `Submitted to` tidak berisi 3rd-Web-Hack. | **Project event lain** |
| tiny.place | [Devpost](https://devpost.com/software/tiny-place) | Marketplace identity, jobs, dan escrow privat untuk AI agents dengan Midnight Compact; halaman secara eksplisit mencantumkan Brainwave 2026 – Midnight Track. | **Project event lain** |
| Cloudy | [Devpost](https://devpost.com/software/podex) | Keychain fisik untuk persetujuan human-in-the-loop atas tindakan agent; halaman mencantumkan OpenAI Build Week, bukan 3rd-Web-Hack. | **Project event lain** |
| RangeShield AI | [Devpost](https://devpost.com/software/rangeshield-ai) | Streaming telemetry dan prediksi range kendaraan listrik dengan Confluent/Google Cloud; halaman `Submitted to` kosong. | **Portfolio project, bukan submission terverifikasi** |

Keanggotaan seseorang pada [halaman peserta](https://3rd-web-hack.devpost.com/participants)
hanya membuktikan bahwa akun tersebut mengikuti hackathon. Itu belum membuktikan
bahwa project tertentu sudah disubmit. Karena itu lead di atas dipisahkan dari
lima kandidat yang mempunyai jejak submission publik.

## Perbandingan dengan MemoryLineage

| Area | Kandidat terkuat | Implikasi untuk MemoryLineage |
| --- | --- | --- |
| Web3 problem fit | Forkline, Kinetic, FinalityDesk | MemoryLineage harus menunjukkan failure mode on-chain yang tidak selesai dengan database biasa |
| Visual presentation | ArcLight AI dan Forkline | MemoryLineage perlu dashboard, timeline, dan demo replay yang polished |
| Demo story | Forkline | MemoryLineage perlu satu failure story yang langsung dipahami dalam 30 detik |
| Public source evidence | Forkline dan FinalityDesk | README, setup, tests, video, dan evidence links harus dapat direplay |
| Narrow verification product | FinalityDesk | MemoryLineage perlu menjelaskan pembeda: continuity/private state lineage, bukan hanya transaction check |
| Publicly documented limitations | Forkline dan FinalityDesk | MemoryLineage perlu menyatakan bahwa lineage membuktikan continuity/authorization, bukan semantic truth |

Artefak lokal MemoryLineage yang relevan untuk pembandingan:

- [MemoryLineageRegistry.sol](<../../contracts/solidity/MemoryLineageRegistry.sol:1>)
- [EVM mutation matrix](<../../evidence/generated/mutation_matrix.json:1>)
- [Sepolia reread evidence](<../../evidence/sepolia/sepolia_reread.json:1>)
- [Independent EVM replay artifact](<../../evidence/generated/memory_lineage_evm_replay.json:1>)

Artefak tersebut menunjukkan jalur bukti teknis internal yang lebih konkret,
tetapi tidak menggantikan kebutuhan UI, video, README submission, dan user
walkthrough.

## Kesimpulan audit

Tidak ada kandidat publik yang saat ini terbukti sekaligus unggul pada semua
criteria. **Forkline** adalah kompetitor terkuat secara keseluruhan karena
menggabungkan problem Web3 konkret, source, local-EVM evidence, live replay,
video, dan batasan yang cukup jujur. **FinalityDesk** paling dekat dengan pola
verification/evidence yang dapat dibandingkan dengan MemoryLineage. **Kinetic**
masih paling kuat sebagai ide decentralized compute, tetapi bukti source dan
demo publiknya tidak tersedia. **ArcLight** unggul pada visual packaging, dan
**DEDSEC** pada storytelling, tetapi keduanya belum menunjukkan blockchain yang
bermakna.

Catatan ini harus dibaca sebagai **competitor evidence ledger**, bukan leaderboard
final. Setelah pencarian lanjutan, jumlah kandidat yang dapat diaudit sebagai
submission publik tetap **lima**; jumlah lead yang sengaja dikeluarkan dari skor
menjadi **delapan**. Audit lengkap baru dapat dilakukan setelah gallery
dipublikasikan atau tersedia daftar submission yang dapat dibuka tanpa login.
