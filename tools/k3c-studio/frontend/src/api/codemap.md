# tools/k3c-studio/frontend/src/api/

## Responsibility
Schnittstelle der Oberfläche zum Go-Backend des Asset-Studios: Typen, Wails-Bindings und ein Mock für Browser und Tests; wählt das Backend je nach Laufzeit.

## Design
- `types.ts`: Typen der Antworten wie Go sie liefert (Slot-Register mit `stand`, Katalog, Recherche, `WorldSource`, `PlanView`/`PlannedChunk`, `WorldPreviewArgs`, Planer `HubPlanView`/`HubMatrix`/`HubSaveResult`, Ereignisse) und das Interface `Backend` (`assets*`, `worldLevel`, `worldPreview`, `plan*`, `worldOpenGame`, `hub*`, `openUrl`, `on`).
- `wails.ts`: `hasWails()` prüft `window.go`; `wailsBackend()` ruft die Bindings der App (bewusst ohne die generierten `wailsjs/`-Dateien, damit der Typecheck keine Wails-CLI braucht) und abonniert `assets:changed`.
- `mock.ts` setzt `mockBackend()` aus `mockAssets.ts` (Slot-Register mit Stand, Schreibregel, Fremdschreiber `external`), `mockWorld.ts` (Level, Vorschau mit `previewFixture.json`, Im-Spiel-Öffnen), `mockPlan.ts` (geplante Chunks im Speicher, Prüfung nur mit wenigen Fehlern) und `mockHub.ts` (Planer mit `hubFixture.json`, gleich zum Go-Backend) zusammen.
- `stale.ts`: `STALE` und `isStale` erkennen die Ablehnung „Register geändert“.
- `index.ts`: exportiert `backend` (Wails oder Mock), die Typen und `isStale`.

## Flow
1. Beim Laden von `index.ts` wird `backend` einmal gewählt: `hasWails()` → Wails-Bindings, sonst Mock (Hinweis „Mock“ in `App.tsx`).
2. Seiten rufen `backend.<methode>()`; Schreibaufrufe tragen den `stand` der letzten Ladung, abgelehnte Aufrufe kommen als `stale`/`error` zurück.
3. Ereignisse (`assets:changed`) laufen über `backend.on` und liefern die Abmeldung.

## Integration
- Gegenstück: `tools/k3c-studio/app*.go` (Bindings) und `tools/assetkit`.
- Konsumenten: alle Seiten (`assets/`, `grafik/`, `sound/`, `world/`, `planer/`, `monarch/`, `gegner/`, `stage/`).
- Typimport `@game/model/types` (Welt für die Vorschau).
