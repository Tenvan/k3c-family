# RS1 · SRV, INF, SIM · Messen, Go-Fixes, Rust-Spike für den Simulationskern

- **Status:** aktiv
- **Projekt:** RST
- **Domäne:** SRV, INF, SIM
- **Reife:** bereit
- **Tickets:** B-725
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, Chat (Ralf), Revision 1, durch 🧑; umfasst B-725 Revision 1

## Ausgangslage

Entscheidung 006 (angenommen 2026-11-07): Der Simulationskern wird Rust, der Server bleibt Go – nach Messung, billigen Go-Fixes und einem Spike mit Abbruchkriterium. Erstmessung: Windows-Timer ist nicht die Ursache, der Datei-Watch kostet 150 ms je Sekunde, Takt-Ausreißer unter Volllast kommen vom OS. Pi 3 liegt mit Tick p99 8,0–10,3 ms am Ziel < 10 ms (B-042). Details: B-725, `docs/decisions/006-rust-engine.md`.

## Ziel

Nach dem Sprint steht mit Zahlen fest, ob der Simulationskern voll nach Rust wandert, und die billigen Go-Gewinne sind schon drin.

Am Ende sichtbar: Baseline-Tabelle in 006 (PC und Pi), leerer Server unter 1 % CPU, `task rust:build` baut die Kern-Bibliothek für Windows und arm64, `rng` und `level` aus Rust treffen die Golden-Dateien, 006 nennt das Spike-Ergebnis.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet über den vollen Umbau und misst auf dem Pi 3 (RS1.2). Agenten setzen die übrigen Sessions um. Nutzen: Spieleabende auf dem Pi, alle Balancing- und Sim-Läufe.

## Anforderungen

Siehe `B-725 › Anforderungen`. Sprint-eigen: Jede Messung läuft vor und nach einer Änderung mit demselben Aufbau und steht als Tabelle in 006 › Messbefunde; parallele Agenten während einer Messung werden notiert.

## Nicht-Ziele

- Voller Umbau von `engine/sim` nach Rust (Folgesprint, nur bei bestandenem Spike).
- Server, Lasttest, TUI, Workbench in Rust (006, Varianten 2–4).
- Client-Leistung (Projekt LST).

## Regeln und Einschränkungen

Entscheidung 001 und 006. Lange Läufe nur über `sim_test`/`check_run`, Dienste über `svc_*`. Determinismus: nur `engine/rng` bzw. sein Rust-Gegenstück würfelt, Golden-Dateien werden nicht neu geschrieben. Keine cgo-Pflicht (lokal kein C-Compiler). `engine/` importiert nichts aus `tools/`. Datei ≤ 400, Funktion ≤ 60 Zeilen, auch in Rust.

## Beispiele

- `sim_test` Meilenstein-Lauf (coop2, standard, 40 Tage) vor RS1.3 und nach RS1.3 → Dauer und Ticks/s in 006.
- `rng` in Rust mit Seed aus `testdata/golden/rng.json` → dieselben Zahlen in derselben Reihenfolge.

## Ausnahme- und Fehlerfälle

- Rust weicht von einer Golden-Datei ab → Spike nicht bestanden, nichts neu schreiben.
- Brückenkosten > 20 % der Tick-Dauer oder < 2× Durchsatz auf PC und Pi → 006 auf „Go optimieren“ ändern, Projekt RST endet.
- Pi nicht verfügbar → RS1.2 blockiert, RS1.6 entscheidet nur mit PC-Zahlen und nennt das.

## Akzeptanzkriterien

- **AC-01** Baseline auf dem PC (pprof CPU/Heap Sim und Raum, gctrace, mutex-Profil, `sim_test` perf online, `sim_run`-Ticks/s, Meilenstein-Lauf-Dauer) steht mit Zahlen in 006 (`B-725/AC-01`).
- **AC-02** Baseline auf dem Pi 3: Tick p99 bei 2 Räumen × 3 Spielern und `sim_run`-Ticks/s stehen in 006 (`B-725/AC-01`).
- **AC-03** Der leere Spielserver braucht höchstens 1 % eines Kerns; der Datei-Watch erzeugt im Leerlauf keine Scan-Allokationen je Sekunde (`B-725/AC-02`).
- **AC-04** Raum-Takt bei 4 Spielern: Tick-Abstand p99 ≤ 40 ms auf dem PC ohne parallele Agenten (`B-725/AC-03`).
- **AC-05** `task rust:build` baut die Kern-Bibliothek für windows/amd64 und linux/arm64; Go lädt sie ohne cgo; die CI baut und testet beides.
- **AC-06** `rng` und `level` aus Rust erzeugen alle betroffenen Golden-Dateien bitgleich auf amd64 und arm64 (`B-725/AC-04`).
- **AC-07** Ein heißer Sim-Pfad ist als Rust-Spike gegen optimiertes Go auf PC und Pi gemessen; 006 nennt mit Zahlen, ob der Kern voll umgestellt wird (`B-725/AC-05`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RS1.1 | `RS1.1-baseline-pc.md` | Umsetzung | autonom | fertig |
| RS1.2 | `RS1.2-baseline-pi.md` | Umsetzung | Mensch | offen |
| RS1.3 | `RS1.3-go-fixes.md` | Umsetzung | autonom | fertig |
| RS1.4 | `RS1.4-rust-toolchain-bruecke.md` | Umsetzung | autonom | fertig |
| RS1.5 | `RS1.5-rng-level-rust.md` | Umsetzung | autonom | fertig |
| RS1.6 | `RS1.6-spike-sim-pfad.md` | Umsetzung | autonom | fertig |
| RS1.7 | `RS1.7-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
