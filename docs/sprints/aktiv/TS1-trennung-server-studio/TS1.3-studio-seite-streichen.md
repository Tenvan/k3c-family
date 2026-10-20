# TS1.3 · Studio-Seite des Servers streichen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** ts1/3-studio-seite-streichen
- **Abhängig von:** TS1.2
- **Tickets:** B-507
- **Kriterien:** AC-03

## Ziel

Der Spielserver hat keine Studio-Seite mehr: `/studio/` und `/api/studio` antworten 404, Kachel, `studio.html`, Vite-Regel, `dist/studio`, `K3C_STUDIO_LAN` und der Dev-Mode-Zwang für das Studio sind entfernt (AC-03).

## Kontext

Spec `B-507 › Anforderungen › Trennung`. Fundstellen: `cmd/k3c-server` (Studio-Route, Dev-Mode), `src/landing/pages.ts` (`studio`-Kachel), `studio.html`, `vite.config.ts` (Regel für `/api/studio`, Test-Ausschluss), `Taskfile.yml` (`studio:frontend` baut nach `dist/studio/`, `build` ruft es auf; `k3c-studio`-Ziele bleiben, bauen aber in einen eigenen Ordner unter `tools/k3c-studio/`), `tests/projectRules.test.ts`, `.github/workflows/ci.yml`. Das Wails-Fenster `k3c-studio` serviert sein Frontend selbst. Der Dev-Mode des Servers (`K3C_DEV`) bleibt, soweit andere Dev-Funktionen ihn brauchen; nur der Studio-Anteil entfällt.

## Erlaubte Dateien

- `cmd/k3c-server/**`, `engine/net/**` nur für Studio-Routen
- `src/landing/pages.ts`, `studio.html`, `vite.config.ts`, `tests/projectRules.test.ts`, `tests/**` zu Landingpage und Studio
- `Taskfile.yml`, `.github/workflows/ci.yml`, `tools/k3c-studio/**` (Build-Ausgabe, Pfade)
- `src/tools/devTiles.ts`, falls dort verlinkt

## Nicht-Ziele

Watch (TS1.4); Pages (TS1.5); Dokumentation (TS1.5); Funktionsänderungen im Studio.

## Schritte

1. Tests zuerst (rot): Server antwortet 404 auf `/studio/` und `/api/studio/…`; Landingpage-Test kennt keine `studio`-Kachel; `task build` erzeugt kein `dist/studio`.
2. Studio-Routen, Kachel, `studio.html`, Vite-Regel, `K3C_STUDIO_LAN` entfernen; Frontend-Build des Studios nach `tools/k3c-studio/` umlenken.
3. `task build`/`serve`, CI-Job `k3c-studio`, `projectRules`-Test anpassen.
4. `check_run` task:check, task:check:go grün; Commit `refactor(srv): Studio-Seite aus dem Server entfernt (TS1.3)`.

## Fertig, wenn

- [ ] AC-03: Tests für 404 auf `/studio/` und `/api/studio`, keine Kachel, kein `dist/studio`; `grep K3C_STUDIO_LAN` ohne Treffer im Code.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check
task check:go
task check:studio
```

## Ergebnis

- **AC-03 geprüft:** `engine/net/nostudio_test.go` (`TestKeineStudioSeite`): `/studio/`, `/studio/index.html`, `/studio.html`, `GET /api/studio/ping` und `POST /api/studio/rpc/…` antworten 404. Dafür beantwortet der Server unbekannte `/api/`-Adressen jetzt für jede Methode mit 404 (vorher 405 aus `static` bei POST). `tests/projectRules.test.ts` › „Keine Studio-Seite im Spielserver (B-507)“: keine `studio.html`, keine Kachel mit `studio`, `task build` ruft `studio:frontend` nicht mehr auf, das Studio baut nicht nach `dist/studio`, keine `/api/studio`-Regel in `vite.config.ts`, `K3C_STUDIO_LAN` kommt im Code (`src`, `engine`, `cmd`, `tools`; `.ts/.tsx/.go`) nicht vor. Nach `task build` gibt es kein `dist/studio`.
- Entfernt: Kachel `studio` (Landingpage) und Dev-Kachel `studio/index.html`, `studio.html` samt Script `src/tools/studioPage.ts`, Vite-Ausnahme `isStudioPath`, `task: studio:frontend` in `build`.
- Studio-Frontend baut nach `tools/k3c-studio/frontend/dist/` (`paths.ts`, Taskfile `generates`); `tools/k3c-studio/server.go` liefert `/studio/` aus diesem Ordner (`uiDir`), `checkBuilt` prüft ihn.
- CI-Job `k3c-studio` (aus dem Review von TS1.1): läuft wieder auf Windows (Wails im Modul), `go-version-file` auf `tools/k3c-studio/go.mod`, zusätzlich `go test` und `golangci-lint` mit `working-directory: tools/k3c-studio`; Kommentar ohne `engine/studio`.
- Prüfungen: `task:check` grün, `task:check:go` grün, `task:build` grün, `check:studio` grün (ein erster Lauf scheiterte einmalig an „Access is denied“ beim Umbenennen in `.work/tmp`, Wiederholung grün).
- Abweichungen: Oxlint-Fehler `complexity` (17 > 15) in `tools/k3c-studio/frontend/src/stage/LivePreview.tsx` behoben, Ausnahme zur Dateiliste vom Orchestrator erlaubt: rein mechanische Extraktion der Planer-Markierungen in den Hook `usePlanerMarks`, ohne Verhaltensänderung; der Fehler bestand schon auf `origin/develop` (Datei dort gleich, seit #392). `src/tools/studioPage.ts` gelöscht (Script der gestrichenen `studio.html`). `engine/net/handler.go` bekommt eine `/api/`-Auffangroute (404). Texte `landing.studio*`, `studio.*` (`src/core/texts.*.ts`) und `tile.studio.*` (`src/tools/texts.*.ts`) sind jetzt ungenutzt, bleiben aber stehen (Dateien nicht erlaubt). `Handler(remote bool)` im Studio-Backend bleibt, nur die Kommentare zu `K3C_STUDIO_LAN` sind angepasst (keine Funktionsänderung). Kein Commit (Orchestrator).
