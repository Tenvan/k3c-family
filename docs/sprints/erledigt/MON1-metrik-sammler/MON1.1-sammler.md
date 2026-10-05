# MON1.1 · Sammler mit Messreihen je Raum, Gerät und Server

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** mon1/1-sammler
- **Abhängig von:** –
- **Tickets:** B-281
- **Kriterien:** AC-01, AC-03, AC-04, AC-07

## Ziel

Der Server führt je Kennzahl eine Messreihe (Ring-Puffer, 3600 Punkte bei 1 s): Tick-Dauer je Raum, RTT, Warteschlange und verworfene Zustände je Gerät, Heap, GC-Pausen, Goroutinen, CPU und Speicher-Dauer. Ein Benchmark zeigt die Mehrkosten im Tick.

## Kontext

- Tick-Dauer misst heute `Room.safeTick` (`engine/room/run.go`) unter der Raum-Sperre, die letzten 300 Dauern liegen in `Room.durations`; `noteTick` (`engine/room/logging.go`) meldet langsame Ticks (über `slowTick`). `Manager.Run` taktet Räume und einmal je Sekunde den Sweep.
- `engine/room/metrics.go` ist der Spielmetrik-Report (B-150), **nicht** dieser Sammler: neue Datei `engine/room/monitor.go`.
- Warteschlange je Verbindung: `conn.push`/`pop` in `engine/net/ws.go` (`sendBuffer` 64, ein Zustand ersetzt einen wartenden derselben Stufe = verworfener Zustand; volle Warteschlange schließt die Verbindung).
- **RTT-Quelle (Offene Frage B-281):** Die Zeitleiste aus N2 (`ack`) misst nur der Client (Glossar „Latenz“). Der Server misst die RTT selbst per WebSocket-Ping (`websocket.Conn.Ping` von `coder/websocket`, Kontrollframe, Browser antworten selbst; kein Protokollwechsel), einmal je Sekunde je Verbindung in einer eigenen Goroutine, nie unter der Raum-Sperre.
- CPU: `newCPUMeter(procCPUTime)` in `engine/net/cpu.go` (nur Linux); der Sammler braucht einen eigenen Messer, sonst teilt er das Intervall mit `/api/status`.
- Speicher-Dauer: `Room.write` in `engine/room/saver.go` (außerhalb der Raum-Sperre).
- Speicherbudget: alle Puffer < 2 MB bei 4 Räumen × 4 Geräten → kompakte Punkte (`float32`, Zeit als Unix-ms). Reihen, deren letzter Punkt älter als das Fenster ist, fallen weg (kein Wachstum durch neue Raum-Codes).
- Benchmark-Vorlage: `engine/net/tick_bench_test.go` (`BenchmarkTickDevices4`).

## Erlaubte Dateien

- `engine/room/monitor.go`, `engine/room/monitor_test.go` (neu); kleine Eingriffe in `engine/room/run.go`, `room.go`, `manager.go`, `saver.go`
- `engine/net/ping.go`, `engine/net/ping_test.go`, `engine/net/monitor_bench_test.go` (neu); kleine Eingriffe in `engine/net/ws.go`, `engine/net/handler.go` (nur `NewHandler`, keine neue Route)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status, Notizen und neue Tickets)

## Nicht-Ziele

Ereignis-Ring und `/api/metrics` (MON1.2), Seite (MON2), Protokolländerung, Log-Meldungen je Sekunde.

## Schritte

1. `Status: in Arbeit`.
2. `engine/room/monitor.go`: generischer Ring-Puffer fester Größe (älteste fallen raus), `Monitor` mit Reihen für Server, je Raum und je Gerät (Kürzel), `Since(t)`.
3. Im Tick nur ein Zähler (`safeTick`); ein eigener 1-s-Ticker in `Manager.Run` liest je Raum max, p99 und Ticks über Budget des Intervalls aus `durations` sowie Heap, GC-Pause, Goroutinen, CPU und die längste Speicherung.
4. `engine/net/ping.go`: je Verbindung einmal je Sekunde Ping; RTT, Länge der Warteschlange und verworfene Zustände seit dem letzten Punkt in die Reihe des Geräts.
5. Tests: Ring über seine Größe (AC-01), zwei Geräte mit getrennten Reihen (AC-04), Speicherbudget. Benchmark mit und ohne Sammler, Ergebnis in die Notizen von B-281 (AC-03).
6. `task check:go`, `task check`. Ergebnis, `Status: fertig`.

## Fertig, wenn

- [x] AC-01: Test füllt den Ring über seine Größe, Länge bleibt fest, älteste Punkte fallen raus.
- [x] AC-03: Benchmark mit aktivem Sammler höchstens 1 % bzw. 20 µs langsamer, Ergebnis in B-281 › Notizen.
- [x] AC-04: Test mit 2 Geräten liefert zwei getrennte RTT- und Warteschlangen-Reihen.
- [x] AC-07: `task check:go` grün; dazu `task check` grün, keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:go
task check
```

## Ergebnis

- **AC-01** umgesetzt, geprüft: `TestRingFestBegrenzt` (`engine/room/monitor_test.go`) füllt den Ring mit 3610 Punkten, Länge bleibt 3600, die ersten 10 sind raus; `TestReiheEndetMitDemFenster`: Reihen geschlossener Räume und getrennter Geräte fallen nach einem Fenster weg; `TestMonitorSpeicherbudget`: 4 Räume × 4 Geräte < 2 MB.
- **AC-03** umgesetzt, geprüft: `BenchmarkTickMonitor` (`engine/net/monitor_bench_test.go`), Sammler 1000-mal so oft wie im Betrieb: +8 µs im Mittel (406 → 415 µs, im Rauschen), unter 20 µs; Zahlen in B-281 › Notizen. Im Tick nur ein Zähler (`secTicks` in `safeTick`), der Punkt je Sekunde kommt aus einem eigenen Ticker in `Manager.Run` (`Manager.Sample`).
- **AC-04** umgesetzt, geprüft: `TestZweiGeraeteGetrennteReihen` (`engine/net/ping_test.go`): zwei Geräte über echte WebSockets, je eine Reihe mit RTT, Warteschlange, verworfenen Zuständen und Raum.
- **AC-07** geprüft: `task check:go` (0 issues, alle Go-Tests grün; `-race` lokal ohne C-Compiler übersprungen, prüft die CI) und `task check` (1269 Tests) grün.
- RTT-Quelle (Offene Frage B-281): eigener WebSocket-Ping (`engine/net/ping.go`), Begründung in B-281.
- Abweichung: `short` von `engine/net/ws.go` nach `ping.go` verschoben (ws.go sonst über 400 Zeilen). Planung von Hand, weil k3c-dev auf die Repo-Wurzel zeigt.
