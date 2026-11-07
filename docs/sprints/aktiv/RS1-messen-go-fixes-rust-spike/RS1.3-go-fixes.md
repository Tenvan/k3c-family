# RS1.3 · Go-Fixes: Datei-Watch, Priorität, Tick-Allokationen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** rs1/3-go-fixes
- **Abhängig von:** RS1.1
- **Tickets:** B-725
- **Kriterien:** AC-03, AC-04

## Ziel

Die billigen Go-Gewinne sind drin: leerer Server < 1 % CPU, stabiler Takt, weniger Allokationen im Tick; die Messung aus RS1.1 ist wiederholt und in 006 gegenübergestellt.

## Kontext

Befunde aus 006: Datei-Watch `engine/watch/watch.go` scannt jede Sekunde ~1900 Dateien (`public/`, `data/`) per `fs.WalkDir` und baut jedes Mal eine neue Map (150 ms je Scan); gestartet in `cmd/k3c-server/restart.go` (`watchEvery`). Entscheidung 004 verlangt den Watch auch produktiv. Takt in `engine/room/run.go`. Windows drosselt Hintergrundprozesse (EcoQoS) und verteilt auf P-/E-Kerne der Hybrid-CPU. Die Top-Allokationen im Tick nennt das Profil aus RS1.1.

## Erlaubte Dateien

- `engine/watch/**`
- `cmd/k3c-server/**`
- `engine/room/run.go`, `engine/room/room.go`
- Dateien in `engine/sim/**`, `engine/net/**` nur für die drei größten Allokationen aus dem RS1.1-Profil
- `docs/decisions/006-rust-engine.md` (Messbefunde)

## Nicht-Ziele

Kein Rust, keine Regeländerung, keine Golden-Änderung, kein neuer Tick-Scheduler-Umbau ohne Messbeleg.

## Schritte

1. Datei-Watch: Ereignis-basiert (Windows `ReadDirectoryChangesW`, Linux inotify) mit Polling als Rückfall, oder Scan nur auf geänderte Ordner-Zeitstempel; keine Map-Neuanlage je Takt.
2. Server-Prozess: Power-Throttling (EcoQoS) unter Windows abschalten, Priorität „above normal“ für das Kind; auf Linux nichts.
3. Die drei größten Tick-Allokationen aus RS1.1 entfernen (Puffer wiederverwenden).
4. Mess-Rezept aus RS1.1 wiederholen, vorher/nachher in 006.

## Fertig, wenn

- [ ] AC-03: leerer Server ≤ 1 % eines Kerns (10 s gemessen), Watch erzeugt im Leerlauf keine Scan-Allokationen (Benchmark mit `-benchmem`).
- [ ] AC-04: Tick-Abstand p99 ≤ 40 ms bei 4 Spielern ohne parallele Agenten.
- [ ] Datei-Watch-Tests grün, Neustart bei geändertem PNG funktioniert weiter.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-03 umgesetzt, gemessen: leerer Server 0,00 % eines Kerns (10 s; vorher 3,3 %), Datei-Watch ereignisgesteuert (fsnotify, `engine/watch/notify.go`), `BenchmarkRunLeerlaufMitWake` 0 allocs/op; Neustart bei geändertem PNG per Hand bestätigt (Kindprozess ersetzt), Watch-Tests grün.
- AC-04 umgesetzt, gemessen: Tick-Abstand p99 35,8–36,0 ms (max 42–45 ms) bei 4 Geräten mit Priorität »über normal« und EcoQoS aus (`cmd/k3c-server/priority_windows.go`), ohne 74–77 ms. Gemessen im Benchmark `TaktAbstand4` mit `K3C_TAKT_TUNE=1` (dieselben Prozess-Einstellungen wie der Server), nicht am laufenden Server; Last 25 %, keine parallelen Agenten im engeren Sinn. Online-Lauf Tick-Dauer p99 max 135,7 → 2,5 ms (Last 68 % → ~30 %, nur Richtung).
- Sim-Fixes (Profil RS1.1): `collectCoins`, `stepSpawns`, `chooseTarget`; Insel-Tick 31,5–42,3 → 20,4–21,5 µs (100 000 Iterationen), 37 → 10 allocs. Golden unverändert, `engine/sim`-Tests grün. Abweichung: `collectCoins` ist CPU- statt Allokationsposten; wegen 77 % der Sim-CPU trotzdem behoben (nur Hochziehen von Werten).
- Vorher/Nachher-Tabelle in 006 › Go-Fixes. Benchmark `SimRun100k` aus RS1.1 hing (ResetTimer in b.Loop), behoben.
- Reviewer (Sonnet): keine schweren Befunde. Geprüft: `task check:diff`.
