# src/tools/

## Responsibility

Presentation-Schicht der Test- und Info-Seiten (Entry-Scripts der Root-`*.html`): Gamepad-Test, Level-Betrachter, Szenario-Start, Sprite-/Grafik-Referenzen, Lizenzen, Monitoring, Hörprobe und Dungeon-Master-Seite. Reine Hilfsseiten neben dem Spiel, ohne eigene Spiel-Logik.

## Design

- Entry-Scripts (je eine Seite, rufen `installPageChrome()` auf): `dev.ts` (Entwicklerseite `dev.html`, B-335: Abschnitte Entwicklung, Performance, Balancing aus `DEV_SECTIONS`, `dm.html` in neuem Fenster, Pad-Fokus über `nextFocus`), `gamepadTest.ts`, `leveltest.ts`, `testing.ts`, `figuren.ts`, `aufstellung.ts`, `grafiken.ts`, `lizenzen.ts`, `monitor.ts` (Monitoring-Seite, B-282: Ampel je Raum, Verläufe, Fehler-Zeitleiste, Token in `localStorage`, Pad-Bedienung), `soundtest.ts` (Hörprobe SO3.2 über `Mixer`, Crossfade, Pad/Tastatur). `dm.ts` (Dungeon-Master-Seite `/dm`, B-232) läuft außerhalb der Shell ohne `installPageChrome()`.
- Reine Logik getrennt vom DOM (testbar): `levelView.ts` (`levelModel`, `startTarget`, `seedFromBytes`), `levelApi.ts` (`levelUrl`, `fetchLevel` mit injizierbarem `FetchLike`, nie Exception), `testScenarios.ts` (`SCENARIOS`, `scenarioUrl`, `saveName`), `testTiles.ts` (`LEVEL_TILES`, `nextFocus`), `devTiles.ts` (`DEV_SECTIONS`, `DEV_TILES`: Kacheln der Entwicklerseite), `selection.ts` (`formatSelection`, `parseSaved`), `credits.ts` (`parseCredits`, `CREDITS`, `isCcBy`, `renderCredits`), `audioProbe.ts` (`decodeFormats`, `buildAudioReport`, `audioRows`, `startAudioProbe`), `monitorData.ts` (`applyDelta`, `stats`, `outlierLimit`, `windowValues`, `roomSummaries`), `monitorApi.ts` (`fetchMetrics` mit `MetricsResult`, `nextDelay` mit Backoff, `POLL_MS` 3000); `monitorChart.ts` (`drawChart`, `ChartSpec`, `PALETTE`) zeichnet Verläufe auf Canvas; `dmApi.ts` (`fetchRooms`, `fetchDiagnose`, `sendAction`, `devUrl`, `DM_ACTIONS`, injizierbarer `FetchLike`); `soundtestLogic.ts` (`parseCandidates`, `groupCandidates`, `fileFor`, `crossfadeCurves`, `keyAction`, `padActions`, `moveSelection`, `stepVolume`).
- Datengetrieben: `spriteReference.ts` liest `data/sprites.json`, `enemies.json`, `troops.json` und zeichnet Figuren auf Canvas ohne Phaser (`allFigures`, `lineup`, `renderReference`, `installPadScroll`); `grafikPacks.ts` (`GRAFIK_PACKS`) listet die Grafik-Packs, auch nicht gewählte Kandidaten (`kandidat()`, GR2); die Hörprobe lädt `audio/kandidaten.json`.
- `credits.ts` bindet `public/grafik/CREDITS.md` und `public/sprites/CREDITS.md` per `?raw`-Import ein.

## Flow

1. `leveltest.ts`: Seed und Biom → `fetchLevel` (`GET /api/level`) → `levelModel` → Canvas; „Im Spiel starten“ via `startTarget` → `openPage('game.html?...')`.
2. `testing.ts`: Kacheln aus `SCENARIOS` und `LEVEL_TILES`; `scenarioUrl(scenario, nonce)` erzeugt `game.html?autostart=1&fresh=1&save=NAME&mock=N`, Fokus per Controller über `nextFocus`.
3. `gamepadTest.ts`: Phaser-Gamepad, `startAudioProbe`, `toggleFullscreen()`; sendet den Bericht per `POST /api/report` (Server legt `reports/*.json` an).
4. `figuren.ts`/`aufstellung.ts`/`grafiken.ts`: `installSelection(key, label)` plus `renderReference(...)` bzw. `fetch('grafik/index.json')`; Auswahl in `localStorage`.
5. `lizenzen.ts`: `renderCredits(CREDITS)`.
6. `monitor.ts`: `poll()` ruft `fetchMetrics(token, since)`, `applyDelta` füllt den `MonitorState`, `render()` zeichnet Übersicht (`roomSummaries`), Verlauf (`drawChart`) oder Ereignisse; Polling nur bei sichtbarer Seite, Fehler verlängern über `nextDelay`.
7. `dm.ts`: `poll()` (1 s) → `fetchRooms`/`fetchDiagnose` (`GET /api/dev`), Aktionen aus `DM_ACTIONS` per `sendAction`.
8. `soundtest.ts`: `init()` lädt Kandidaten, `act(Action)` spielt/blendet über `Mixer`.

## Integration

- Eingebunden von `gamepad-test.html`, `leveltest.html`, `testing.html`, `figuren.html`, `aufstellung.html`, `grafiken.html`, `lizenzen.html`, `monitor.html`, `soundtest.html`, `dev.html`, `dm.html`; Kacheln in `src/tools/devTiles.ts` (Entwicklerseite), die Landingpage (`src/landing/pages.ts`) führt nur über die kleine Kachel „Entwicklung“ dorthin.
- Abhängigkeiten: `src/core/shell.ts` (`installPageChrome`, `openPage`), `src/core/fullscreen.ts`, `src/model/` (`BIOMES`, `LevelLayout`), `data/*.json`, `public/grafik/`, `public/sprites/`; Endpoints `/api/level`, `/api/report`, `/api/metrics`, `/api/dev`; `src/audio/mixer.ts`.
