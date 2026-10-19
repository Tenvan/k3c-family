# tools/k3c-studio/frontend/src/api/

## Responsibility
Schnittstelle der Oberfläche zum Studio-Backend des Go-Servers (`engine/studio`): Typen, HTTP-Backend und ein Mock für Browser ohne Server und Tests; wählt das Backend je nach Laufzeit.

## Design
- `types.ts`: Typen der Antworten wie Go sie liefert (Slot-Register mit `stand`, Katalog, Recherche, `WorldSource`, `PlanView`/`PlannedChunk`/`IslandSize`/`WorldsView`, `WorldPreviewArgs` mit `SiteLook` und `hubCandidate`, Planer `HubPlanView`/`HubMatrix`/`HubSaveResult`/`HubRulesView`, `OpenGameResult`, Ereignisse `Events`) und das Interface `Backend` (`mock`, `assets*`, `worldLevel`, `worldPreview`, `worldOpenGame`, `plan*` inkl. Welten und Inseln, `hub*`, `openUrl`, `on`).
- `http.ts`: `httpBackend()` ruft jede Methode als `POST /api/studio/rpc/<Methode>` (Argumente als JSON-Array, Antwort `{result}` oder `{error}`, Fehler werden zu `Error`); Kopfzeile `X-K3C-Studio` hält fremde Seiten fern. `assetsImport` öffnet die Dateiwahl des Browsers und lädt per `POST import?id&hit&name` hoch; `worldOpenGame` öffnet die vom Server gelieferte Adresse per `window.open`; `on` abonniert Server-Sent Events (`GET events`, ein gemeinsamer `EventSource`). `useMock()` ist wahr im Test (`MODE === 'test'`) und mit `?mock=1`.
- `mock.ts`: `mockBackend()` setzt `mockAssets.ts`, `mockWorld.ts`, `mockPlan.ts` und `mockHub.ts` zusammen; `external` spielt einen fremden Schreiber nach.
- `mockAssets.ts`: erfundenes Slot-Register und Katalog, Stand zählt jede Änderung hoch, Schreibregel (`STALE`), `external` mit `quiet` (Fenster vor `assets:changed`).
- `mockWorld.ts`: erfundenes Level je Seed (`kindsOf` nach Chunk-Anzahl, Seeds `fehler`/`warnung`), Vorschau-Welt aus `previewFixture.json` (Hub-Stufe, Phase, Gebäude-Auswahl, Kandidat), `mockOpenGame` (prüft den Seed wie Go, öffnet nichts).
- `mockPlan.ts`: Plan-Teil im Speicher (Welten mit Inseln, Tiefen 0–4, Planung je Insel, `MOCK_SIZES` gleich `data/island-sizes.json`, `mockCheckPlan` mit wenigen Fehlern, Stand `mock-<n>`, `touch` simuliert Änderung von außen, Seed-Inhalt für die Umwandlung).
- `mockWorlds.ts`: Welten-Teil des Mock-Plans (`MockWorld`, `MockIsland`, `newWorld`, `mockWorlds`: wählen, anlegen leer oder als Kopie, entfernen; Standard bleibt).
- `mockWorldDraft.ts`: Welt-Entwurf im Mock (`mockCheckWorld` mit den Fehlern der Prüfung des Ladens, die die Oberfläche zeigt, `planWorldSave` mit Stand; `HINT`, `staleText` wie in Go).
- `worldTypes.ts`: Typen des Welt-Entwurfs (`WorldDraft`, `IslandDraft`, `DraftScaling`, `WorldCheckView`, `WorldDraftApi`), getrennt von `types.ts`.
- `mockHub.ts`: Gebäude-Planer im Mock mit `hubFixture.json` (7-Stufen-Fixture, gleich zum Go-Backend).
- `hubFixture.json`, `previewFixture.json`: Antworten des Go-Backends als Daten für die Mocks.
- `stale.ts`: `STALE` und `isStale` erkennen die Ablehnung „Register geändert“ (Fehlertext aus Go als String oder `Error`).
- `index.ts`: exportiert `backend` (`useMock()` ? Mock : HTTP), alle Typen und `isStale`/`STALE`.
- Tests: `mockPlan.test.ts` (Inselgrößen gegen `data/island-sizes.json`, Stale-Text und REG-Hinweis wie Go).

## Flow
1. Beim Laden von `index.ts` wird `backend` einmal gewählt: `useMock()` → Mock (Hinweis „Mock“ in `App.tsx`), sonst `httpBackend()`.
2. Seiten rufen `backend.<methode>()`; Schreibaufrufe tragen den `stand` der letzten Ladung, abgelehnte Aufrufe kommen als `stale`/`error` zurück oder werfen.
3. Ereignisse (`assets:changed`) laufen über `backend.on` (SSE bzw. Mock-Bus) und liefern die Abmeldung.

## Integration
- Gegenstück: `engine/studio/` (`rpc.go` Routen unter `/api/studio/`, `app*.go` die aufrufbaren Methoden mit Präfix Assets/World/Plan/Hub) und `tools/assetkit`.
- Dev-Server: Vite (Port 5192) reicht `/api` an den Spielserver (8080) weiter, siehe `../../vite.config.ts`; ohne Server hilft `?mock=1`.
- Konsumenten: alle Seiten (`assets/`, `graphics/`, `sound/`, `world/`, `planner/`, `monarch/`, `enemies/`, `stage/`).
- Typimport `@game/model/types` (Welt für die Vorschau); `mockWorld.ts` nutzt `../stage/stageLogic`.
