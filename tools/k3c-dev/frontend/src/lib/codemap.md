# tools/k3c-dev/frontend/src/lib/

## Responsibility

Kleine, seitenunabhängige Hilfsfunktionen und React-Hooks: deutsche Formatierung, Fehlertext, gemerkte Einstellungen und Zeit-Hooks.

## Design

- Pure Utilities: `format.ts` (`formatNumber`, `formatPercent`, `formatBytes`, `formatUptime`, `formatDuration`, `formatTime`, de-DE, deckungsgleich mit den Go-Formatern in `internal/mcpsrv`), `errors.ts` (`errorText`).
- Persistenz-Wrapper: `prefs.ts` (`loadPref` mit Whitelist, `loadText`, `savePref`) auf `localStorage` mit Präfix `k3c-dev:`; jeder Zugriff ist try/catch-gesichert (gesperrte Website-Daten → Vorgabe).
- Hooks: `useDebounced.ts` (Wert zieht nach 300 ms nach), `useNow.ts` (Intervall-Timestamp).

## Flow

1. Eine Seite liest ihren Zustand initial mit `loadPref('key', ALLOWED, fallback)` und schreibt bei Änderung mit `savePref`.
2. Textfelder werden mit `useDebounced` entprellt, bevor ein Backend-Query startet.
3. Anzeige-Komponenten formatieren Zahlen/Zeiten über `format.ts`; abgelehnte Promises werden mit `errorText` in `NoticeCard`s gezeigt.
4. Zeitabhängige Anzeigen (Uptime, Live-Fenster) ticken über `useNow(ms)`.

## Integration

- Konsumenten: `App.tsx`, `logs/`, `mcp/`, `planning/`, `services/`, `tasks/`.
- Abhängigkeiten: nur `react` und Browser-APIs (`localStorage`, `Intl`); keine Importe aus anderen `src/`-Ordnern.
