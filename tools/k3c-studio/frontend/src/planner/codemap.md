# tools/k3c-studio/frontend/src/planner/

## Responsibility
Seite Gebäude-Planer (B-462, WZ9): zeigt und bearbeitet `data/hub.json` als Matrix aus Plätzen und Hub-Stufen, prüft jeden Stand im Go-Backend, vergleicht mit der Regeltabelle, zeigt die Live-Vorschau und speichert nach Schreibregel.

## Design
- `usePlanner.ts`: Zustand: geladene Datei (`hubPlan`), Änderungen als Kandidat (`edit`), Prüfung (`hubCheck`, letzte lesbare Prüfung bleibt sichtbar) und Abweichung der Regeltabelle (`hubRules`), Speichern (`hubSave` mit Stand beider Dateien) nur ohne Befund; verwirft veraltete Antworten. `usePlannerView.ts` reicht die Sicht an die Vorschau.
- `plannerEdit.ts`: reine Änderungen am Kandidaten (`parseHub`, `candidateText`, `setSiteField`, `setSiteKind`, `moveSite`, `setZone1Wall`, `setZone1Tower`, `setMerchant`, `addSite`, `removeSite`); prüft nichts.
- `plannerLogic.ts`: Anzeige (`rowLabel`, `rowPlace`, `offsetText`, `depthText`, `levelColumns`) und `rowOffset`, `rowAt` (Bauplatz der Bühne → Zeile, B-670).
- `PlannerPage.tsx` (B-670): Werkzeugleiste (`PlannerToolbar.tsx`: Quelle, Hub-Stufe und Tageszeit per `controlsHost`, Chips Prüfung und Regeltabelle mit Panel auf Klick, Änderungsanzeige, Verwerfen, Speichern), Live-Vorschau mit Chip-Leiste, darunter Matrix (`HubMatrixView.tsx`: Kästchen je Stufe, Platz und Liste unter dem Namen, „+ Platz“ im Kopf) und Inspektor (`PlannerEditor.tsx` › `PlannerInspector`: Felder der Zeile, Art, Liste, Vorschau-Zustand per `lookHost`, Hintergrund per `layersHost`). Eine Auswahl (`useRowPick`) für Bühne, Matrix und Platz-Chips.
- Tests: `plannerLogic.test.ts`, `plannerEdit.test.ts`, `PlannerEdit.test.tsx`, `PlannerPage.test.tsx`, `PlannerSelect.test.tsx`, `usePlannerView.test.tsx`.

## Flow
1. Laden → Matrix und Befunde aus dem Backend.
2. Änderung → neuer Kandidat → `hubCheck` und `hubRules` → Matrix und Vorschau folgen.
3. Speichern → `hubSave(candidate, stand)` → gespeichert, `stale` (nichts geschrieben) oder Fehler; bei Erfolg Neuladen.

## Integration
- Backend: `app_planer*.go`, `hub_patch.go`; Mock `../api/mockHub.ts`.
- Abhängigkeiten: `../stage/LivePreview`, `../api`, `@webkit/errors`; eingehängt in `../App.tsx` (Seite `planer`, Komponente `PlannerPage`).
