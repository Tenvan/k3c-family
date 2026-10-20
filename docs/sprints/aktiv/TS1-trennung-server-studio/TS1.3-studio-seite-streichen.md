# TS1.3 · Studio-Seite des Servers streichen

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
