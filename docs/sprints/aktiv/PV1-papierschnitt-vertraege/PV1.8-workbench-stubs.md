# PV1.8 · Workbench-Stubs: RPC-Methoden Assets, Review-Platzhalter, Slider-Atom

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** pv1/8-workbench-stubs
- **Abhängig von:** PV1.4
- **Tickets:** B-699
- **Kriterien:** AC-07

## Ziel

Auf `develop` gibt es die RPC-Methoden der Asset-Oberfläche als Stubs mit Fixture-Antworten, verdrahtet in Wails und Web, einen Platzhalter für die Review-Seite und ein Slider-Atom; danach bauen WB1 und RV1 parallel, ohne sich gegenseitig zu brauchen.

## Kontext

- Die Oberfläche spricht nur über Wails-Bindings bzw. `POST /api/dev/rpc/{Method}` mit der Allowlist `uiMethods` (`tools/k3c-workbench/uiapi.go`); `TestUIMethodsWieWailsTS` hält sie gleich mit `frontend/src/api/wails.ts`. Kein REST.
- Vertrag: `docs/standards/paper-mcp.md` › Oberfläche (PV1.4). Fixture-Antworten: `tools/paper/fixtures/ui/*.json`.
- Atomic Design: rohe Controls nur in `tools/webkit/atoms/` (`tests/atomicDesign.test.ts`).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/assetsui/` (neu: Stub-Methoden)
- `tools/k3c-workbench/app_assets.go` (neu: Methoden an `App` binden)
- `tools/k3c-workbench/uiapi.go` (nur `uiMethods` ergänzen)
- `tools/k3c-workbench/frontend/src/api/wails.ts`
- `tools/k3c-workbench/frontend/src/assets/review/index.tsx` (Platzhalter)
- `tools/webkit/atoms/Slider.tsx` und Test

## Nicht-Ziele

Keine Navigation (WB1), keine echte Logik (RV1).

## Schritte

1. Go-Test `TestAssetsUIStubs`: jede Methode `AssetsReview`, `AssetsFrame`, `AssetsPixelDiff`, `AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts` liefert die Fixture-Antwort; `TestUIMethodsWieWailsTS` bleibt grün. Rot.
2. `internal/assetsui`: Typen und Stubs laut Vertrag; `app_assets.go` bindet sie; `uiMethods` und `wails.ts` ergänzen.
3. `review/index.tsx` exportiert `ReviewPage` (Text „Review folgt mit RV1“).
4. `Slider` in `tools/webkit/atoms` (Radix Slider über `@webkit`), Vitest.
5. Commit `feat(dev): Stubs der Asset-Oberfläche für parallele Lanes`.

## Fertig, wenn

- [ ] AC-07: Go- und Vitest-Tests grün; `task check:dev` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
