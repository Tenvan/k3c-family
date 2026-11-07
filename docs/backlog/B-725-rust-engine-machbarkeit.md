# B-725 · Simulationskern läuft schneller in Rust, Server bleibt Go (Machbarkeit und Umstiegspfad)

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf meldet Timing-Probleme schon ohne Spiellast auf einem schnellen PC, zu lange Simulationen und Balancing-Läufe und noch größere Welten. Er möchte die Engine nach Rust bringen. Analyse und Varianten stehen im Entwurf `docs/decisions/006-rust-engine.md`. Erstmessung 2026-11-07: Windows-Timer ist nicht die Ursache (Go 1.27 schläft auf ~0,5 ms genau), Takt-Ausreißer kommen vom OS-Scheduler unter 100 % Last, der Datei-Watch kostet 150 ms je Sekunde. Die volle Messung hat Ralf in die Umsetzung verschoben.

## Ziel

Simulation und Balancing laufen messbar schneller, der Raum-Takt bleibt stabil. Rust kommt nur dort hin, wo eine Messung gegen optimiertes Go mindestens 2× Durchsatz zeigt; der Server bleibt Go, bis der Spike entschieden ist.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet über 006 und gibt die Spec frei. Nutzen haben Balancing-Läufe (`sim_test`, `sim_run`), die Spieleabende (Takt) und alle Agenten, die Sims fahren.

## Anforderungen

- Baseline mit pprof CPU/Heap, gctrace, mutex-Profil, `sim_test` perf und `sim_run`-Durchsatz vor jeder Änderung.
- Go-Grundlast senken (Datei-Watch, Prozess-Priorität/EcoQoS) und erneut messen.
- Rust-Spike: `engine/rng` und `engine/level` bitgleich zu `testdata/golden/`, Anbindung als DLL ohne cgo.
- Determinismus bleibt: gleiche Seeds, gleiche Golden-Dateien, 2+ Spieler.

## Nicht-Ziele

- Server, Lasttest, TUI oder Workbench in Rust (Varianten 2–4 in 006), bis der Spike entschieden ist.
- Migration alter Spielstände.
- Client-Leistung (Projekt LST).

## Regeln und Einschränkungen

Entscheidung 001 bleibt gültig (Engine im Server). Lange Läufe nur über `sim_test`/`check_run`. Nur `engine/rng` würfelt; Golden-Dateien sind das Orakel. Lokal fehlt ein C-Compiler, daher keine cgo-Pflicht. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen gilt auch für Rust.

## Beispiele

- `sim_test` Meilenstein-Lauf (coop2, standard, 40 Tage) dauert heute X min → nach RST-M2 messbar kürzer, nach Spike ≥ 2× schneller oder Rust wird verworfen.
- Leerer Spielserver: heute 8,9 % eines Kerns → nach RST-M2 unter 1 %.

## Ausnahme- und Fehlerfälle

- Rust-Ergebnis weicht von einer Golden-Datei ab → Spike gilt als nicht bestanden, kein Golden-Neuschrieb.
- Brückenkosten > 20 % der Tick-Dauer → Variante 1 verworfen.

## Akzeptanzkriterien

- **AC-01** Eine Baseline (pprof CPU/Heap Sim und Raum, gctrace, mutex-Profil, `sim_test` perf online, `sim_run`-Ticks/s) liegt mit Zahlen in 006 vor.
- **AC-02** Der leere Spielserver braucht höchstens 1 % eines Kerns; der Datei-Watch erzeugt im Leerlauf keine Scan-Allokationen je Sekunde.
- **AC-03** Raum-Takt bei 4 Spielern: Tick-Abstand p99 ≤ 40 ms auf dem Entwickler-PC ohne parallele Agenten.
- **AC-04** `engine/rng` und `engine/level` in Rust erzeugen alle betroffenen Dateien in `testdata/golden/` bitgleich; Aufruf aus Go ohne cgo.
- **AC-05** Ein Spike eines heißen Sim-Pfads ist gegen optimiertes Go gemessen; 006 ist danach angenommen oder verworfen mit Zahlen.

## Offene Fragen

- Variante und Empfehlung in 006 annehmen? (🧑 Ralf)
- Ist der Pi (B-042) noch Zielplattform? Davon hängt der FMA-/Determinismus-Aufwand Go↔Rust ab. (🧑 Ralf)

## Notizen

Erstmessung und Codeumfang: `docs/decisions/006-rust-engine.md`. Die Workbench importiert `engine/sim` an 36 Stellen – jede Teil-Umstellung braucht dort dieselbe Brücke.
