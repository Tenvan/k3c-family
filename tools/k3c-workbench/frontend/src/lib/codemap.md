# tools/k3c-workbench/frontend/src/lib/

## Responsibility

Kleine, seitenunabhängige Hilfsfunktionen und React-Hooks von k3c-workbench: deutsche Formatierung, gemerkte Einstellungen (Präfix), Markdown-Parser, gedrosseltes Nachladen bei Live-Ereignissen und Zeit-Hook. Fehlertext, Debounce und Splitter-Rechnung kommen aus `tools/webkit` (`@webkit/lib/errors`, `@webkit/lib/useDebounced`, `@webkit/lib/split`).

## Design

- Pure Utilities: `format.ts` (`formatNumber`, `formatPercent`, `formatBytes`, `formatUptime`, `formatDuration`, `formatTime`, de-DE, deckungsgleich mit den Go-Formatern in `internal/mcpsrv`).
- Persistenz: `prefs.ts` bindet `createPrefs('k3c-workbench.')` aus `@webkit/lib/prefs` (`loadPref`, `loadText`, `savePref`, `loadJSON`, `saveJSON`, `isStringList`; Favoriten, Eingeklapptes) und `prefsStore` (`{ loadText, savePref }` für `useSplit`). Freitext-Filter werden nicht gemerkt.
- `markdown.ts` liegt seit WZ63.6 in `@webkit/lib/markdown` (gezeichnet von `MarkdownView` in `@webkit/molecules`); nie ein HTML-String.
- `liveReload.ts` (rein, B-895): `liveReload(run, everyMs, doc)` bündelt Nachlade-Wünsche – der erste nach einer Pause sofort, weitere höchstens einmal je Takt, nicht bei verborgenem Fenster; beim Sichtbarwerden wird ein verpasstes Nachladen einmal nachgeholt. Hintergrund: im Wails-Fenster kommt jede Antwort als eigenes Skript, ungedrosseltes Nachladen ließ den WebView2-Renderer wachsen.
- Hooks: `useLiveReload.ts` (`useLiveReload(events, run, everyMs, onEvent?)`: lädt beim Einhängen und gedrosselt bei jedem Ereignis über `backend.on`; der optionale `onEvent` sieht jede Nutzlast (`Events[E]`) sofort und ungedrosselt, z. B. um geänderte IDs zu sammeln, bis `run` sie abholt; `PLAN_RELOAD_MS = 15 s` für die Planung), `useNow.ts` (Intervall-Timestamp).

## Flow

1. Eine Seite liest ihren Zustand initial mit `loadPref('key', ALLOWED, fallback)` bzw. `loadJSON('key', valid, fallback)` und schreibt bei Änderung mit `savePref`/`saveJSON`.
2. Live-Ereignis (z. B. `planning:changed`) -> `useLiveReload` -> `liveReload(...).soon()` -> höchstens ein `run()` je Takt.
3. Anzeige-Komponenten formatieren Zahlen/Zeiten über `format.ts`; abgelehnte Promises werden mit `errorText` in `NoticeCard`s gezeigt.
4. Zeitabhängige Anzeigen (Uptime, Live-Fenster) ticken über `useNow(ms)`.

## Integration

- Konsumenten: `App.tsx`, `Header.tsx`, `assets/`, `balancing/`, `logs/`, `mcp/`, `monitoring/`, `planning/`, `services/`, `tasks/`, `ui/`, `world/`; `useLiveReload` in `mcp/useMcpData.ts` und `planning/` (`PlanningPage`, `SprintsBacklog`, `GitHubBar`, `WorktreePills`).
- Abhängigkeiten: `react`, Browser-APIs (`localStorage`, `Intl`, `document.visibilityState`), `tools/webkit`; aus `src/` nur `api` (`useLiveReload`).
