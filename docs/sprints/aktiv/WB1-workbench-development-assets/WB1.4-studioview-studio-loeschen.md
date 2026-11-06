# WB1.4 · Lesende Studio-Teile nach internal/studioview, tools/k3c-studio löschen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** ui
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** wb1/4-studioview
- **Abhängig von:** WB1.2, WB1.5
- **Tickets:** B-699
- **Kriterien:** AC-02, AC-04

## Ziel

Die Workbench hat aus dem Studio nur noch Lesendes: die Seiten **Sound** und **Quellen** im Bereich Assets und **„Im Spiel ansehen“** mit Testspielstand (Engine als Bibliothek im Workbench-Backend, Paket `internal/studioview`). Der Ordner `tools/k3c-studio/` samt Chat, Edit-Seiten und Planer ist gelöscht; `task buildall` baut keine Studio-EXE mehr.

## Kontext

- Entscheidung 005 › 5: „Aus dem Studio bleibt nur Lesendes, u. a. „Im Spiel ansehen“ mit Testspielstand (Engine als Bibliothek im Workbench-Backend). Edit-Seiten und Planer-UI entfallen.“ Sound- und Quellen-Tools (`assets_research`, `assets_hits`, `assets_import` …, MCP) bleiben unverändert – die Oberfläche zeigt nur.
- Voraussetzung **WB1.5**: Taskfile, CI, Repo-Tests und Doku kennen das Studio nicht mehr (sonst bricht `task` am fehlenden Include `tools/k3c-studio/Taskfile.yml`). Voraussetzung WB1.2: `assets/readonly.test.tsx`, Platzhalter Sound und Quellen in `assets/AssetsArea.tsx`.
- Quellen im Studio (zum Lesen, dann löschen; nach dem Löschen per `git show HEAD~1:<pfad>`):
  - `tools/k3c-studio/backend/engine.go`: `NewEngine(root, log)` mit eigenem `room.Manager` (Dev-Mode, Welten aus `data/worlds`) und `store.Saves`; `App.Game()` = Spiel-API (`/api/…`, `/ws`).
  - `backend/app_testsaves.go` (`WorldTestSaveCreate(seed)` voll ausgebaut über `room.Manager.FullSave`, `WorldTestSaveOpen`), `backend/app_game.go` (`WorldOpenGame`: Adresse nur Loopback, liefert `<Adresse>/index.html#game.html?autostart=1&save=…`).
  - `tools/k3c-studio/server.go`: `newHandler` (Spiel-API, `gameGuard`, `hostGuard`, `siteHandler`: erst `public/`, dann `dist/`).
  - Sound/Quellen: `backend/app_assets.go` (`AssetsList`, `AssetsCatalog`, `AssetsHits`) über `tools/assetkit` (`Load`, `Status`, `Catalog`, `LicenseFlag`, `LoadResearch`, `HitsOf`); Frontend-Bausteine `frontend/src/sound/` (`soundRows.ts`, `playback.ts`) und `frontend/src/sources/sourcesLogic.ts` als Vorlage.
- Workbench-Backend: Bindings sind Methoden an `App` (`tools/k3c-workbench/app_*.go`), Web-Zugriff nur über `uiMethods` in `uiapi.go` (gleich mit `GoApp` in `frontend/src/api/wails.ts`, `TestUIMethodsWieWailsTS`). Das Workbench-Modul hat schon `replace k3c => ../..` und `k3c/tools/assetkit` (go.mod). `engine/` darf nichts aus `tools/` importieren (umgekehrt erlaubt).
- **Entscheidungen der Sprint-README:** D-03 Sound/Quellen-Daten und Abspielen; D-04 „Im Spiel ansehen“ auf eigenem Loopback-Listener `127.0.0.1:0` (kein fester Port, keine Firewall-Abfrage), gestartet beim ersten Aufruf, Spielstände außerhalb des Repos unter `os.UserCacheDir()/k3c/studioview/<Hash des Checkouts>/saves`; D-06 „Testspielstand löschen“, Welten-Editor, Planer, Chat entfallen.
- Der Spiel-Client (`game.html`) muss gebaut sein (`dist/`); fehlt `dist/index.html`, nennt die Antwort `task build` statt eines leeren Fensters.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/studioview/` (neu, mit `codemap.md`)
- `tools/k3c-workbench/app_studio.go`, `app_studio_test.go` (ersetzt `StudioOpen` durch die Lese-Bindings)
- `tools/k3c-workbench/uiapi.go` (nur `uiMethods`: `StudioOpen` raus, Studio-Lesemethoden rein)
- `tools/k3c-workbench/go.mod`, `go.sum` (nur falls neue Abhängigkeiten nötig)
- `tools/k3c-workbench/frontend/src/assets/sound/`, `assets/sources/`, `assets/game/` (neu), `assets/AssetsArea.tsx`, `assets/readonly.test.tsx`, `assets/codemap.md`
- `tools/k3c-workbench/frontend/src/api/` (`types.ts`, `wails.ts`, `http.ts`, `mock.ts`)
- `tools/k3c-studio/` (ganz löschen)
- `tools/assetkit/writepaths_test.go` (liest heute `../k3c-studio`; nur diesen Teil entfernen)

## Nicht-Ziele

Kein Welten-Editor, kein Gebäude-Planer, kein Chat, keine Kandidaten-Zuordnung, kein Download/Import, kein Löschen von Testspielständen; keine Änderung an `engine/` (nur Import); Taskfile, CI, Repo-Tests und Doku sind WB1.5.

## Schritte

1. **Rot (Go):** `internal/studioview/studioview_test.go`: `Sounds(root)` liefert je Sound-Slot aus `data/assets.json` ID, Titel, Status (`assetkit.Status`) und Dateipfad; `SoundData(root, pfad)` liefert eine Data-URL nur für Dateien unter `public/` (`os.Root`), `..`, absolute Pfade und Backslash werden abgelehnt; `Sources(root)` liefert Katalog-Einträge (`assetkit.Catalog`, Lizenz-Marke `LicenseFlag`) und Recherche-Treffer je Slot (`LoadResearch`); `Game.Open(seed)` legt den Testspielstand `voll-<seed>` an (Ordner aus `t.TempDir()` als Cache) und liefert `http://127.0.0.1:<port>/index.html#game.html?autostart=1&save=voll-<seed>`; ungültiger Seed → Fehler; der Listener hört nur auf Loopback und lehnt fremde `Host`-Header ab. Vorlagen für Testfälle: `tools/k3c-studio/backend/app_testsaves_test.go`, `app_game_test.go`, `server_test.go`.
2. **Rot (Go):** `app_studio_test.go`: Bindings `StudioSounds`, `StudioSoundData`, `StudioSources`, `StudioGameOpen` rufen `studioview`; `StudioOpen` gibt es nicht mehr; `TestUIMethodsWieWailsTS` grün mit den neuen Methoden.
3. Umsetzen: `internal/studioview` (`engine.go` aus dem Studio übernommen und gekürzt, `game.go` mit Listener aus `server.go` ohne Studio-UI, `sounds.go`, `sources.go`), `app_studio.go`, `uiMethods`.
4. **Rot (Vitest):** `assets/sound/Sound.test.tsx` (Liste mit Status, Abspielen über `studioSoundData`, kein Zuordnen), `assets/sources/Sources.test.tsx` (Katalog mit Lizenz-Marke und Treffern je Slot, kein Recherche-/Download-Knopf), `assets/game/GameOpen.test.tsx` (Knopf „Im Spiel ansehen“ im Katalog-Kopf ruft `studioGameOpen(seed)` und öffnet die Adresse; Fehlertext sichtbar). `readonly.test.tsx` um Sound, Quellen und den Knopf ergänzen.
5. Frontend umsetzen, Mock in `api/mock.ts` ergänzen.
6. `tools/k3c-studio/` löschen (`git rm -r`), `tools/assetkit/writepaths_test.go` ohne `../k3c-studio`.
7. Prüfen: `rg -n "k3c-studio" tools/k3c-workbench tools/assetkit --glob '!*.md'` leer (außer Kommentaren mit Verweis auf die Herkunft); `check_run dev:test`; `task buildall` (oder `task k3c-workbench:build` + `task k3c-workbench:web:build`, falls `buildall` am Rechner an laufenden EXEs scheitert – dann im Ergebnis nennen).
8. Einmal `task check:diff`; Commit `feat(dev): lesende Studio-Teile in der Workbench, Studio-Programm entfernt (WB1.4)`.

## Fertig, wenn

- [ ] AC-02 (Sound, Quellen): `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts sound sources readonly` grün.
- [ ] AC-04: `tools/k3c-studio/` existiert nicht mehr (`git ls-files tools/k3c-studio` leer); `task buildall` grün und `ls bin/k3c-studio*` findet nichts Neues; `cd tools/k3c-workbench && go test ./internal/studioview/ ./` grün (bzw. `check_run dev:test`).
- [ ] `cd tools/assetkit && go test ./...` grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Während der Arbeit nur gezielt: `go test ./internal/studioview/` in `tools/k3c-workbench`, `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts <muster>`; mit k3c-workbench `check_run dev:test` bzw. `task:check:diff` mit `checkout`. Code-Review vor dem Commit (Shell-, Pfad- und Netz-Code, > 150 Zeilen): Reviewer-Agent parallel zu `task check:diff`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
