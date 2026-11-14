# ST2.2 · Stresstest-Lauf mit Leistungsanzeige und Bericht in reports/

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/st2
- **Abhängig von:** ST2.1
- **Tickets:** B-896
- **Kriterien:** AC-02, AC-03

## Ziel

`game` erfüllt `view.StressStarter`: startet ein Spiel mit den Werten, schaltet F2 und F10 an, führt Ausbau, Truppen, Zeitraffer und zusätzliche Gegner je Welle über Dev-Aktionen aus, misst und schreibt am Ende `reports/native-stress-*.json`.

## Kontext

- Schleife `native/k3c-game/game/game.go` › `advance`; Dev-Aktionen über `view.DevSession.Dev` (Raum: `spawn`, `buildAll`, `timescale`, `wave`).
- Wellen erkennt `game` am Ereignis `wave` der Simulation in `Frame.Events`.
- Rechenzeit je Takt: `Frame.StepMs`; Bildrate `ebiten.ActualFPS`; Speicher `runtime.ReadMemStats`.
- Berichte: Ordner `view.ReportsDir`, Vorbild `devtools/report.go`.

## Erlaubte Dateien

- `native/k3c-game/game/**`
- `native/k3c-game/view/**`

## Nicht-Ziele

- Änderungen an `engine/room`, `engine/sim`.
- Automatische Läufe (`sim_test`).

## Schritte

1. `game/stress.go`: Rolle `StartStress` (Session-Hülle für die Menüs), Vorspann (Ausbau, Truppen, Zeitraffer), Wellen-Gegner, Abbruch nach Wellen oder Dauer.
2. `game/stressreport.go`: Messwerte und Bericht (Takte/s, Taktdauer p50/p95/max, FPS, Objekte, Speicher).
3. Tests mit Schein-Session, `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-02: Go-Test: Konfiguration → `NewGame` (Seed, Tiefe, Geräte) und Dev-Aktionen (buildAll, spawn Truppen, timescale, spawn Gegner je Welle nach Kurve).
- [ ] AC-03: Go-Test: nach n Wellen liegt ein Bericht mit den Leistungswerten im Berichtsordner.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-02 geprüft:** `game/stress_test.go` › `TestStressStartsWithConfig`: `NewGame` mit Seed, Tiefe, Spielstand `stresstest` und Geräten, F2 und F10 an, im ersten Takt `buildAll`, `spawn troop`, `timescale`, zu jeder Welle `spawn enemy` nach der Kurve (10, dann 15); `TestStressEnemiesFor` (linear, exponentiell, Deckel 200).
- **AC-03 geprüft:** `TestStressWritesReport`: nach der letzten geschlagenen Welle liegt `native-stress-*.json` im Berichtsordner (Takte, Takte/s, Taktdauer p50/p95/p99/max aus `Frame.StepMs`, FPS Mittel und Minimum, Höchst- und Endwerte der Objekte, Go-Speicher), die Simulation steht (F7 weiter). `TestStressEndsOnLeaveAndNeedsDev`: Verlassen schreibt den Bericht ohne Halt; ohne Entwickler-Rolle kein Start.
- Umsetzung: `game/stress.go` (Rolle `view.StressStarter` über die Session-Hülle `menuSession` der Menüs, Vorspann, Wellen, Ende), `game/stressreport.go` (Messwerte, Bericht). Spieler ohne Gerät bekommen Platzhalter-Pads (Platz wartet auf ein Gerät). Ende-Meldung als Banner („Stresstest beendet: <Pfad>“).
- `task check:diff` grün.
