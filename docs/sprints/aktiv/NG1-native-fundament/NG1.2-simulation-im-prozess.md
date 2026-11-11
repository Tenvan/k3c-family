# NG1.2 · Simulation im Prozess und Spielstände (local)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng1/2-simulation-im-prozess
- **Abhängig von:** NG1.1
- **Tickets:** B-793
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

Paket `local` führt ein Spiel mit 1–2 lokalen Spielern über `engine/room` im selben Prozess, synchron getaktet, mit Spielständen über `engine/store`, und liefert je Takt einen `view.Frame`.

## Kontext

Spec: B-793. `engine/room`: `Manager` (`NewManager(Store)`, Feld `Snapshot func(*sim.World, int, bool) any`, `Create`, `Join`), `Room.Tick()` und `Room.Input(id, peer, map[int]sim.PlayerCommand)` in `engine/room/actions.go`, `Peer`-Schnittstelle in `engine/room/room.go` (nicht blockieren, nicht zurückrufen), Autosave in `saver.go`, Pause in `pause.go`, Inselwechsel in `island_switch.go`. `Manager.Run` startet je Raum eine Goroutine mit Ticker: hier **nicht** nutzen, sondern `Room.Tick` aus der Spielschleife rufen (prüfen, ob Autosave und Fristen ohne `Run` greifen; sonst Befund als Ticket SRV, nicht `engine/` ändern). Vergleich für den Determinismus-Test: `engine/room/determinism_test.go`. Spielstände: `engine/store/saves.go`. Opus, weil Nebenläufigkeit und Spielstand berührt sind: Reviewer (Opus) parallel zu `task check:diff`.

## Erlaubte Dateien

- `native/k3c-game/local/**`
- Planung NG1, B-793 (über MCP)

## Nicht-Ziele

Änderungen unter `engine/` oder `testdata/`; Online-Spiel, Beobachter, Dev-Aktionen; Menü (NG7); Sim- und Balancing-Läufe.

## Schritte

1. Test zuerst: 2 Spieler, Seed fest, 300 Takte mit festen Befehlen über `local` gegen denselben Ablauf direkt über `engine/room` → gleicher Welt-Hash.
2. Lokaler `Peer` und `Manager` mit `Snapshot`, der `view.Frame` mit `*sim.World` liefert.
3. Synchroner Takt (`Step()` je 1/30 s) und Befehle je Slot.
4. `view.Session` erfüllen: Neues Spiel, Weiter, Laden, Liste, Löschen über `engine/store` mit Ordner aus der Entscheidung 🧑 (Default `saves/` neben der EXE).
5. Tests gegen Temp-Ordner; Fehlerfälle aus B-793.

## Fertig, wenn

- [ ] AC-03: Determinismus-Test grün (gleicher Welt-Hash nach 300 Takten).
- [ ] AC-04: Tests für Speichern, Liste, Laden, Löschen grün; Hash nach Laden gleich.
- [ ] AC-05: `git diff --stat origin/develop -- engine testdata` leer.
- [ ] Review ohne schwere Befunde.

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
