# tools/k3c-studio/frontend/src/world/

## Responsibility
Seite Welten Editor (Welt-Betrachter, B-460/B-461/B-477): Chunk-Streifen eines Levels, Live-Vorschau, Editor für geplante Chunks und „Im Spiel öffnen“.

## Design
- `WorldPage.tsx`: eine Ansicht mit Chunk-Streifen oben (`ChunkPreview`) und Live-Vorschau darunter (`../stage/LivePreview`); Position und Zoom liegen hier und gelten für beide; Taste M wechselt kompakt/maximiert.
- `ChunkPreview.tsx`: Streifen mit Abschnitten, Objekten, Hub-Linie, Legende, Warnungen; Mausrad meldet `onPan`; nur die letzte Anfrage (`backend.worldLevel`) zählt. `levelView.ts`: Zeichenmodell (`levelModel`), Zufalls-Seed, `openGameReason`.
- `viewState.ts`: reine Funktionen: Zoom um die Mitte (`zoomView`), `clampLeft`, `wheelAction`, `jumpPoints` (Anfang, Mitte, Ende, Hub, Tor, Portale, Ausgang); `ViewBar.tsx` Zoom und Sprungpunkte.
- `plan.ts`, `usePlan.ts`, `PlanEditor.tsx`: Entwurf geplanter Chunks einer Tiefe (hinzufügen, ändern, verschieben, entfernen), Prüfung verzögert (`planCheck`), Speichern mit Stand (`planSave`), Meldungen `hint`, `error`, `stale`.
- `GameLauncher.tsx`: „Im Spiel öffnen“ und „Voll ausgebaut öffnen“ (`worldOpenGame`), Adresse des Spielservers gemerkt.
- Tests: `levelView.test.ts`, `viewState.test.ts`, `plan.test.ts`, `PlanEditor.test.tsx`, `GameLauncher.test.tsx`, `WorldPage.test.tsx`.

## Flow
1. Seed und Biom → `worldLevel` → Streifen; dieselben Angaben → `worldPreview` → Phaser-Bühne.
2. Planänderung → Prüfung → gültiger Stand geht an Streifen und Vorschau → Speichern schreibt `data/islands.json`.
3. Schaltfläche → `worldOpenGame` → Browser auf dem Spielserver.

## Integration
- Backend: `app_world.go`, `app_preview.go`, `app_plan.go`, `plan_file.go`, `app_game.go`; Mock `../api/mockWorld.ts`, `../api/mockPlan.ts`.
- Abhängigkeiten: `../stage/`, `../api`, `../lib/prefs`; eingehängt in `../App.tsx` (Seite `welt`).
