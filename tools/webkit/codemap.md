# tools/webkit/

## Responsibility

Gemeinsame Oberflächen-Bausteine der beiden Werkzeug-Frontends (k3c-dev und Asset-Studio): Fehlertext, gemerkte Einstellungen, Debounce-Hook, Radix-Bausteine und Farb-Theme. Reiner Quellordner ohne eigenes Paket und ohne eigene Abhängigkeiten.

## Design

- `errors.ts`: `errorText` (Text einer Ablehnung: Fehlertext aus Go oder `Error` des Mocks).
- `prefs.ts`: `createPrefs(prefix)` bindet `loadPref` (Whitelist), `loadText`, `savePref`, `loadJSON` (mit Prüffunktion), `saveJSON` an ein Präfix in `localStorage`, jeder Zugriff try/catch-gesichert; `isStringList` als Prüfung. Jede App bindet ihr Präfix in ihrem `src/lib/prefs.ts` (`k3c-dev.`, `k3c-studio.`).
- `useDebounced.ts`: Wert zieht nach `ms` Ruhe nach (Filterfelder, 300 ms).
- `parts.tsx`: Wrapper über Radix Themes: `Tone`, `StatusBadge`, `NoticeCard`, `ActionButton` (Ladezustand, sperrt Doppelklicks), `Tip` (Tooltip).
- `theme.css`: Farb-Namen der Oberfläche auf Radix-Variablen (hell/dunkel) samt ANSI-Farben der Konsole.

## Flow

1. Ein Frontend importiert über den Alias `@webkit/<datei>` (`paths.ts` → `vite.config.ts`, `vitest.config.ts`; `tsconfig.json` › `paths`).
2. `resolve.dedupe` (`react`, `react-dom`, `@radix-ui/themes`) bzw. die `paths`-Einträge im `tsconfig.json` lösen die Pakete dieser Dateien aus dem `node_modules` des jeweiligen Frontends auf.

## Integration

- Konsumenten: `tools/k3c-dev/frontend/src/**`, `tools/k3c-studio/frontend/src/**`; geprüft von deren Typecheck (`include` mit `../../webkit`) und Vitest (`task check:dev`, `task check:studio`); Build-Quellen in `dev:frontend` und `studio:frontend`.
- Abhängigkeiten: `react`, `@radix-ui/themes` (aus dem Frontend), Browser-`localStorage`.
