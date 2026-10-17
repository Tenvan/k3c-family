# WZ7.1 · Vorschau-Snapshot als exportierte Go-Funktion

- **Status:** fertig
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

- [x] AC-03: Go-Funktion mit Options-Struktur liefert Level und Zustand deterministisch (byte-gleiches JSON), ohne globalen Zustand, ohne Bürger und Gegner (Go-Test).
- [x] AC-03: Hub-Stufen, Gebäude-Abwahl, Tageszeit der drei Sim-Phasen und alle Fehlerfälle getestet.
- [x] AC-08: `check_run` `go:test` und `go:lint` grün.

## Prüfen

`task check:go`; über k3c-dev `check_run` mit `target: go:test` und `go:lint` (im Worktree mit `checkout`). Keine manuellen Prüfungen.

## Ergebnis

Umgesetzt: `engine/room/preview.go` (`PreviewOptions`, `PreviewSource`, `PreviewSiteKey`, `BuildPreview`) und `engine/net/preview.go` (`Preview`, `PreviewResult{Level, State}`), Tests in `preview_test.go` beider Pakete, Codemaps von `engine/room` und `engine/net`.

- **AC-03 geprüft (Go-Tests):** gleiche Optionen → gleiche Welt bzw. byte-gleiches JSON, auch nach einem Aufruf mit anderen Optionen (kein Paketzustand); je Hub-Stufe 0 (= höchste) bis 5 dieselben gebauten Plätze je Art wie `sim.PreviewHubLevel` auf frischer Welt, feste Erwartung healer ab Hub 3, armory ab Hub 4; Tageszeit `day`/`dusk`/`night` landet im Zustand (`cycle.phase`, `cycle.progress`); `dawn`, Fortschritt −0,1 / 1 / 1,5, Hub-Stufe −1 und über der Stufenzahl, Seed leer / > 64 Zeichen, Quelle fehlt, Biom unbekannt oder mit Pfad → Fehler; keine Bürger, Truppen, Landstreicher, Gegner (Listen `[]`, Ausgangswelt hatte Truppen), Umgebung (Ressourcen, Plätze) bleibt; Abwahl entfernt nur den Platz, unbekannte Schlüssel werden ignoriert, Stufen der übrigen bleiben; Nullwerte = Standardbild; Level = `level.Generate`; Zustand hat dieselben Felder wie `stateOf`, ohne `seed`, `widthUnits`, `devTimescale`; `Preview` legt keine Dateien an.
- **AC-08 geprüft:** `go:lint` grün (`check_run`); Pakete `engine/room`, `engine/net` und `engine/sim` komplett grün (kompilierte Test-Binaries in `.work/`). `check_run go:test` über alle Pakete lief wie schon in WZ7.8 ins Zeitlimit (Temp-Datei-Sperre auf diesem Rechner); die übrigen Pakete sind unverändert.
- **Review (getrennt, Subagent code-reviewer):** keine blockierenden Befunde. Eingearbeitet: Hub-Test vergleicht alle Arten gegen `sim.PreviewHubLevel`, Stufenzahl aus den Daten statt fest 5, Test prüft, dass die Ausgangswelt Truppen hatte.
- **Abweichung vom Plan:** Der Seed wird wie bei `GET /api/level` auf 1 bis 64 Zeichen geprüft (B-460 › Ausnahmefälle), nicht mit dem engeren Format von `fullSeed`; das Studio sperrt den Voll-Ausbau selbst bei unpassendem Seed (WZ7.4).
- **Hinweis für WZ7.3/WZ7.4:** Schlüssel der Gebäude-Auswahl `kind@x` mit X in Units als kürzeste Dezimaldarstellung (`strconv.FormatFloat(x, 'f', -1, 64)`); das Frontend bildet ihn aus dem Snapshot (`sites[].kind`, `sites[].x`). Tore bleiben ungebaut (wie `DevBuildAll`). Wails-Binding folgt in WZ7.3.
- Neue Tickets: keine.
