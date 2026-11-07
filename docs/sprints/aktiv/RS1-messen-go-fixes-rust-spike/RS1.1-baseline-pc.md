# RS1.1 · Baseline auf dem PC messen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** rs1/1-baseline-pc
- **Abhängig von:** –
- **Tickets:** B-725
- **Kriterien:** AC-01

## Ziel

Eine reproduzierbare Baseline für PC steht als Tabelle in `docs/decisions/006-rust-engine.md` › Messbefunde, dazu ein Mess-Rezept, das RS1.3 und RS1.6 unverändert wiederholen.

## Kontext

Entscheidung 006 (Simulationskern wird Rust, erst messen). Erstmessung 2026-11-07 lief unter 100 % CPU durch parallele Agenten; diese Session misst ohne parallele Agenten oder notiert die Last. Tick-Schleife: `engine/room/run.go` (`time.NewTicker(time.Second / TickHz)`, TickHz 30 in `engine/room/room.go`), Tick-Dauern p99 in `Room.tickMs`. Benchmarks: `engine/sim/island_bench_test.go`, `engine/net/tick_bench_test.go`, `engine/net/monitor_bench_test.go`, `engine/room/stages_bench_test.go`, `engine/level/bench_test.go`. Der Server hat kein pprof; Workbench-pprof liegt unter `/debug/pprof/` des MCP-Ports. Datei-Watch: `engine/watch/watch.go` (Scan je Sekunde, 150 ms). Lange Läufe nur über `sim_test`/`sim_run`/`check_run`.

## Erlaubte Dateien

- `docs/decisions/006-rust-engine.md` (nur Abschnitt Messbefunde)
- `docs/messung/rst-baseline.md` (neu: Mess-Rezept)
- `engine/**/*_bench_test.go` (neue Benchmarks, keine Logikänderung)

## Nicht-Ziele

Keine Optimierung (RS1.3), kein Rust, kein pprof-Endpunkt im Server (falls nötig: Ticket).

## Schritte

1. Last am Rechner prüfen (CPU < 20 % ohne Messung), sonst Last notieren.
2. Benchmarks mit `-cpuprofile`/`-memprofile` (Sim, Raum, Level), Top-15 je Profil als Tabelle.
3. `GODEBUG=gctrace=1` bei einem `sim_test` online perf (1 Raum × 4 Spieler, 5 min): GC-Pausen p50/p99, Heap.
4. Mutex-/Block-Profil im Raum-Benchmark (`-mutexprofile`).
5. `sim_run` 100 000 Ticks: Ticks/s; Meilenstein-Lauf `sim_test` (coop2, standard, 40 Tage): Dauer.
6. Tick-Abstand p99 und Tick-Dauer p99 aus dem Online-Lauf.
7. Mess-Rezept nach `docs/messung/rst-baseline.md`, Zahlen nach 006.

## Fertig, wenn

- [ ] AC-01: 006 › Messbefunde enthält PC-Baseline mit pprof-Top, GC-Pausen, Mutex-Wartezeit, `sim_run`-Ticks/s, Meilenstein-Dauer, Tick-Abstand und -Dauer p99.
- [ ] Das Mess-Rezept ist so genau, dass RS1.3 es ohne Rückfrage wiederholt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
