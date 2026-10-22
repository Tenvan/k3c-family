# tools/k3c-workbench/frontend/src/lib/

## Responsibility

Kleine, seitenunabhängige Hilfsfunktionen und React-Hooks von k3c-workbench: deutsche Formatierung, gemerkte Einstellungen (Präfix), Markdown-Parser und Zeit-Hook. Fehlertext, Debounce und Splitter-Rechnung kommen aus `tools/webkit` (`@webkit/lib/errors`, `@webkit/lib/useDebounced`, `@webkit/lib/split`).

## Design

- Pure Utilities: `format.ts` (`formatNumber`, `formatPercent`, `formatBytes`, `formatUptime`, `formatDuration`, `formatTime`, de-DE, deckungsgleich mit den Go-Formatern in `internal/mcpsrv`).
- Persistenz: `prefs.ts` bindet `createPrefs('k3c-workbench.')` aus `@webkit/lib/prefs` (`loadPref`, `loadText`, `savePref`, `loadJSON`, `saveJSON`, `isStringList`; Favoriten, Eingeklapptes) und `prefsStore` (`{ loadText, savePref }` für `useSplit`). Freitext-Filter werden nicht gemerkt.
- `markdown.ts` (+ Test): Markdown-Teilmenge -> Block-/Inline-Baum (`parseMarkdown`, `parseInline`), gezeichnet von `ui/molecules/MarkdownView`; lag früher in `mcp/`.
- Hooks: `useNow.ts` (Intervall-Timestamp).

## Flow

1. Eine Seite liest ihren Zustand initial mit `loadPref('key', ALLOWED, fallback)` bzw. `loadJSON('key', valid, fallback)` und schreibt bei Änderung mit `savePref`/`saveJSON`.
2. Textfelder werden mit `useDebounced` (`@webkit`) entprellt, bevor ein Backend-Query startet.
3. Anzeige-Komponenten formatieren Zahlen/Zeiten über `format.ts`; abgelehnte Promises werden mit `errorText` in `NoticeCard`s gezeigt.
4. Zeitabhängige Anzeigen (Uptime, Live-Fenster) ticken über `useNow(ms)`.

## Integration

- Konsumenten: `App.tsx`, `logs/`, `mcp/`, `planning/`, `services/`, `tasks/`, `ui/`.
- Abhängigkeiten: `react`, Browser-APIs (`localStorage`, `Intl`), `tools/webkit`; keine Importe aus anderen `src/`-Ordnern.
