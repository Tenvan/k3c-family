# RS2.3 · B-731 Stufe 1: Zustand direkt bauen, Orakel-Test

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** sprint/rs2
- **Abhängig von:** –
- **Tickets:** B-731
- **Kriterien:** AC-02, AC-03

## Ziel

`net.stateOf` baut den Zustand direkt aus `sim.World` und `sim.EconomyOf` ohne JSON-Roundtrip; Snapshot- und Delta-JSON bleiben byte-gleich zum alten Weg, der als Test-Orakel erhalten bleibt. Gemessen vorher/nachher nach `docs/messung/rst-baseline.md`, Zahlen in 006.

## Kontext

B-731 Stufe 1 (Spec-Entwurf, wird mit dem Sprint freigegeben). Befund RS1.6: 97,7 % von `Room.Tick` liegen in `pushStatesSaved` → `stateOf` (`engine/net/protocol.go:158`), 1,9 % in `sim.StepIsland`; Baseline `BenchmarkTickDevices4` 1,1–1,8 ms, 5 571 allocs/op. Der Zustand wird je Stufe einmal gebaut und von allen Geräten der Stufe geteilt (B-276). `deltaOf` (`engine/net/delta.go`) vergleicht Maps mit `reflect.DeepEqual`. Stufe 2 (typisierte Snapshots, eigener Encoder) nur, wenn AC-01 verfehlt wird. Zweck im Projekt: Das ist der schnelle Go-Referenzweg für Etappe 2 (B-736, Gate G1) und liefert Struktur und Feldliste für das Inventar (RS2.4). Fallstricke: Zahlen als `float64`, Schlüssel sortiert, `null`, Auslassen von `events`/`eventsDropped`/`unset` wie in `delta.go`; Zeitraffer (`devTimescale`/`devPaused`), `saved`-Ereignis, Vorschau (`preview.go`).

## Erlaubte Dateien

- `engine/net/protocol.go`, `engine/net/state.go` (neu), `engine/net/delta.go`
- `engine/net/*_test.go`, `engine/net/*_bench_test.go`
- `docs/decisions/006-rust-engine.md` (Zahlen unter »Go-Fixes«)
- `docs/messung/rst-baseline.md`

## Nicht-Ziele

Kein Rust, keine Protokolländerung, keine Änderung an `engine/sim`, kein Binary-Format, kein Client, kein Umbau der Spielstand-Kodierung. Stufe 2 nur nach Messung und Rücksprache im Ergebnis.

## Schritte

1. Baseline `BenchmarkTickDevices4` (`-benchtime 100000x`) messen und notieren; Last (`LoadPercentage`) notieren.
2. Orakel sichern: bisherigen Weg als `stateOfJSON` in `_test.go` verschieben.
3. Vergleichstest zuerst (rot): alle sechs Biome, feste Seeds, ≥ 600 Ticks mit 2 Spielern, Zeitraffer, `saved`, Game Over; Snapshot und Delta byte-gleich.
4. Reflection-Test: jedes exportierte `sim.World`-Feld mit JSON-Tag erscheint im Zustand oder steht in der Ausnahmeliste (`seed`, `widthUnits`, `free`).
5. `stateOf` handgeschrieben bauen (`state.go`), Funktion ≤ 60 Zeilen, Datei ≤ 400 Zeilen; Teilfunktionen je Bereich (Spieler, Gegner, Gebäude, Wirtschaft, Plätze).
6. Messen nach `docs/messung/rst-baseline.md`; Ergebnis in 006 › Go-Fixes. Wird AC-01 verfehlt: Stufe 2 als Ergebnis melden (Spec ändert sich nicht still).
7. `sim_test` online (`perf`, 4 Geräte) laufen lassen: Tick-p99 nicht schlechter.

## Fertig, wenn

- [ ] B-731/AC-01: `BenchmarkTickDevices4` mindestens 3× schneller, allocs/op ≤ 25 % (Zahlen in 006).
- [ ] B-731/AC-02: Vergleichstest Snapshot/Delta byte-gleich grün.
- [ ] B-731/AC-03: Protokolltests und Golden unverändert grün, `docs/protocol.md` unverändert.
- [ ] B-731/AC-04: Reflection-Test grün.
- [ ] B-731/AC-05: `sim_test` online perf, Tick-p99 nicht schlechter.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Domäne SRV mit Protokollnähe: Reviewer-Agent (Sonnet, getrennt) vor dem Commit im Hintergrund; `task check:race` nur im Review.

## Ergebnis

- **B-731/AC-01 geprüft:** `BenchmarkTickDevices4` (100 000x) vorher 894–914 µs, 5 647 allocs/op; nachher 215–225 µs, 1 229 allocs/op (4,0–4,2×, 21,8 %). Vorher = `HEAD` als Kopie im Scratchpad, beide auf demselben PC nacheinander; Last 91 % vor dem ersten Paar (parallele Agenten), danach 2–65 %. Tabelle in 006 › Go-Fixes (RS2.3).
- **B-731/AC-02 geprüft:** `TestStateGleichOrakel*` (alle Biome × 660 Takte mit 2 Spielern, Zeitraffer/Pause, `saved`, Game Over; Bosswelle; alle Golden-Läufe; leere Stufe; Vorschau) vergleicht Map, Snapshot- und Delta-Bytes gegen `stateOfJSON`, grün.
- **B-731/AC-03 geprüft:** `go:test` grün, in `task check:diff` alle Go-Schritte (go test, golangci-lint, Workbench) grün; Golden und `docs/protocol.md` unverändert. Rot nur `tests/planning.test.ts` wegen fremder Tickets ohne Projekt (B-740, B-741 aus RS2.2, B-742 aus paralleler Session), nicht Teil dieser Session.
- **B-731/AC-04 geprüft:** `TestStateReflection*` (Welt in drei Füll-Arten, jedes JSON-Feld im Zustand oder in `seed`/`widthUnits`/`free`, Wirtschaft) grün; nach Review bricht `fillScalar` bei unbekannter Feld-Art ab.
- **B-731/AC-05 geprüft:** `sim_test` online perf 1 × 4, 5 min (run-28): Tick p99 max 2,5 ms, gleich RS1.3.
- Abweichungen: Session von einem zweiten Agenten fertiggestellt (erster Agent hatte Code und Tests geschrieben); Stufe 2 nicht nötig. Code-Review (Sonnet): keine schweren Befunde. Keine neuen Tickets.
