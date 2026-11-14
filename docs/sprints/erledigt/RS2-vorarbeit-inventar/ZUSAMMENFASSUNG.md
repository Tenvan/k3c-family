# RS2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Etappe 2 vorbereiten: Workbench entkoppeln, Zustand ohne JSON-Roundtrip, Inventar
- **Projekt:** RST

## Ziel

Nach dem Sprint ist die Workbench sprachneutral gegenüber der Engine, der Go-Raum-Tick ohne JSON-Roundtrip gemessen, die Pi-Zahl nach B-731 bekannt und der Rust-Server als Entwurf (007) mit Inventar beschrieben. Ralf kann an Gate G1 entscheiden, ob RS3 (Orakel und Skelett) startet. Am Ende sichtbar: `go list -deps ./tools/k3c-workbench/...` ohne `engine/sim`, `bin/k3c-sim`, Benchmark-Tabelle in 00 …

## Ergebnis

2026-11-08, Review RS2.6 (autonom, Claude): AC-01 bis AC-04 geprüft (Nachweise in RS2.1–RS2.4 und RS2.6), `task check` und `task check:go` grün. AC-05 angenommen, Validierung offen (RS2.5, Pi-Messung und Gate G1 durch Ralf); 007 bleibt Entwurf bis zur Freigabe durch Ralf. Behobene Befunde: keine schweren. Neue Tickets: B-743, B-744 (Review), B-745 (Abnahme-Läufe Sim, Balance, Gen, Race nach B-742).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Workbench ohne `engine/sim`, `room`, `net`, `level`; `sim_run`, `sim_test` offline, `level … |
| AC-02 | geprüft | `BenchmarkTickDevices4` ≥ 3× schneller, allocs/op ≤ 25 %, `sim_test` online perf nicht sch … |
| AC-03 | geprüft | Snapshot/Delta byte-gleich zum Orakel, Protokolltests und Golden unverändert, Reflection-T … |
| AC-04 | geprüft | Inventar und Entscheidung 007 (Entwurf) liegen vor (`B-736/AC-03`). |
| AC-05 | – | Pi-3-Baseline nach B-731 steht in 006, Gate G1 von Ralf bewertet (`B-736/AC-04`, `AC-09`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-736 | verworfen | AC-01 geprüft, AC-02 geprüft, AC-04 geprüft, AC-05 – | Etappe 2: Server und Simulation laufen komplett in Rust, Go-Server und Brücke en … |
| B-726 | verworfen | AC-01 geprüft | Workbench ruft Sim und Balancing als Task auf statt engine/sim zu importieren |
| B-731 | verworfen | AC-02 geprüft, AC-03 geprüft, AC-05 – | Zustands-Snapshot ohne JSON-Roundtrip (stateOf) – 98 % der Raum-Tick-Zeit |
