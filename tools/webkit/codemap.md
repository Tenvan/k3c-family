# tools/webkit/

## Responsibility

Gemeinsame Oberflächen-Bausteine aller Wails-Frontends (k3c-dev, Asset-Studio, Starter) als Atomic Design auf Radix Themes (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme). Reiner Quellordner ohne eigenes Paket und ohne eigene Abhängigkeiten.

## Design

- `lib/` (ohne Oberfläche): `errors.ts` (`errorText`), `prefs.ts` (`createPrefs(prefix)` mit `loadPref`, `loadText`, `savePref`, `loadJSON`, `saveJSON`, `isStringList`; jede App bindet ihr Präfix in `src/lib/prefs.ts`), `useDebounced.ts`, `split.ts` (`splitShare`, `clampSplit`, `parseSplit`, `useSplit(store, key, dflt)`).
- `atoms/` (`@webkit/atoms`): Re-Export der Radix-Grund-Atome (Layout, Text, Controls) und eigene Atome `ActionButton` (Ladezustand), `CardButton` (Karte als Knopf), `LinkButton` (Knopf als Link), `SelectField` (Radix Select aus Einträgen, leerer Wert erlaubt), `StatusBadge` (Badge mit Punkt), `Splitter`, `Tip`, `Tone`/`TONE_COLOR`. Nur hier stehen rohe HTML-Controls.
- `molecules/` (`@webkit/molecules`): Radix Card, Callout, DataList, Gruppen, Popover; `NoticeCard` (Callout), `CheckField`, `SwitchField`.
- `organisms/` (`@webkit/organisms`): Radix Dialog, AlertDialog, DropdownMenu, ContextMenu, Tabs, TabNav, Table; `AppHeader` (Titel, Reiter, rechte Leiste).
- `templates/` (`@webkit/templates`): Radix Theme, ThemePanel; `AppFrame` (Theme, Kopfzeile, optionale Reiter, Seite, `aside`, `overlay`, `fixed`).
- `testing/select.ts`: Test-Helfer `pickOption`, `selectedText`, `selectTrigger` für Radix Select in happy-dom; `testing/options.ts`: `shownOptions`, `optionTexts`, `pickWhere` (offene Liste lesen, Eintrag nach Bedingung wählen).
- `theme.css`: Farb-Namen auf Radix-Variablen (hell/dunkel), ANSI-Farben, Rahmen (`wk-shell`, `wk-header`, `wk-page`), Status-Punkt, Hinweis, Splitter.

## Flow

1. Ein Frontend importiert über den Alias `@webkit/<ebene>` bzw. `@webkit/lib/<datei>` (`paths.ts` → `vite.config.ts`, `vitest.config.ts`; `tsconfig.json` › `paths`).
2. `resolve.dedupe` (`react`, `react-dom`, `@radix-ui/themes`) bzw. die `paths`-Einträge im `tsconfig.json` lösen die Pakete dieser Dateien aus dem `node_modules` des jeweiligen Frontends auf.
3. Importe nur nach unten: lib ← atoms ← molecules ← organisms ← templates ← Seiten der App.

## Integration

- Konsumenten: `tools/k3c-dev/frontend/src/**`, `tools/k3c-studio/frontend/src/**`, `tools/k3c-starter/frontend/src/**`; geprüft von deren Typecheck (`include` mit `../../webkit`), Vitest (webkit-Tests laufen im Vitest von k3c-dev, `task check:dev`) und `tests/atomicDesign.test.ts` (Ebenen, Importrichtung, Radix nur über webkit, rohe Controls nur in Atomen).
- Abhängigkeiten: `react`, `@radix-ui/themes` (aus dem Frontend), Browser-`localStorage`.
