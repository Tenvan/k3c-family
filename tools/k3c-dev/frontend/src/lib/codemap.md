# tools/k3c-dev/frontend/src/lib/

## Responsibility

Kleine, seitenunabhängige Hilfsfunktionen und React-Hooks von k3c-dev: deutsche Formatierung, gemerkte Einstellungen (Präfix) und Zeit-Hook. Fehlertext und Debounce kommen aus `tools/webkit` (`@webkit/errors`, `@webkit/useDebounced`).

## Design

- Pure Utilities: `format.ts` (`formatNumber`, `formatPercent`, `formatBytes`, `formatUptime`, `formatDuration`, `formatTime`, de-DE, deckungsgleich mit den Go-Formatern in `internal/mcpsrv`).
- Persistenz: `prefs.ts` bindet `createPrefs('k3c-dev.')` aus `@webkit/prefs` (`loadPref`, `loadText`, `savePref`, `loadJSON`, `saveJSON`, `isStringList`; Favoriten, Eingeklapptes, Splitter). Freitext-Filter werden nicht gemerkt.
- Hooks: `useNow.ts` (Intervall-Timestamp).

## Flow

1. Eine Seite liest ihren Zustand initial mit `loadPref('key', ALLOWED, fallback)` bzw. `loadJSON('key', valid, fallback)` und schreibt bei Änderung mit `savePref`/`saveJSON`.
2. Textfelder werden mit `useDebounced` (`@webkit`) entprellt, bevor ein Backend-Query startet.
3. Anzeige-Komponenten formatieren Zahlen/Zeiten über `format.ts`; abgelehnte Promises werden mit `errorText` in `NoticeCard`s gezeigt.
4. Zeitabhängige Anzeigen (Uptime, Live-Fenster) ticken über `useNow(ms)`.

## Integration

- Konsumenten: `App.tsx`, `logs/`, `mcp/`, `planning/`, `services/`, `tasks/`.
- Abhängigkeiten: `react`, Browser-APIs (`localStorage`, `Intl`), `tools/webkit`; keine Importe aus anderen `src/`-Ordnern.
