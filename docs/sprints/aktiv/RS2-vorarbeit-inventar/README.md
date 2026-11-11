# RS2 · SRV, DEV, INF · Etappe 2 vorbereiten: Workbench entkoppeln, Zustand ohne JSON-Roundtrip, Inventar

- **Status:** aktiv
- **Projekt:** RST
- **Domäne:** SRV, DEV, INF
- **Reife:** bereit
- **Tickets:** B-736, B-726, B-731
- **Start-Commit:** 8d74a196
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, Chat (Ralf), Revision 1, durch 🧑; umfasst B-736, B-731 Revision 1 und B-726 Revision 1

## Ausgangslage

Ralf hat Etappe 2 (Rust-Server, Entscheidung 006 Variante 2) am 2026-11-07 gewählt (B-736). RS1 hat gezeigt: Sim ~2 % der Raum-Tick-Zeit, 97,7 % im JSON-Roundtrip `stateOf`; Spike `collectCoins` blieb auf dem PC unter 2×; Pi-Werte fehlen. Dieser Sprint macht die Vorarbeit, die auch ohne Rust Gewinn bringt und für den Port nötig ist: Workbench entkoppeln (B-726), schneller Go-Zustandsaufbau (B-731, Referenz und Gate G1), Inventar und Entscheidung 007 (Entwurf), Pi-Messung.

## Ziel

Nach dem Sprint ist die Workbench sprachneutral gegenüber der Engine, der Go-Raum-Tick ohne JSON-Roundtrip gemessen, die Pi-Zahl nach B-731 bekannt und der Rust-Server als Entwurf (007) mit Inventar beschrieben. Ralf kann an Gate G1 entscheiden, ob RS3 (Orakel und Skelett) startet.

Am Ende sichtbar: `go list -deps ./tools/k3c-workbench/...` ohne `engine/sim`, `bin/k3c-sim`, Benchmark-Tabelle in 006, `docs/decisions/007-rust-server.md`.

## Beteiligte und Zielgruppen

Ralf (🧑) gibt die Specs frei, misst auf dem Pi 3 (RS2.5) und entscheidet an Gate G1 und über 007. Agenten setzen RS2.1–RS2.4 und das Review um. Nutzen: Spieleabende auf dem Pi, Balancing, alle weiteren RST-Sprints.

## Anforderungen

Siehe `B-736 › Anforderungen` (Vorarbeit), `B-726 › Anforderungen` und `B-731 › Anforderungen`. Sprint-eigen: Jede Messung läuft vor und nach der Änderung mit demselben Aufbau (`docs/messung/rst-baseline.md`) und steht als Tabelle in 006; parallele Agenten während der Messung werden notiert.

## Nicht-Ziele

- Rust-Code für den Server (RS3 und folgende).
- Stufe 2 von B-731 (nur bei verfehltem Ziel und nach Rücksprache).
- Freigabe von 007 und der Folgesprints (Ralf).

## Regeln und Einschränkungen

Entscheidung 001, 004, 006; B-736 › Regeln. Lange Läufe nur über `sim_test`/`check_run`. Sessions teilen sich den Branch `sprint/rs2` (ein Commit je Session). Hardware-Session RS2.5 entkoppelt (Sprint bleibt aktiv, bis sie fertig ist). Reihenfolge: RS2.1 → RS2.2 (DEV), RS2.3 (SRV) und RS2.4 nach RS2.3; gleiche Domäne SRV: RS2.1 und RS2.3 nacheinander, nicht parallel.

## Beispiele

- `sim_test` Meilenstein-Lauf nach RS2.2: Dauer ≤ +5 % gegenüber vorher, Bericht gleich.
- `BenchmarkTickDevices4` nach RS2.3: ≤ 0,5 ms und ≤ 1 400 allocs/op (Baseline 1,1–1,8 ms, 5 571 allocs).

## Ausnahme- und Fehlerfälle

- Byte-Abweichung im Orakel-Test (RS2.3): alter Weg bleibt aktiv, Abweichung erklären, kein Aufweichen des Vergleichs.
- AC-02 verfehlt: Ergebnis nennt Stufe 2 als Folge, Spec ändert sich nicht still.
- Pi nicht verfügbar: AC-05 bleibt `angenommen, Validierung offen (RS2.5)`.

## Akzeptanzkriterien

- **AC-01** Workbench ohne `engine/sim`, `room`, `net`, `level`; `sim_run`, `sim_test` offline, `level_generate` unverändert, Meilenstein-Lauf ≤ +5 % (`B-726/AC-01`, `AC-02`, `AC-03`; `B-736/AC-01`).
- **AC-02** `BenchmarkTickDevices4` ≥ 3× schneller, allocs/op ≤ 25 %, `sim_test` online perf nicht schlechter (`B-731/AC-01`, `AC-05`; `B-736/AC-02`).
- **AC-03** Snapshot/Delta byte-gleich zum Orakel, Protokolltests und Golden unverändert, Reflection-Test grün (`B-731/AC-02`, `AC-03`, `AC-04`).
- **AC-04** Inventar und Entscheidung 007 (Entwurf) liegen vor (`B-736/AC-03`).
- **AC-05** Pi-3-Baseline nach B-731 steht in 006, Gate G1 von Ralf bewertet (`B-736/AC-04`, `AC-09`).

## Offene Fragen

Siehe `B-736 › Offene Fragen` (Frage 1 betrifft den Sprint: G1 bei erfülltem Pi-Ziel). Keine blockierende Frage für die Freigabe von RS2.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RS2.1 | `RS2.1-sim-cli.md` | Umsetzung | autonom | fertig |
| RS2.2 | `RS2.2-workbench-entkoppeln.md` | Umsetzung | autonom | fertig |
| RS2.3 | `RS2.3-zustand-stufe1.md` | Umsetzung | autonom | fertig |
| RS2.4 | `RS2.4-inventar-007.md` | Umsetzung | autonom | fertig |
| RS2.5 | `RS2.5-pi-messung.md` | Umsetzung | Mensch | verworfen |
| RS2.6 | `RS2.6-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-11-08, Review RS2.6 (autonom, Claude): AC-01 bis AC-04 geprüft (Nachweise in RS2.1–RS2.4 und RS2.6), `task check` und `task check:go` grün.
AC-05 angenommen, Validierung offen (RS2.5, Pi-Messung und Gate G1 durch Ralf); 007 bleibt Entwurf bis zur Freigabe durch Ralf.
Behobene Befunde: keine schweren. Neue Tickets: B-743, B-744 (Review), B-745 (Abnahme-Läufe Sim, Balance, Gen, Race nach B-742).
