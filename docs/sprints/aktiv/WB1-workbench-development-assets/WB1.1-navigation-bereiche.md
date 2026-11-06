# WB1.1 · Navigation Development/Assets, Routen inkl. ReviewPage

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** ui
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** wb1/1-navigation-bereiche
- **Abhängig von:** PV1.8
- **Tickets:** B-699
- **Kriterien:** AC-01

## Ziel

Die Kopfleiste der Workbench schaltet zwischen den Bereichen **Development** (alle heutigen Seiten) und **Assets** (Review, Katalog, Styleguide, Sound, Quellen); Review ist die Startseite von Assets und zeigt den Platzhalter `ReviewPage` aus PV1.8. Der Studio-Verweis ist weg, die Asset-RPC-Methoden sind im Frontend typisiert erreichbar.

## Kontext

- Entscheidung `docs/decisions/005-papierschnitt-assets-mcp.md` › 5 (Workbench), Vertrag `docs/standards/paper-mcp.md` › 4 Oberfläche. Die Oberfläche spricht nur RPC: Wails-Bindings bzw. `POST /api/dev/rpc/{Method}` (Allowlist `uiMethods` in `tools/k3c-workbench/uiapi.go`, gleich gehalten mit `GoApp` in `frontend/src/api/wails.ts` durch `TestUIMethodsWieWailsTS`). Kein REST.
- Ist-Stand Frontend `tools/k3c-workbench/frontend/src/`: `App.tsx` hängt sieben Seiten als `Tabs.Content` in `AppFrame` (`@webkit/templates`, Props `tabs`, `right`, `overlay`); `Header.tsx` hält `PAGES`/`LABELS` und `HeaderRight` (StudioLink, MCP-Badge, Mock-Badge, Dark-Schalter); `StudioLink.tsx` ruft `backend.studioOpen()` (startet `bin/k3c-studio.exe`). Gemerkte Seite: `loadPref('app.page', …)` aus `lib/prefs.ts`. `keep()` hält Dienste, Tasks, Git, Monitoring, Balancing eingehängt.
- Seit PV1.8 (`origin/develop` nach Merge von PV1): `GoApp` in `wails.ts` kennt `AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts`, `AssetsReview`, `AssetsFrame`, `AssetsPixelDiff` mit Typ `unknown`; Backend-Stubs in `internal/assetsui/stubs.go` antworten mit `tools/paper/fixtures/ui/<Methode>.json` (nur Asset `monarch`). `frontend/src/assets/review/index.tsx` exportiert `ReviewPage` – **gehört RV1, nicht ändern**.
- Atomic Design (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme, `tests/atomicDesign.test.ts`): Radix nur über `@webkit/*`, rohe Controls nur in `tools/webkit/atoms/`, Importe nur nach unten; eigene Organismen der App liegen unter `frontend/src/ui/organisms/`. Für den Bereichsumschalter gibt es das Molekül `ChipSwitch` (`@webkit/molecules`).
- Typen und Zugriff der Asset-Methoden legt WB1 einmal unter `frontend/src/assets/api.ts` an (Typen genau laut Vertrag: `AssetEintrag`, `Spec`, `Stil`, `Vorlage`, `Review`, `Frame`, `PixelDiff`); RV1 darf sie nur importieren (Absprache in der Sprint-README, D-02).

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/App.tsx`, `App.test.tsx`, `Header.tsx`
- `tools/k3c-workbench/frontend/src/StudioLink.tsx`, `StudioLink.test.tsx` (löschen)
- `tools/k3c-workbench/frontend/src/api/` (`types.ts`, `wails.ts`, `http.ts`, `mock.ts`: `studioOpen` entfernen, Asset-Methoden typisieren)
- `tools/k3c-workbench/frontend/src/assets/` (neu: `api.ts`, `AssetsArea.tsx` u. a.; **ohne** `assets/review/`)
- `tools/k3c-workbench/frontend/src/ui/organisms/` (neu: Bereichsumschalter, falls nötig)
- `tools/k3c-workbench/frontend/paths.ts` (nur `MOCK_DATA` um `tools/paper/fixtures/ui` ergänzen)
- `tools/k3c-workbench/frontend/src/codemap.md`, `frontend/src/api/codemap.md`

## Nicht-Ziele

Keine Inhalte von Katalog, Styleguide, Sound, Quellen (WB1.2, WB1.4: hier nur Platzhalter-Seiten mit Überschrift); kein Ändern von `assets/review/` (RV1); Go-Seite (`app_studio.go`, `uiMethods` mit `StudioOpen`) bleibt bis WB1.4.

## Schritte

1. **Rot:** `App.test.tsx` erweitern: Kopfleiste zeigt den Bereichsumschalter „Development | Assets“; unter Development sind alle sieben Reiter (`dienste` … `balancing`) erreichbar wie bisher; Wechsel auf Assets zeigt die Reiter Review, Katalog, Styleguide, Sound, Quellen mit Review als Start und dem Text „Review folgt mit RV1“; Bereich und Seite je Bereich werden gemerkt (`app.area`, `app.page`, `app.assetsPage`); kein Knopf „Asset-Studio öffnen“. Test für `assets/api.ts`: der Mock liefert für jede der sieben Methoden die Fixture `tools/paper/fixtures/ui/<Methode>.json` (JSON-Import per relativem Pfad, `MOCK_DATA` in `frontend/paths.ts` um den Fixture-Ordner ergänzen; keine Kopie der Fixtures anlegen). `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts App assets` → rot.
2. `assets/api.ts`: Typen laut Vertrag (Feldnamen deutsch wie im Vertrag), Funktionen `assetsList()` … `assetsPixelDiff()` über `backend`; `Backend` in `api/types.ts` um die sieben Methoden erweitern, Umsetzung in `wails.ts` (Typen statt `unknown`), `http.ts` (RPC wie die übrigen Methoden) und `mock.ts` (Fixtures).
3. `Header.tsx`: `AREAS = ['development', 'assets']`, `PAGES` (Development, unverändert) und `ASSET_PAGES = ['review', 'katalog', 'styleguide', 'sound', 'quellen']` mit Labels; `StudioLink` aus `HeaderRight` entfernen, `StudioLink*.tsx` löschen, `studioOpen` aus `types.ts`/`mock.ts`/`http.ts` entfernen.
4. `App.tsx`: Bereichsumschalter (`ChipSwitch` aus `@webkit/molecules`, in `right` vor dem MCP-Badge oder als eigener Organismus in `src/ui/organisms/AreaSwitch.tsx`); `AppFrame.tabs` zeigt die Reiter des aktiven Bereichs. Development-Seiten bleiben mit `keep()` eingehängt, auch während Assets aktiv ist (laufende Ansichten verlieren keinen Zustand). `assets/AssetsArea.tsx` rendert `ReviewPage` und Platzhalter für Katalog, Styleguide, Sound, Quellen.
5. Codemaps `frontend/src/codemap.md` und `api/codemap.md` nachziehen.
6. Grün: gezielte Vitest-Läufe, dann einmal `task check:diff`.
7. Commit `feat(dev): Bereiche Development und Assets in der Workbench (WB1.1)`.

## Fertig, wenn

- [ ] AC-01: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts App` grün: Umschalter Development/Assets, alle sieben bisherigen Seiten unter Development erreichbar, Assets startet mit `ReviewPage`.
- [ ] Asset-Methoden typisiert: Vitest `assets/api` grün (Mock = Fixtures); `go test ./...` in `tools/k3c-workbench` mit `TestUIMethodsWieWailsTS` grün (`check_run dev:test`).
- [ ] Kein Verweis auf `studioOpen` mehr im Frontend (`rg studioOpen tools/k3c-workbench/frontend/src` leer); `tests/atomicDesign.test.ts` grün (`task test -- atomicDesign`).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Während der Arbeit nur gezielt: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts <muster>`, `task test -- atomicDesign`; mit k3c-workbench `check_run dev:test` bzw. `task:check:diff` mit `checkout`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
