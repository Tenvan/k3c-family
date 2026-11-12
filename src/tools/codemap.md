# src/tools/

## Responsibility

Presentation-Schicht der verbleibenden Werkzeug-Seiten (Entry-Scripts der Root-`*.html`): Lizenzen und Monitoring; dazu die Kacheln des Spielmenüs (`devTiles.ts`). Reine Hilfsseiten neben dem Spiel, ohne eigene Spiel-Logik. Testseiten (Gamepad-Test, Hörprobe, Dungeon Master, Stress, Textur, Szenario-Start) sind entfernt (B-822).

## Design

- Entry-Scripts (je eine Seite, rufen `installPageChrome()` auf): `lizenzen.ts`, `monitor.ts` (Monitoring-Seite, B-282: Ampel je Raum, Verläufe, Fehler-Zeitleiste, Token in `localStorage`, Pad-Bedienung über `nextFocus`).
- Reine Logik getrennt vom DOM (testbar): `testTiles.ts` (`nextFocus`), `devTiles.ts` (`DEV_SECTIONS`, `DEV_TILES`: Kacheln im Spielmenü), `credits.ts` (`parseCredits`, `CREDITS` = Sound-Credits, `isCcBy`, `renderCredits`), `padScroll.ts` (Pad-Scrollen von `lizenzen.ts`), `monitorData.ts` (`applyDelta`, `stats`, `outlierLimit`, `windowValues`, `roomSummaries`), `monitorApi.ts` (`fetchMetrics` mit `MetricsResult`, `nextDelay` mit Backoff, `POLL_MS` 3000); `monitorChart.ts` (`drawChart`, `ChartSpec`, `PALETTE`) zeichnet Verläufe auf Canvas.

- Texte: `texts.ts` (`t`, `textOf`, `applyTexts`: eigene Tabellen der Werkzeug-Seiten, Sprache aus `currentLanguage()`, Rückfall Deutsch), `texts.de.ts` (`de`, `ToolTextKey`: Quelle der Schlüssel), `texts.en.ts` (`en`); alle Entry-Scripts und Logik-Module beziehen ihre Texte darüber.
- `credits.ts` bindet nur die Sound-Credits `public/audio/README.md` per `?raw`-Import ein; die Grafik ist eigener Papierschnitt (CU1.3).

## Flow

1. `lizenzen.ts`: `renderCredits(CREDITS)`.
2. `monitor.ts`: `poll()` ruft `fetchMetrics(token, since)`, `applyDelta` füllt den `MonitorState`, `render()` zeichnet Übersicht (`roomSummaries`), Verlauf (`drawChart`) oder Ereignisse; Polling nur bei sichtbarer Seite, Fehler verlängern über `nextDelay`.

## Integration

- Eingebunden von `lizenzen.html` und `monitor.html`; Kacheln in `src/tools/devTiles.ts`, angezeigt vom Spielmenü (Seiten › Kategorie, `src/scenes/menuPages.ts`).

- Abhängigkeiten: `src/core/shell.ts` (`installPageChrome`, `openPage`), `data/*.json`; Endpoint `/api/metrics`.
