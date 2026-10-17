# WZ6.5 · Workbench ohne Assets-Seite, Verweis auf das Asset-Studio

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz6/5-workbench-ohne-assets
- **Abhängig von:** WZ6.3, WZ6.4
- **Tickets:** B-459
- **Kriterien:** AC-03

## Ziel

Die Workbench hat keine Assets-Seite mehr; an ihrer Stelle startet der Verweis „Asset-Studio öffnen“ die EXE `bin/k3c-studio.exe`. Ein Neubau des Asset-Studios bei laufender Workbench lässt Workbench und MCP verbunden.

## Kontext

Nach WZ6.3/WZ6.4 liegen Grafik- und Sound-Seite im Asset-Studio (`tools/k3c-studio`), `task k3c-studio:build` baut `bin/k3c-studio.exe` (WZ6.2).

- **Navigation:** `tools/k3c-dev/frontend/src/Header.tsx` (`PAGES` mit `assets`, Beschriftung „Assets“), `App.tsx` (Import `AssetsPage`, `Tabs.Content value="assets"`).
- **Seite und API:** `frontend/src/assets/` (samt `codemap.md`), `api/assetTypes.ts`, `api/mockAssets.ts` und die Asset-Teile von `api/index.ts`, `api/mock.ts`, `api/wails.ts`, `api/types.ts`. `frontend/wailsjs/` wird generiert und ist nicht eingecheckt.
- **Go-Bindings:** `tools/k3c-dev/app_assets.go` (`AssetsList`, `AssetsSet`, `AssetsCatalog`, `AssetsHits`, `AssetsResearch`, `AssetsHitDrop`, `AssetsImport`, `AssetsDownload`, `AssetsLicenseOK`, `previewBase`) nutzt nur die Oberfläche. Die MCP-Tools `assets_*` rufen das gemeinsame Asset-Paket (WZ6.2) direkt und bleiben unverändert. Ungenutzte Bindings fallen samt Test weg, notfalls die ganze Datei.
- **Neues Binding:** Der Start des Asset-Studios ist neu in k3c-dev, in der neuen Datei `tools/k3c-dev/app_studio.go` mit Test `app_studio_test.go`. Die Methode an `App` startet `<Repo-Wurzel der Workbench>/bin/k3c-studio.exe` mit festem Pfad: kein `go run`, kein `wails dev`, kein Shell-Aufruf, keine Pfadteile oder Argumente aus dem Frontend. Der Prozess läuft losgelöst; ein Ende der Workbench beendet das Studio nicht. Vorbilder für Prozessstarts: `internal/proc/proc.go`, `app_claude.go`.
- **Fehlerfall (B-459 › Ausnahme- und Fehlerfälle):** Fehlt `bin/k3c-studio.exe`, liefert das Binding den Fehler „Asset-Studio nicht gebaut (`task k3c-studio:build`)“, und die Oberfläche zeigt ihn als Hinweis.
- **Firewall:** EXE mit festem Pfad (keine Firewall-Abfrage), Proben nur auf `127.0.0.1`.
- **Fallstrick Neubau:** Windows sperrt eine laufende EXE. Für die Neubau-Probe ist das Asset-Studio deshalb geschlossen, die Workbench läuft.
- **Umgebung live:** Die Neubau-Probe braucht die laufende Workbench. Fenster- und Browser-Prüfungen macht ein Agent nur mit Freigabe von 🧑 für diesen Lauf, den Freigabevermerk (Datum, Quelle) trägt er ins Ergebnis ein.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-dev/frontend/src/…` (Navigation `Header.tsx`, `App.tsx`, Verweis „Asset-Studio öffnen“, `assets/` entfernen, `api/` mit Typen, Mocks und Wails-Aufrufen, Tests, `codemap.md` der Unterordner)
- `tools/k3c-dev/app_studio.go`, `tools/k3c-dev/app_studio_test.go` (neu: Start des Asset-Studios)
- `tools/k3c-dev/app_assets.go`, `tools/k3c-dev/app_assets_test.go` (nur nicht mehr gebrauchte Bindings entfernen, notfalls die ganze Datei)
- `tools/k3c-dev/app.go` (nur, falls das neue Binding dort die Repo-Wurzel braucht)
- `tools/k3c-dev/codemap.md`, `tools/k3c-dev/frontend/codemap.md`

## Nicht-Ziele

MCP-Tools `assets_*` oder das gemeinsame Asset-Paket ändern; das Asset-Studio selbst ändern (`tools/k3c-studio/`, WZ6.2–WZ6.4); `Taskfile.yml` ändern; das Studio aus der Workbench beenden, überwachen oder als Dienst in `services.json` führen; Dev-Seiten entfernen (WZ6.6).

## Schritte

1. Go: Binding zum Start des Asset-Studios in `app_studio.go`, Name frei, z. B. `StudioOpen()`. Pfad aus der Repo-Wurzel, `os.Stat`, Fehlertext laut Kontext, losgelöster Start. Test in `app_studio_test.go`: Fehlt die EXE, kommt genau der Fehlertext; liegt eine Datei dort, wird genau dieser Pfad gestartet. Den Start im Test über eine austauschbare Funktion abfangen, keinen echten Prozess starten.
2. Frontend: Reiter „Assets“ aus `Header.tsx`/`App.tsx` entfernen, Verweis „Asset-Studio öffnen“ in die Kopfzeile, den Fehler als Hinweis zeigen; Mock-Aufruf in `api/mock.ts` ergänzen, Test für den Hinweis.
3. `frontend/src/assets/`, `api/assetTypes.ts`, `api/mockAssets.ts` und die Asset-Teile in `api/*` entfernen; ungenutzte Bindings in `app_assets.go`/`app_assets_test.go` entfernen. Die Tests der MCP-Tools `assets_*` bleiben grün.
4. Codemaps nachziehen; `check_run dev:test`, `task check:dev` und `task check` grün.
5. Probe nur mit Freigabe von 🧑 für diesen Lauf: Die Workbench läuft, `workbench_status` notiert Laufzeit und MCP. Das Asset-Studio ist geschlossen; dann `task k3c-studio:build`. Danach zeigt `workbench_status` dieselbe Instanz mit weiterlaufender Laufzeit, `plan_get WZ6` antwortet, und der Verweis startet das neu gebaute Studio. Die Beobachtung kommt mit Datum und Freigabevermerk ins Ergebnis. Ohne Freigabe kommt die Probe als offener Punkt in die Abnahme-Liste von WZ6.8, im Ergebnis steht „Probe offen (WZ6.8)“.

## Fertig, wenn

- [x] AC-03: Die Kopfzeile der Workbench hat keinen Reiter „Assets“, `tools/k3c-dev/frontend/src/assets/` gibt es nicht mehr; der Verweis „Asset-Studio öffnen“ startet `bin/k3c-studio.exe` (Go-Test `app_studio_test.go`).
- [x] AC-03: Fehlt die EXE, meldet die Workbench „Asset-Studio nicht gebaut (`task k3c-studio:build`)“ (Go-Test und Frontend-Test mit Mock).
- [ ] AC-03: Ein Neubau des Asset-Studios bei laufender Workbench führt zu keinem Neustart und keinem MCP-Abbruch: Probe offen (WZ6.8), für diesen Lauf nicht freigegeben.
- [ ] Die MCP-Tools `assets_*` sind unverändert; `check_run dev:test`, `task check:dev` und `task check` sind grün: MCP-Tools unverändert, Go-Tests und Frontend-Tests grün; `task:lint` und `golangci-lint` rot nur durch B-465 (Fix in PR #339, noch nicht in `develop`).

## Prüfen

```bash
task check
task check:dev
```

Manuelle Prüfung: nur die Neubau-Probe (Schritt 5) samt Klick auf den Verweis im Workbench-Fenster, und nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-03 (Assets-Seite weg, Verweis) umgesetzt, geprüft:** Reiter „Assets“ aus `Header.tsx`/`App.tsx` entfernt; `frontend/src/assets/`, `api/assetTypes.ts`, `api/mockAssets.ts`, `styles/assets.css` und die Asset-Teile von `api/types.ts`, `api/wails.ts`, `api/mock.ts` entfernt; `app_assets.go` und `app_assets_test.go` ganz entfernt (nur die Oberfläche nutzte sie). Neu `app_studio.go` (`StudioOpen`, fester Pfad `<Wurzel>/bin/k3c-studio.exe`, Start ohne Job Object, Arbeitsordner = Repo-Wurzel) mit `app_studio_test.go` (fehlende EXE → genauer Hinweis, Ordner statt EXE → Hinweis, sonst genau dieser Pfad, Startfehler wird durchgereicht; kein echter Prozess). Kopfzeile: `StudioLink` „Asset-Studio öffnen“, Hinweis daneben; `StudioLink.test.tsx` (kein Reiter Assets, Hinweis bei fehlender EXE, kein Hinweis bei Erfolg).
- **AC-03 (Neubau-Probe):** Probe offen (WZ6.8), für diesen Lauf nicht freigegeben.
- **Prüfung:** `task check:dev`: Frontend-Build und `go test` grün, `golangci-lint` nur mit den zwei B-465-Befunden; `check_run task:check`: Typecheck und Tests grün, Lint nur B-465 (Fix in PR #339). MCP-Tools `assets_*` und `tools/assetkit` unverändert.
- **Geprüft von:** Claude (autonom). **Neue Tickets:** keine.
