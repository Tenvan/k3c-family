# tools/k3c-workbench/frontend/src/planner/

## Responsibility

Seite **Gebäude-Planer** (B-923, B-462, B-670; Reiter `planer` im Bereich Studio, Port aus dem früheren Asset-Studio): Die Matrix Hub-Stufe × Bauplatz aus `data/hub.json` ansehen, Plätze im Inspektor bearbeiten (Art, Liste, Stufe, Platz, Tiefe, Baugrund der Zone 1, Händler) und mit Prüfung und Regeltabellen-Abweichung aus dem Go-Backend speichern. Das Frontend formatiert und ändert nur den Kandidaten; geprüft wird im Backend (`internal/hubplan`).

## Design

- `PlannerPage.tsx`: Wurzel; `PlannerToolbar` oben, darunter bei geladenem Stand `PlanerBody` (Matrix | `Splitter` | Inspektor, Anteil `planer.split` über `useSplit`, Vorgabe 64 %). Ladefehler als `NoticeCard`, Fehler der Datei als roter Text. Hub-Stufe wählbar (gemerkt nur im Zustand), Auswahl meldet sich per `useTicketPage('planer', zeile)`. „+ Platz“ legt einen Platz in `sites` an und wählt ihn.
- `usePlanner.ts`: Zustand der Seite aus drei Teilen. `useLoaded` lädt `backend.hubPlan()` (jede Ladung ersetzt die vorige und verwirft Änderungen, veraltete Antworten fallen weg). `useChecked` fragt für den Kandidaten `hubRules` und bei Änderungen `hubCheck`; `lastGood` hält die letzte lesbare Prüfung, damit die Matrix bis zur nächsten Antwort steht. `usePlanner` führt Kandidat (`candidateText`), `edit(change)` (während des Speicherns gesperrt), Speichern über `hubSave` mit dem Stand beim Laden (Schreibregel: extern geänderte Datei, `stale`) und das Ergebnis `HubSaveResult`.
- `plannerEdit.ts` (ohne React): `HubJson`/`HubSiteJson` (nur die Teile, die der Planer ändert, der Rest der hub.json bleibt), `parseHub`, `candidateText`, `integerOf`, `setSiteKind`, `moveSite` (zwischen `sites` und `islandSites`), `addSite`.
- `plannerLogic.ts` (ohne React): Anzeige der Matrix – `rowLabel` (Spieltext der Bauplätze, „Baugrund Zone n“, Händler), `rowPlace`, Platz und Tiefe als Text.
- `HubMatrixView.tsx`: Matrix aus dem Modell des Backends (Zeilen = Bauplätze, Spalten = Hub-Stufen), Zellen als Kästchen (ab der Stufe des Platzes kräftig), `rowId` als stabile Zeilenkennung für die Auswahl.
- `PlannerEditor.tsx`: `PlannerInspector` mit den Feldern der gewählten Zeile; Zahlenfelder behalten den getippten Text und melden nur gültige Zahlen; Entfernen mit Hinweis auf alte Spielstände.
- `PlannerToolbar.tsx`: Werkzeugleiste in einer Zeile – Quelle, Hub-Stufe als Segment-Knöpfe, Status-Chips „Prüfung“ (Befunde aus `sim.ValidateHub`) und „Regeltabelle“ (nur bei Abweichung), Panels unter der Leiste, Speichern mit Grund einer Sperre.
- Stile: `../styles/planer.css`.

## Flow

1. Mount -> `usePlanner` -> `backend.hubPlan()` -> `HubPlanView` (Matrix, Befunde, `kinds`, Stand der Dateien).
2. Eingabe im Inspektor oder „+ Platz“ -> `edit` -> `plannerEdit` ändert den Kandidaten -> `hubCheck(candidate)` und `hubRules(candidate)` -> Matrix und Chips zeigen den neuen Stand.
3. Speichern -> `hubSave(candidate, stand)`; bei `stale` wird nichts geschrieben, bei Erfolg lädt die Seite neu.

## Integration

- Konsument: `src/App.tsx` (`Tabs.Content value="planer"`), Reiter in `Header.tsx` (`ASSET_PAGES`, Label „Gebäude-Planer“).
- Backend: `Hub*` in `tools/k3c-workbench/app_hub.go` (Paket `internal/hubplan`); Verträge `api/typesHub.ts`, Mock `api/mockHub.ts` mit der Fixture `api/hubFixture.json`.
- Abhängigkeiten: `src/api`, `lib/prefs`, `@webkit/atoms|molecules`, `@webkit/lib/split`, `@webkit/lib/ticketContext`, `@webkit/lib/errors`.
- Tests: `PlannerPage.test.tsx`, `PlannerEdit.test.tsx`, `PlannerSelect.test.tsx`, `plannerEdit.test.ts`, `plannerLogic.test.ts`.
