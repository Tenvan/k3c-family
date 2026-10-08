# DBG3.1 · Dev-HTTP-API /api/dev mit Welle und Tageszeit

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** dbg3/1-dev-api
- **Abhängig von:** –
- **Tickets:** B-232
- **Kriterien:** AC-02, AC-03, AC-05

## Ziel

Der Server bietet `/api/dev`: Raumliste, Diagnose eines Raums und Dev-Aktionen ohne WebSocket-Gerät, dazu die neuen
Aktionen „Welle auslösen“ und „Tageszeit setzen“. Dazu liefert er `/dm` als `dm.html` aus.

## Kontext

- Dev-Aktionen: `engine/room/dev.go` (`Room.Dev`, an ein Gerät mit Slot gebunden), Sim-Hilfen in `engine/sim/dev.go`.
- Diagnose: `Room.Summary()` in `engine/room/status.go`, Raumliste `Manager.Rooms()` (öffentlich auch in der Lobby).
- Dev-Mode: `Manager.Dev` (Umgebung `K3C_DEV`); ohne ihn lehnt `Room.Dev` mit `ErrForbidden` ab.
- Routen: `engine/net/handler.go`; Auslieferung des Builds: `engine/net/static.go`.
- Welle: `startWave` in `engine/sim/waves.go`; Tageszeit: `cycleAt`/`stepCycle` in `engine/sim/cycle.go`, Zeit je Stufe `World.Time`.

## Erlaubte Dateien

- `engine/sim/dev.go`, `engine/sim/dev_test.go`
- `engine/room/dev.go`, `engine/room/dev_test.go`, `engine/room/status.go` (Zeitraffer und Pause in der Diagnose), `engine/room/run_test.go`
- `engine/net/dev.go` (neu), `engine/net/devpage_test.go` (neu), `engine/net/handler.go`, `engine/net/static.go`
- `docs/protocol.md` (Abschnitt HTTP), Planungsdateien

## Nicht-Ziele

WebSocket-Protokoll ändern; Passwortschutz; Seite `dm.html` (DBG3.2).

## Schritte

1. Sim: `DevStartWave(isl, stage)` (Welle sofort, wie bei Nachtbeginn) und `DevSetPhase(isl, phase)` (alle Stufen springen
   vorwärts zum Beginn der nächsten Phase `day`, `dusk` oder `night`; die Übergangs-Ereignisse laufen im nächsten Schritt wie sonst).
2. Raum: Dev-Aktionen ohne Gerät für die Seite (`Room.DevRoom`), Slot über den Monarch-Index; neue Aktionen `wave`, `phase`.
3. Net: `GET /api/dev` (`dev` + Raumliste), `GET /api/dev?room=CODE` (Diagnose), `POST /api/dev?room=CODE` (Aktion als JSON).
   Aktionen ohne Dev-Mode → 403. `/dm` → `dm.html`.
4. Tests je Aktion und für den Fall ohne Dev-Mode.

## Fertig, wenn

- [x] AC-02: `GET /api/dev?room=CODE` liefert die Diagnose (Test).
- [x] AC-03: Gold, Material, Zeitraffer und Pause über `POST /api/dev` (Test).
- [x] AC-05: Welle und Tageszeit über `POST /api/dev` (Test).
- [x] `task check` und `task check:go` grün.

## Prüfen

```bash
task check:go
task check
```

## Ergebnis

- AC-02 geprüft (Server-Teil): `TestDevSeiteOhneDevMode` – `GET /api/dev` Liste, `GET /api/dev?room=` Diagnose mit `timescale`/`paused`, auch ohne Dev-Mode.
- AC-03 geprüft (Server-Teil): `TestDevSeiteAktionen` – gold, material, timescale, pause über `POST /api/dev`; ohne Dev-Mode 403.
- AC-05 geprüft (Server-Teil): `TestDevStartWave`, `TestDevSetPhase` (Sim), `TestDevSeiteAktionen` (wave, phase → Nacht mit Nachtwelle).
- `/dm` → `dm.html`: `TestDevSeiteUnterDm` (allgemein: Pfad ohne Endung → `.html`).
- `task check:go` und `task check` grün (2026-10-05, Agent). Abweichung: `engine/room/status.go` und `run_test.go` zusätzlich erlaubt (Diagnose zeigt Zeitraffer/Pause).
- Lesen von `/api/dev` braucht kein Token (Spec: ohne Dev-Mode nur Diagnose); Passwortschutz bleibt Offene Frage.
