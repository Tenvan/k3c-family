# WZ7.1 · Vorschau-Snapshot als exportierte Go-Funktion

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** wz7/1-vorschau-snapshot
- **Abhängig von:** WZ7.8
- **Tickets:** B-460
- **Kriterien:** AC-03, AC-08

## Ziel

Eine exportierte Go-Funktion erzeugt aus einer Options-Struktur Level und Zustand im Format von `level` und `snap` (AC-03), ohne globalen Zustand, ohne Bürger und Gegner, mit Hub-Stufe und Tageszeit über `sim.PreviewHubLevel` und `sim.PreviewCycle` aus WZ7.8.

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- `engine/room/fullsave.go` (`fullIsland`) baut eine Insel; die Stufenrotation dort ignoriert die Sim-Regeln (B-425) und wird nicht übernommen.
- `engine/net/protocol.go`: `stateOf(w, timescale, paused)` und `levelMsg` (Layout = `w.Level`). `engine/room` darf `engine/net` nicht importieren: Welt-Aufbau in `room`, Serialisierung in `net`.
- `sim.CreateWorld` legt Startbesatzung (`hub.StartTroops`) und Landstreicher an den `recruitCamp`s an; beides muss aus dem Snapshot verschwinden (`w.Troops`, Camps ohne Landstreicher, `Enemies`, `SpawnQueue` leer, Listen `[]` statt `null`).
- Options-Struktur: Seed, Quelle (Wert: in WZ7 das Biom, später Insel + Stufe), Hub-Stufe, Gebäude-Auswahl, Tageszeit (Phase + Fortschritt); neue Felder (WZ8.4 „geplante Chunks“, WZ9 „Kandidaten-`hub.json`“) dürfen bestehende Aufrufer nicht ändern, Nullwert = heutiges Verhalten.
- Plan mit Details: `PreviewOptions`, `BuildPreview(PreviewOptions) (*sim.World, error)` in `engine/room/preview.go`, `Preview(opts) (PreviewResult{Level, State}, error)` in `engine/net/preview.go`.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `engine/room/preview.go`, `engine/room/preview_test.go` (neu)
- `engine/net/preview.go`, `engine/net/preview_test.go` (neu)
- `engine/room/codemap.md`, `engine/net/codemap.md` (nachziehen)
- diese Session-Datei

## Nicht-Ziele

Keine Änderung in `engine/sim/` (WZ7.8), `engine/level/`, `src/`, `tools/`, `data/`. Kein HTTP-Endpunkt, kein Wails-Binding (WZ7.3), kein Spielstand-Schreiben (`saves/` bleibt unberührt), kein `sim.UseData`, keine Paket-Variablen. Kein Protokoll- oder Spielstand-Wechsel.

## Schritte

1. Test zuerst für die Optionen-Prüfung: Seed-Format (wie `fullSeed`), unbekanntes Biom, fehlende Quelle, Hub-Stufe außerhalb von 1 bis Anzahl `levels` (0 = höchste), Phase nicht `day`/`dusk`/`night` (auch `dawn`), Fortschritt außerhalb [0, 1) (auch genau 1) → Fehler.
2. `PreviewOptions` und `BuildPreview` umsetzen: `sim.CreateWorld(biome, seed, …)`, Bürger und Gegner entfernen, `sim.PreviewHubLevel`, `sim.PreviewCycle`.
3. Gebäude-Auswahl: abgewählte Plätze (`kind@x`) nach dem Aufbau entfernen; die Stufen der übrigen ändern sich dadurch nicht; unbekannte Schlüssel werden ignoriert.
4. `net.Preview`: Level aus `w.Level`, Zustand über `stateOf(w, 0, false)`; Feldvergleich mit einem `snap` einer Raum-Welt gleicher Eingaben (Format).
5. Tests: gleiche Eingaben → byte-gleiches JSON, auch bei zwischenzeitlichem Aufruf mit anderen Optionen; je Hub-Stufe aus `levels`; Abwahl; keine Bürger/Truppen/Gegner/Landstreicher; drei Sim-Phasen; Fehlerfälle; `saves/` unberührt.
6. Codemaps nachziehen, Prüfungen laufen lassen.

## Fertig, wenn

- [ ] AC-03: Go-Funktion mit Options-Struktur liefert Level und Zustand deterministisch (byte-gleiches JSON), ohne globalen Zustand, ohne Bürger und Gegner (Go-Test).
- [ ] AC-03: Hub-Stufen, Gebäude-Abwahl, Tageszeit der drei Sim-Phasen und alle Fehlerfälle getestet.
- [ ] AC-08: `check_run` `go:test` und `go:lint` grün.

## Prüfen

`task check:go`; über k3c-dev `check_run` mit `target: go:test` und `go:lint` (im Worktree mit `checkout`). Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
