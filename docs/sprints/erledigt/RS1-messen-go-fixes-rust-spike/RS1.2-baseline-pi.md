# RS1.2 · Baseline auf dem Pi 3 messen

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** rs1/2-baseline-pi
- **Abhängig von:** –
- **Tickets:** B-725
- **Kriterien:** AC-02

## Ziel

Die Pi-3-Baseline (Tick p99 bei 2 Räumen × 3 Spielern, `sim_run`-Ticks/s) steht in 006 › Messbefunde.

## Kontext

Pi-Ziel aus B-042: Pi 3 (64-Bit-OS), 2 Räume × 3 Spieler, Tick-Dauer p99 < 10 ms bei 30 Hz. Handmessung 2026-10-03: Tag 8,0–9,5 ms, Nacht 10,2–10,3 ms. Docker-Image für arm64 baut die CI (`.github/workflows/ci.yml`, Job docker). Lasttest-Werkzeug `cmd/k3c-load` (B-175). Ralf (🧑) stellt den Pi bereit und startet den Server; Claude führt die Last und trägt Zahlen ein.

## Erlaubte Dateien

- `docs/decisions/006-rust-engine.md` (nur Abschnitt Messbefunde)
- `docs/messung/rst-baseline.md` (Abschnitt Pi)

## Nicht-Ziele

Keine Einrichtung des Pi (B-035), keine Code-Änderung.

## Schritte

1. **Ralf:** aktuellen Server-Build (arm64) auf dem Pi 3 starten, Adresse nennen.
2. **Claude:** Lasttest 2 Räume × 3 Spieler, 10 min, Tag und Nacht; Tick-Dauer p99 und Tick-Abstand p99 aus `/api/status`.
3. **Claude:** Durchsatz der Sim auf dem Pi (Benchmark-Binary `engine/sim` für arm64 cross-gebaut, `-bench IslandStep`).
4. **Claude:** Zahlen und Aufbau in 006 und im Mess-Rezept.

## Fertig, wenn

- [ ] AC-02: 006 enthält Pi-Werte (Tick-Dauer p99 Tag/Nacht, Tick-Abstand p99, Sim-Benchmark ns/op) mit Datum und Build.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Verworfen am 2026-11-11 (🧑, Chat): Entscheidung 008, das Spiel wird ein lokaler Monolith in Go + Ebitengine ohne Server; die Pi-Messung ist gegenstandslos.
