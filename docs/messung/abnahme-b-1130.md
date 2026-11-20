# Abnahme B-1130 Teil 1: Szenen, Pixelmessung, Profil go (RU64.5)

2026-11-19, STIRA-PC, Second Worker, Commit eb9784dc. Teil 2 (Frame-Zeit, `alloc_je_bild`, Workspace-Tests): RU64.6. Szenen und Pixelwerte: `docs/messung/look-k2c.md`.

## Befehle und Ergebnis

| Lauf | Befehl | Ergebnis |
|---|---|---|
| Aufnahmen k2c | `K3C_LOOK=k2c k3c-game.exe --seed 1 --players N --input … --tick T --shots … --size 1920x1080` (Tag, Abend, Nacht, Morgengrauen, 1 und 2 Ansichten) | 8 Aufnahmen, Messung in `look-k2c.md` |
| Bildaufnahme Profil go | `K3C_LOOK=go K3C_RUST_EXE=… cargo test --release -p k3c-game --test e2e_images` | grün, 14 s, Lauf a und b pixelgleich |
| Bildvergleich Profil go | `K3C_LOOK=go … --test e2e_compare -- --nocapture` | Test grün (meldet nur), **14 von 14 Stationen über der Schwelle**, Werte wie `docs/messung/rust-bildvergleich.md` (Mittel 1,9 bis 11,7; Anteil 0,013 bis 0,146) |
| Zum Vergleich Profil k2c | `K3C_LOOK=k2c … e2e_images` und `e2e_compare` | 14 von 14 über der Schwelle, deutlich schlechter in den Weltstationen: 04 muenzen 37,3, 05 35,8, 06 32,4, 07 36,0, 08 nacht 15,1, 11 weiter 15,5, 12 15,4, 13 13,9 (Mittel) |
| testdata | `git status` | unverändert |

**Wichtig für B-1120:** Das Spiel baut `k3c-data` mit dem Feature `go-default` (`rust/crates/k3c-game/Cargo.toml`); ohne `K3C_LOOK` gilt also das Profil `go`. Die Zahlen in `rust-bildvergleich.md` (RU64.1) sind damit Profil `go`, wie hier bestätigt (gleiche Werte).

## Für den Hauptrechner

- AC-02 (Profil `go` grün) ist **nicht erfüllt**: alle 14 Stationen über der Schwelle, auch im Referenzprofil; k2c weicht in den Weltstationen noch stärker ab (erwartet, anderes Layout).
- Look k2c (Hub Tag, Abend, Nacht): Gehlinie 57,7 bis 57,8 %, Ufer 7,0 bis 7,6 %, Fluss 34,7 bis 35,1 % im Ziel. Spiegel-Helligkeit Tag 74 % (Ziel 60 bis 70 %); M (Reiterhöhe) nur gesichtet, ca. 15 % (Ziel 12 %), ungeprüft.
- Nicht aufnehmbar ohne Dev-Menü oder neuen Schalter: Blutmond, dichter Wald, gerodeter Hub, Lava-Stufe (Ticket-Vorschlag: `--dev-event`, `--phase` o. ä. für Aufnahmen, Entscheidung beim Hauptrechner).
- Morgengrauen: Kantenmessung unsicher (schwacher Kontrast); Messmethode für dunkle Szenen offen.

## Prüfliste für Ralf (AC-04, am TV)

Nacht, Blutmond, Wald und Hub wirken wie das K2C-Vorbild; Weite 4, 4,5 oder 5 M wählen (Default 5 M, B-1045).
Dazu (Teil 2): Frame-Zeit am Mini-PC prüfen (`messen.cmd -Engine rustgame`), schlechtester Frame unter 16,7 ms.

## Teil 2: Frame-Zeit und Allokationen (RU64.6)

2026-11-19, STIRA-PC (AMD Ryzen 7 5800X, 32 GB RAM, Radeon RX 6700 XT; kein Mini-PC), Commit 440dfd48, Rust-Release-EXE `dist/k3c-game` (wgpu 30.0.2), Fenster 1920 × 1080, 2 Spieler.

Befehl: `powershell -NoProfile -ExecutionPolicy Bypass -File tools\measure\measure.ps1 -Engine rustgame -Runs 5 -Frames` (je Szene 5 Läufe zu 20 s nach 5 s Aufwärmen; 4 min 37 s; Bericht `reports/native-rustgame-STIRA-PC-20261119-224927.json`, ignoriert, nicht eingecheckt). Der Bericht meldet `vsync: false`, die Bilder laufen trotzdem mit 60 FPS (Takt 16,7 ms); die Frame-Zeit ist damit Taktung, nicht Rechenzeit (Rechenzeit je Bild 0,25 bis 0,27 ms).

| Szene (2 Ansichten) | FPS Mittel / 1-%-Low | Frame p50 (Median, Min, Max der 5 Läufe) | p99 (Median) | Frame max je Lauf (Median, Min, Max) | Takt max | Start | RAM / GPU |
|---|---|---|---|---|---|---|---|
| tag | 60,0 / 57,0 | 16,75 ms (16,70 bis 16,76) | 17,38 ms | 17,84 ms (17,71 bis **18,21**) | 0,88 ms | 210 ms | 351 / 534 MiB |
| nacht | 60,0 / 55,9 | 16,72 ms (16,71 bis 16,74) | 17,50 ms | 20,76 ms (20,69 bis **21,71**) | 4,88 ms | 214 ms | 353 / 534 MiB |

Schlechtester Frame (zwei Ansichten): **18,2 ms (tag), 21,7 ms (nacht)**, also über 16,7 ms. Nur berichtet, kein Gate (Messungen zum Schluss). Go-Referenz (`docs/messung/go-referenz.json`, Szene tag/nacht, 1028 × 749, ohne VSync): Frame p50 0,30 / 0,43 ms, schlechtester 5,0 / 7,4 ms, Start 1298 / 2102 ms, RAM 371 / 331 MiB, GPU 162 MiB, EXE 25,1 MiB; mit VSync p50 78 ms (13 FPS). Rust: Start 210 ms (schneller), GPU 534 MiB (mehr), EXE 19,3 MiB. Die Werte sind wegen der 60-FPS-Taktung und der anderen Fenstergröße nicht direkt vergleichbar.

| Lauf | Befehl | Ergebnis |
|---|---|---|
| `alloc_je_bild` (mit Grafik) | `cargo test --release -p k3c-game --test alloc_je_bild -- --nocapture` | grün, 0,2 s: 179 Bilder, Mittel 464,9, Maximum **786** (step 786, draw 19), Obergrenze 786 unverändert |
| Workspace | Zitat RU64.3 (gleicher Code, seither nur Docs) | 5 rot (still mittag, still_hud 2, still_props 1, `worlds_tests` rom.json), 2668 grün |
| `check:race` | Zitat RU64.2 (`abnahme-b-1123.md`) | keine Data Races; nur `engine/internal/detvec` rot |

### Für den Hauptrechner (Teil 2)

- Schlechtester Frame nacht 21,7 ms (Takt max 4,88 ms in der Nacht, gegen 0,88 ms am Tag): Nachtszene prüfen (RU43), nur gemessen, nicht optimiert.
- Rust läuft trotz `vsync: false` im Bericht mit 60 FPS: Ob der Schalter `--novsync` im wgpu-Fenster wirkt, ungeprüft (Go lief mit über 3000 FPS).
- Mini-PC (🧑): „schlechtester Frame unter 16,7 ms“ bleibt offen.
