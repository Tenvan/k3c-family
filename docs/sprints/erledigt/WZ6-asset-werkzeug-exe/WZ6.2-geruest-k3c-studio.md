# WZ6.2 · Gerüst k3c-studio: Wails-App, EXE, Task, gemeinsames Asset-Paket

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz6/2-geruest
- **Abhängig von:** WZ6.1
- **Tickets:** B-459
- **Kriterien:** AC-01, AC-04, AC-07

## Ziel

`task k3c-studio:build` legt `bin/k3c-studio.exe` an, `task k3c-studio` startet ein Asset-Studio-Fenster mit leerer Navigation (Grafik, Sound). Die Asset-Logik liegt im gemeinsamen Go-Modul `tools/assetkit`, das k3c-dev und das Asset-Studio nutzen; das Backend des Asset-Studios meldet Änderungen an `data/assets.json` ans Frontend und lehnt Speichern auf veraltetem Stand ab (Backend-Teil von AC-07).

## Kontext

Entscheidungen 🧑 2026-10-17 (B-459 › Offene Fragen, Sprint-README › Anforderungen): Wails-App `tools/k3c-studio`, Name Asset-Studio, EXE `bin/k3c-studio.exe`; gemeinsames Go-Modul `tools/assetkit` mit `replace`, **kein `go.work`**; keine neue Abhängigkeit (auch kein `fsnotify`, deshalb Polling); die MCP-Tools `assets_*` bleiben in k3c-dev. Grenzfälle in DEV wie GEB1.2 (`tools/gebaeude`): `Taskfile.yml`, `.github/workflows/ci.yml`, `.oxlintrc.json`, `.gitignore`, `.dockerignore`, `requirements.md`, `CLAUDE.md` › Struktur, `codemap.md`.

### Vorbild k3c-dev

- Eigenes Modul `k3c/tools/k3c-dev` (`tools/k3c-dev/go.mod`, Wails v2.16.0). Am Ende steht `replace k3c => ../..` (Zeile ~58); genauso bekommt es `require k3c/tools/assetkit v0.0.0` und `replace k3c/tools/assetkit => ../assetkit`.
- `main.go` (Wurzel über `mcpsrv.FindRoot(os.Getwd())`, `go:embed` von `frontend/dist`, `assetserver.Options{Assets: …}`), `app.go`, `window.go`, `wails.json`, `.golangci.yml`, Frontend Vite/React/Radix unter `frontend/` mit eigenem `package-lock.json` und generiertem `wailsjs/`.
- Tasks in `Taskfile.yml`: Variable `DEV` (Zeile 6), `install` (Zeilen 29–34: `npm ci`, `npm ci --prefix {{.DEV}}/frontend`), `dev:frontend` (Zeile 190, **nicht** `k3c-dev:frontend`: `tsc --noEmit` + `vite build`), `check:dev` (Zeile 204, deps `dev:frontend`, `go test ./...`, `golangci-lint run`), `k3c-dev` (`wails dev`), `k3c-dev:build` (`wails build`), `check:all` (Zeile 267: `check`, `build`, `check:go`, `check:dev`).
- CI: Job `k3c-dev` in `.github/workflows/ci.yml` (Zeilen ~143–178, Windows: `npm ci`, `task dev:frontend`, `go test`, golangci-lint v2.14, `wails build -s`). Die Komponententests der Frontends laufen über das Vitest des Hauptprojekts in `task test` mit (`vite.config.ts` schließt nur `.claude/**` und `tools/cockpit/**` aus); ihre Abhängigkeiten holt `task install`.
- `.oxlintrc.json` ignoriert `tools/k3c-dev/frontend/dist` und `tools/k3c-dev/frontend/wailsjs`; `.dockerignore` nennt `tools/k3c-dev`; `.gitignore` hat `/bin/`.

### Asset-Logik heute

`tools/k3c-dev/internal/assets/` (`catalog.go`, `credits.go`, `import.go`, `register.go`, `research.go`, `sources.go`, Tests, `testdata/`, `codemap.md`) importiert nur die Standardbibliothek. Nutzer: `tools/k3c-dev/app_assets.go` und `app_assets_test.go` (Alias `assetreg`), `tools/k3c-dev/internal/mcpsrv/tools_assets.go`, `tools_assets_research.go`, `tools_assets_research_test.go`. Bindings in `app_assets.go`: `AssetsList`, `AssetsSet`, `AssetsCatalog`, `AssetsHits`, `AssetsResearch`, `AssetsHitDrop`, `AssetsImport`, `AssetsDownload`, `AssetsLicenseOK`.

### Fallstricke

- **`internal/` über Modulgrenzen:** Aus `tools/k3c-studio` ist `k3c/tools/k3c-dev/internal/...` nicht importierbar. Deshalb zieht das Paket nach `tools/assetkit` (Modul `k3c/tools/assetkit`, `git mv`, damit die Historie bleibt).
- **Relativer Pfad im Test:** `internal/assets/repo_test.go:13` sucht die Repo-Wurzel mit `filepath.Join("..", "..", "..", "..")`; nach dem Umzug stimmt das nicht mehr. Wurzel im Test aufwärts suchen (erstes Verzeichnis mit `data/assets.json`), keinen neuen festen Relativpfad.
- **Bilder ohne Vite:** `AssetsList` setzt `Preview` über `previewBase()` auf die Adresse des laufenden Vite-Dienstes (`app_assets.go:60–70`); `a.wait()` und `a.svcCtx` hängen am Dienste-Start von k3c-dev. Das Asset-Studio hat keine Dienste: Es liefert die Dateien aus `public/` selbst über den Wails-Asset-Handler (`assetserver.Options{Handler: …}` auf `<Wurzel>/public`, nur lesend, Pfade unterhalb von `public/` erzwingen), `Preview` ist dort `/`. Recherche und Import laufen im Kontext aus `OnStartup` statt `svcCtx`.
- **Repo-Wurzel und Worktree:** `mcpsrv.FindRoot` ist `internal`. Das Asset-Studio sucht die Wurzel selbst: ab Arbeitsverzeichnis aufwärts das `go.mod` mit `module k3c`, ersatzweise ab dem Ordner der EXE (Start per Doppelklick aus `bin/`). Gestartet aus einem Worktree bedient es diesen Worktree; die EXE baut und startet nur die Repo-Wurzel (fester Pfad, Firewall). Die Funktion kommt als `FindRoot` mit Test nach `tools/assetkit`; `mcpsrv` behält seine eigene (nur Importpfade ändern).
- **Datei-Beobachtung:** `planning.Watch` (`tools/k3c-dev/internal/planning/watch.go:62`) ist `internal` und pollt. Das Asset-Studio bekommt eine eigene Poll-Schleife für `data/assets.json` (z. B. `assetkit.Watch(ctx, root, every, onChange)`, Vergleich über Größe, Änderungszeit und Hash), die per `runtime.EventsEmit` ein Ereignis `assets:changed` ans Frontend schickt.
- **Schreibregel** (B-459 › Ausnahme- und Fehlerfälle): Speichern nur, wenn `data/assets.json` seit dem Laden unverändert ist. `AssetsList` liefert einen Stand (Hash des Datei-Inhalts), `AssetsSet` nimmt ihn mit und lehnt bei Abweichung mit „Register geändert, bitte erneut speichern“ ab, ohne zu schreiben. Die Prüfung liegt in `assetkit` (z. B. `SetChoicesIf(root, id, wahl, stand)`), damit sie testbar ist; `assets_set` der MCP-Tools bleibt unverändert. Das Neuladen ohne Verlust von Auswahl, Filter und Eingaben ist Frontend-Arbeit von WZ6.3, nicht dieser Session.
- **`wails build`** legt die EXE nach `tools/k3c-studio/build/bin/k3c-studio.exe`, nicht nach `bin/`. `k3c-studio:build` kopiert sie danach ausdrücklich nach `bin/k3c-studio.exe`.
- **Firewall:** EXE mit festem Pfad, kein `go run`. Das Asset-Studio öffnet keinen eigenen Port (kein MCP, kein HTTP-Server); Proben, falls nötig, nur auf `127.0.0.1`.
- **`go:embed`:** Wie bei k3c-dev braucht `go test` und `golangci-lint` in `tools/k3c-studio` ein gebautes `frontend/dist`; das Prüfziel hängt deshalb am Frontend-Task.

## Erlaubte Dateien

- `tools/k3c-studio/**` (neu: `go.mod`, `go.sum`, `wails.json`, `main.go`, `app*.go`, Tests, `.golangci.yml`, `frontend/` mit `package.json`, `package-lock.json`, `src/`, `codemap.md`)
- `tools/assetkit/**` (neu: `go.mod`, `go.sum`, Paket aus `tools/k3c-dev/internal/assets/` samt `testdata/` und `codemap.md`, `.golangci.yml`)
- `tools/k3c-dev/internal/assets/**` (nur verschieben/entfernen)
- `tools/k3c-dev/app_assets.go`, `tools/k3c-dev/app_assets_test.go` (Importpfad, sonst unverändert)
- `tools/k3c-dev/internal/mcpsrv/tools_assets.go`, `tools_assets_research.go`, `tools_assets_research_test.go` (nur Importpfade)
- `tools/k3c-dev/go.mod`, `tools/k3c-dev/go.sum` (`require` und `replace` für `k3c/tools/assetkit`)
- `tools/k3c-dev/codemap.md` (Verweis auf `tools/assetkit`)
- Grenzfall DEV (wie GEB1.2):
  - `Taskfile.yml` (Variable, Tasks `k3c-studio`, `k3c-studio:build`, `studio:frontend`, `check:studio`; `install` und `check:all` erweitern)
  - `.github/workflows/ci.yml` (Studio-Prüfung: Frontend, `go test` für `tools/assetkit` und `tools/k3c-studio`, golangci-lint, `wails build -s`; Lockfile im Cache-Pfad)
  - `.oxlintrc.json` (`ignorePatterns` um `tools/k3c-studio/frontend/dist` und `tools/k3c-studio/frontend/wailsjs`)
  - `.gitignore` (Build-Ausgaben `tools/k3c-studio/build/bin/`, `frontend/dist/`, soweit nicht schon abgedeckt)
  - `.dockerignore` (`tools/k3c-studio`, `tools/assetkit`)
  - `requirements.md` (Wails-Zeile und `npm ci --prefix tools/k3c-studio/frontend`)
  - `CLAUDE.md` (nur Abschnitt Struktur: `tools/k3c-studio`, `tools/assetkit`)
  - `codemap.md` (Wurzel: neue Ordner)

## Nicht-Ziele

- Inhalte der Seiten Grafik und Sound (WZ6.3, WZ6.4), auch nicht das Neuladen ohne Verlust von Auswahl, Filter und Eingaben im Frontend (WZ6.3).
- Assets-Seite der Workbench entfernen oder Verweis auf das Asset-Studio (WZ6.5); `assetvorschau.html` (WZ6.6).
- Eigener MCP-Server, eigener HTTP-Port oder Dienste-Steuerung im Asset-Studio.
- Änderungen am Format von `data/assets.json` oder am Verhalten der MCP-Tools `assets_*`.
- Neue Go- oder npm-Abhängigkeiten (Frontend nur mit Paketen und Versionen aus `tools/k3c-dev/frontend/package.json`), `go.work`, `fsnotify`.
- Spiel-Code unter `src/` und der Vite-/TS-Alias auf `src/` (kommt mit WZ6.3).

## Schritte

1. **Modul `tools/assetkit`:** `git mv tools/k3c-dev/internal/assets tools/assetkit`, `go.mod` mit `module k3c/tools/assetkit` (Go-Version wie k3c-dev), Paketname bleibt `assets`. `repo_test.go` sucht die Wurzel aufwärts statt `../../../..`. In k3c-dev `require` + `replace k3c/tools/assetkit => ../assetkit` eintragen, Importe in `app_assets.go`, `app_assets_test.go` und `mcpsrv/tools_assets*.go` umstellen. `go test ./...` in `tools/assetkit` und `tools/k3c-dev` grün.
2. **Gemeinsame Helfer in `assetkit`, je mit Test:** `FindRoot` (aufwärts `go.mod` mit `module k3c`), `Stamp` (Hash von `data/assets.json`), `SetChoicesIf` (schreibt nur bei gleichem Stand, sonst Fehler „Register geändert, bitte erneut speichern“ ohne Schreibzugriff), `Watch` (Poll-Schleife mit Intervall, ruft `onChange` nur bei echter Änderung, endet mit dem Kontext).
3. **`tools/k3c-studio` nach Vorbild k3c-dev:** Modul `k3c/tools/k3c-studio` (Wails v2.16.0 wie k3c-dev, `replace` auf `../assetkit`), `wails.json` (Ausgabename `k3c-studio`), Fenster „Asset-Studio“, Frontend mit leerer Navigation Grafik/Sound und einem Komponententest (läuft im Vitest des Hauptprojekts).
4. **Backend:** Wurzel per `assetkit.FindRoot` (Arbeitsverzeichnis, ersatzweise Ordner der EXE); Asset-Handler liefert `<Wurzel>/public` nur lesend; Bindings wie `app_assets.go` von k3c-dev ohne Vite und Dienste (`AssetsList` mit `Stand` und `Preview: "/"`, `AssetsSet` mit Stand über `SetChoicesIf`, `AssetsCatalog`, `AssetsHits`, `AssetsResearch`, `AssetsHitDrop`, `AssetsImport`, `AssetsDownload`, `AssetsLicenseOK`); `assetkit.Watch` auf `data/assets.json` sendet `assets:changed`. Tests: Ablehnung bei geändertem Stand, Ereignis bei Änderung durch einen anderen Schreiber, Handler verweigert Pfade außerhalb von `public/`.
5. **Tasks in `Taskfile.yml`:** Variable `STUDIO: tools/k3c-studio`; `studio:frontend` (wie `dev:frontend`); `check:studio` (deps `studio:frontend`; `go test ./...` und `golangci-lint run` in `tools/assetkit` und `{{.STUDIO}}`); `k3c-studio` (`wails dev`, Vorbedingung Wails-CLI); `k3c-studio:build` (`wails build`, danach Kopie `{{.STUDIO}}/build/bin/k3c-studio{{exeExt}}` → `bin/k3c-studio{{exeExt}}`); `install` um `npm ci --prefix {{.STUDIO}}/frontend` erweitern; `check:all` nimmt `check:studio` mit.
6. **Grenzfälle nachziehen:** CI-Job für das Asset-Studio (Windows wie `k3c-dev`, Lockfile im Cache-Pfad auch im Client-Job), `.oxlintrc.json`, `.gitignore`, `.dockerignore`, `requirements.md`, `CLAUDE.md` › Struktur, `codemap.md` (Wurzel, `tools/k3c-dev`, neue Ordner).
7. **Fenster starten** (nur mit Freigabe 🧑 für diesen Lauf): `task k3c-studio:build`, `bin/k3c-studio.exe` aus der Repo-Wurzel starten, Fenster „Asset-Studio“ mit Navigation Grafik/Sound sichtbar, keine Firewall-Abfrage; danach Fenster schließen. Ohne Freigabe im Ergebnis als offen für WZ6.8 vermerken.
8. Prüfen wie unten, Ergebnis mit Nachweis je Kriterium; Wahl `tools/assetkit` mit `replace` statt `go.work` im Ergebnis kurz begründen.

## Fertig, wenn

- [ ] AC-01: `task k3c-studio:build` erzeugt `bin/k3c-studio.exe` (geprüft); Fensterstart von `task k3c-studio` und der EXE ohne Freigabe 🧑 nicht geprüft → offen für WZ6.8.
- [x] AC-04: `task check:studio` (Go-Tests und golangci-lint für `tools/assetkit` und `tools/k3c-studio`, Frontend-Build) läuft in `task check:all`; der Komponententest des Studio-Frontends läuft in `task test`; die CI prüft das Asset-Studio (Job `k3c-studio`: Frontend, Go-Tests, Lint, `wails build -s`).
- [x] AC-07 (Backend): Tests in `tools/assetkit` bzw. `tools/k3c-studio` belegen: Speichern auf veraltetem Stand wird abgelehnt und schreibt nichts; eine Änderung von `data/assets.json` durch einen anderen Prozess löst `assets:changed` aus. Die MCP-Tools `assets_*` funktionieren unverändert (`go test ./...` in `tools/k3c-dev` grün).
- [x] `tools/k3c-dev/internal/assets/` existiert nicht mehr; kein Import von `k3c/tools/k3c-dev/internal/assets` bleibt (`git grep` leer); kein `go.work`.
- [ ] `task check`, `task check:dev` und `task check:studio` grün: `check:studio` grün; `task check` und `check:dev` nur durch vorbestehende Lint-Befunde auf `develop` rot (B-465), Tests und Typecheck grün.

## Prüfen

```bash
task check
task check:dev
task check:studio
task k3c-studio:build
```

Läuft k3c-dev, `task check` und `task check:dev` über `check_run`. `task k3c-studio:build` baut nur das Asset-Studio und startet die Workbench nicht neu.

Umgebung `live`: Schritt 7 (Fenster starten) braucht die laufende Oberfläche im Checkout, nicht im Worktree. **Browser-/Fensterprüfung nur mit Freigabe 🧑** für diesen Lauf; ohne Freigabe bleibt sie im Ergebnis als offen vermerkt und geht in die Abnahme WZ6.8.

## Ergebnis

- **AC-01 teilweise geprüft:** `task k3c-studio:build` baut `tools/k3c-studio/build/bin/k3c-studio.exe` und kopiert nach `bin/k3c-studio.exe` (im Worktree, nicht gestartet). Das Asset-Studio öffnet keinen Port (kein MCP, kein HTTP-Server; Dateien aus `public/` über den Wails-Asset-Handler). Fensterprüfung (Schritt 7) ohne Freigabe 🧑 nicht gelaufen → offen für WZ6.8.
- **AC-04 umgesetzt, geprüft:** `task check:studio` grün (Go-Tests und golangci-lint `tools/assetkit` und `tools/k3c-studio`, Frontend-Build), in `task check:all` eingetragen; `tools/k3c-studio/frontend/src/App.test.tsx` läuft in `check_run task:test` (grün); CI-Job `k3c-studio` (Windows) und Lockfile im Cache des Jobs `check`.
- **AC-07 (Backend) umgesetzt, geprüft:** `assetkit.SetChoicesIf`/`ErrStale` („Register geändert, bitte erneut speichern“), `Stamp`, `Watch` mit Tests (`stamp_test.go`); Studio-Tests `TestAssetsSetSchreibregel`, `TestAssetsChangedEreignis`, `TestPublicHandler`. `go test ./...` in `tools/k3c-dev` grün (MCP `assets_*` unverändert, nur Importpfad).
- **Prüfung:** `check_run task:test` und `task:typecheck` grün; `task:lint` und `golangci-lint` in `tools/k3c-dev` rot nur durch vorbestehende Befunde auf `develop` (`SprintsBacklog.tsx`, `internal/planning/mergebase.go`, `wtdiff.go`, alle aus #329) → B-465 ergänzt.
- **Review (getrennter Reviewer):** keine schweren Befunde. Behoben: Single-Instance-Lock je Checkout (Hash der Wurzel), `secondInstance` wie k3c-dev, `Watch` übernimmt den Stand nach Lesefehler nicht, Hinweis in `k3c-studio:build` (laufendes Studio schließen), Testkommentar. Vite `appType: 'mpa'`, damit `wails dev` Bilder aus `public/` über den Handler liefert (ungeprüft bis zum ersten Fensterlauf, WZ6.3 prüft). Bewusst gelassen (ponytail-Kommentar): Fenster zwischen Stamp und Save bei einem fremden Prozess. Kommentar `internal/assets` in `tools/k3c-dev/frontend/src/api/assetTypes.ts` nimmt WZ6.5 mit.
- **Wahl `replace` statt `go.work`:** wie das bestehende `replace k3c => ../..` in k3c-dev; jedes Werkzeug bleibt allein baubar (CI, `wails build`), kein Arbeitsbereich-Zustand im Repo, den Go-Befehle in anderen Modulen still mitlesen.
- **Abweichungen:** `tools/codemap.md` und `tools/k3c-dev/internal/codemap.md` mitgezogen (Verweis auf das verschobene Paket, sonst tote Links); `.gitignore` der Wurzel unverändert, stattdessen `tools/k3c-studio/.gitignore` wie bei k3c-dev. Vite-Port des Studios 5192.
- **Geprüft von:** Claude (autonom). **Neue Tickets:** B-467 (Wunsch 🧑 während der Session: Worktree-Pills auf der Planungsseite).
