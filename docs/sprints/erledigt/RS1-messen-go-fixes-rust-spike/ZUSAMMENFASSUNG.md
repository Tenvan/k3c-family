# RS1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Messen, Go-Fixes, Rust-Spike für den Simulationskern
- **Projekt:** RST

## Ziel

Nach dem Sprint steht mit Zahlen fest, ob der Simulationskern voll nach Rust wandert, und die billigen Go-Gewinne sind schon drin. Am Ende sichtbar: Baseline-Tabelle in 006 (PC und Pi), leerer Server unter 1 % CPU, `task rust:build` baut die Kern-Bibliothek für Windows und arm64, `rng` und `level` aus Rust treffen die Golden-Dateien, 006 nennt das Spike-Ergebnis.

## Ergebnis

2026-11-07 (Claude, autonom): AC-01, AC-03, AC-04, AC-05, AC-06 (amd64), AC-07 (nur PC) siehe Session-Ergebnisse RS1.1–RS1.6; `task check`, `check:go`, `check:diff` grün, `check:race` lokal ohne gcc übersprungen, `rust:test`/clippy grün. Offen (Ralf): AC-02 Pi-Baseline (RS1.2), AC-06 arm64 und CI-Jobs `rust`/`go-arm64`/`docker` laufen erst nach Push auf develop (nicht im PR), AC-04 am Benchmark gemessen, Entscheidung Vollumbau (Empfehlung 006: nein, zuerst B-731). Behoben: Review-Befunde FFI (escape, Null-Guards, MAX_CHUNKS), Fehlertext Level. Neue Tickets: B-731.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Baseline auf dem PC (pprof CPU/Heap Sim und Raum, gctrace, mutex-Profil, `sim_test` perf o … |
| AC-02 | – | Baseline auf dem Pi 3: Tick p99 bei 2 Räumen × 3 Spielern und `sim_run`-Ticks/s stehen in … |
| AC-03 | – | Der leere Spielserver braucht höchstens 1 % eines Kerns; der Datei-Watch erzeugt im Leerla … |
| AC-04 | – | Raum-Takt bei 4 Spielern: Tick-Abstand p99 ≤ 40 ms auf dem PC ohne parallele Agenten (`B-7 … |
| AC-05 | – | `task rust:build` baut die Kern-Bibliothek für windows/amd64 und linux/arm64; Go lädt sie … |
| AC-06 | – | `rng` und `level` aus Rust erzeugen alle betroffenen Golden-Dateien bitgleich auf amd64 un … |
| AC-07 | teilweise | Ein heißer Sim-Pfad ist als Rust-Spike gegen optimiertes Go auf PC und Pi gemessen; 006 ne … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-725 | verworfen | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-06 –, AC-07 teilweise | Simulationskern läuft schneller in Rust, Server bleibt Go (Machbarkeit und Umsti … |
