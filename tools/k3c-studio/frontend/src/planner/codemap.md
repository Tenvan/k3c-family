# tools/k3c-studio/frontend/src/planner/

## Responsibility
Seite Gebäude-Planer (B-462, WZ9): zeigt und bearbeitet `data/hub.json` als Matrix aus Plätzen und Hub-Stufen, prüft jeden Stand im Go-Backend, vergleicht mit der Regeltabelle, zeigt die Live-Vorschau und speichert nach Schreibregel.

## Design
- `usePlanner.ts`: Zustand: geladene Datei (`hubPlan`), Änderungen als Kandidat (`edit`), Prüfung (`hubCheck`, letzte lesbare Prüfung bleibt sichtbar) und Abweichung der Regeltabelle (`hubRules`), Speichern (`hubSave` mit Stand beider Dateien) nur ohne Befund; verwirft veraltete Antworten. `usePlannerView.ts` reicht die Sicht an die Vorschau.
- `plannerEdit.ts`: reine Änderungen am Kandidaten (`parseHub`, `candidateText`, `setSiteField`, `setWallUnit`, `setGateHubLevel`, `setMerchant`, `addSite`, `removeSite`); prüft nichts.
- `plannerLogic.ts`: nur Anzeige (`rowLabel`, `offsetText`, `depthText`, `levelColumns`).
- `PlannerPage.tsx`: Quelle, Befunde, Abweichung, Matrix, Live-Vorschau (`LivePreview` mit Planer-Stand), Speichern; `HubMatrixView.tsx` Matrix mit wählbarer Hub-Stufe je Spalte; `PlannerEditor.tsx` `RowEditor` und `AddSite`.
- Tests: `plannerLogic.test.ts`, `plannerEdit.test.ts`, `PlannerEdit.test.tsx`, `PlannerPage.test.tsx`, `usePlannerView.test.tsx`.

## Flow
1. Laden → Matrix und Befunde aus dem Backend.
2. Änderung → neuer Kandidat → `hubCheck` und `hubRules` → Matrix und Vorschau folgen.
3. Speichern → `hubSave(candidate, stand)` → gespeichert, `stale` (nichts geschrieben) oder Fehler; bei Erfolg Neuladen.

## Integration
- Backend: `app_planer*.go`, `hub_patch.go`; Mock `../api/mockHub.ts`.
- Abhängigkeiten: `../stage/LivePreview`, `../api`, `@webkit/errors`; eingehängt in `../App.tsx` (Seite `planer`, Komponente `PlannerPage`).
