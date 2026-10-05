# src/tools/

## Responsibility

Presentation-Schicht der Test- und Info-Seiten (Entry-Scripts der Root-`*.html`): Gamepad-Test, Level-Betrachter, Szenario-Start, Sprite-/Grafik-Referenzen und Lizenzen. Reine Hilfsseiten neben dem Spiel, ohne eigene Spiel-Logik.

## Design

- Entry-Scripts (je eine Seite, rufen `installPageChrome()` auf): `gamepadTest.ts`, `leveltest.ts`, `testing.ts`, `figuren.ts`, `aufstellung.ts`, `grafiken.ts`, `lizenzen.ts`, `monitor.ts` (Monitoring-Seite, B-282).
- Reine Logik getrennt vom DOM (testbar): `levelView.ts` (`levelModel`, `startTarget`, `seedFromBytes`), `levelApi.ts` (`levelUrl`, `fetchLevel` mit injizierbarem `FetchLike`, nie Exception), `testScenarios.ts` (`SCENARIOS`, `scenarioUrl`, `saveName`), `testTiles.ts` (`LEVEL_TILES`, `nextFocus`), `selection.ts` (`formatSelection`, `parseSaved`), `credits.ts` (`parseCredits`, `CREDITS`, `isCcBy`, `renderCredits`), `audioProbe.ts` (`decodeFormats`, `buildAudioReport`, `audioRows`, `startAudioProbe`), `monitorData.ts` (`applyDelta`, `stats`, `outlierLimit`, `windowValues`, `roomSummaries`), `monitorApi.ts` (`fetchMetrics`, `nextDelay`); `monitorChart.ts` zeichnet Verläufe auf Canvas.
- Datengetrieben: `spriteReference.ts` liest `data/sprites.json`, `enemies.json`, `troops.json` und zeichnet Figuren auf Canvas ohne Phaser (`allFigures`, `lineup`, `renderReference`, `installPadScroll`); `grafikPacks.ts` (`GRAFIK_PACKS`) listet die Grafik-Packs.
- `credits.ts` bindet `public/grafik/CREDITS.md` und `public/sprites/CREDITS.md` per `?raw`-Import ein.

## Flow

1. `leveltest.ts`: Seed und Biom → `fetchLevel` (`GET /api/level`) → `levelModel` → Canvas; „Im Spiel starten“ via `startTarget` → `openPage('game.html?...')`.
2. `testing.ts`: Kacheln aus `SCENARIOS` und `LEVEL_TILES`; `scenarioUrl(scenario, nonce)` erzeugt `game.html?autostart=1&fresh=1&save=NAME&mock=N`, Fokus per Controller über `nextFocus`.
3. `gamepadTest.ts`: Phaser-Gamepad, `startAudioProbe`, `toggleFullscreen()`; sendet den Bericht per `POST /api/report` (Server legt `reports/*.json` an).
4. `figuren.ts`/`aufstellung.ts`/`grafiken.ts`: `installSelection(key, label)` plus `renderReference(...)` bzw. `fetch('grafik/index.json')`; Auswahl in `localStorage`.
5. `lizenzen.ts`: `renderCredits(CREDITS)`.

## Integration

- Eingebunden von `gamepad-test.html`, `leveltest.html`, `testing.html`, `figuren.html`, `aufstellung.html`, `grafiken.html`, `lizenzen.html`, `monitor.html`; Kacheln in `src/landing/pages.ts`.
- Abhängigkeiten: `src/core/shell.ts` (`installPageChrome`, `openPage`), `src/core/fullscreen.ts`, `src/model/` (`BIOMES`, `LevelLayout`), `data/*.json`, `public/grafik/`, `public/sprites/`; Endpoints `/api/level`, `/api/report`, `/api/metrics`.
