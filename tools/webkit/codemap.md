# tools/webkit/

## Responsibility

Gemeinsame Oberflächen-Bausteine der Tauri-Frontends (k3c-workbench, k3c-studio) als Atomic Design auf Radix Themes (`docs/arbeitsweise.md` › Oberflächen der Tauri-Programme). Reiner Quellordner ohne eigenes Paket und ohne eigene Abhängigkeiten; direkt im Ordner liegt nur `theme.css`.

## Design

| Ordner | Rolle |
|--------|-------|
| [`lib/`](lib/codemap.md) | ohne Oberfläche: `errorText`, `createPrefs`, `useDebounced`, Splitter-Rechnung, Listen-Filter, Diagramm-Skalen, Ticket-Typen und `TicketProvider`, Vite-Plugin `waitForBackend` |
| [`atoms/`](atoms/codemap.md) | `@webkit/atoms`: Radix-Grund-Atome und eigene Atome (`ActionButton`, `CardButton`, `LinkButton`, `SelectField`, `ValueSlider`, `Splitter`, `StatusBadge`, `Tip`, `Tone`); nur hier rohe HTML-Controls |
| [`molecules/`](molecules/codemap.md) | `@webkit/molecules`: Radix Card, Callout, Popover …; `NoticeCard`, `CheckField`/`SwitchField`, `ChipSwitch`, `FocusBar`, `ActionMenu`, `ListFilter`, `SplitStack`, `FramePlayer`, `LoadingNote`, SVG-Diagramme `LineChart`, `BarChart`, `CorridorBar` |
| [`organisms/`](organisms/codemap.md) | `@webkit/organisms`: Radix Dialog, Menüs, Tabs, Table; `AppHeader`, `FacetRail`, `TransportBar`, `TicketButton`, `TicketDialog` |
| [`templates/`](templates/codemap.md) | `@webkit/templates`: Radix Theme, ThemePanel; `AppFrame`, `ListDetail` |
| [`testing/`](testing/codemap.md) | Test-Helfer für Radix Select in happy-dom (`pickOption`, `pickValue`, `shownOptions` …) |

- `theme.css`: Farb-Namen auf Radix-Variablen (hell/dunkel), ANSI-Farben, Rahmen (`wk-shell`, `wk-header`, `wk-page`, `wk-listdetail`), Status-Punkt, Hinweis, Splitter.

## Flow

1. Ein Frontend importiert über den Alias `@webkit/<ebene>` bzw. `@webkit/lib/<datei>` (`tools/k3c-workbench/frontend/paths.ts` → `vite.config.ts`, `vitest.config.ts`; `tsconfig.json` › `paths`).
2. `resolve.dedupe` (`react`, `react-dom`, `@radix-ui/themes`) bzw. die `paths`-Einträge im `tsconfig.json` lösen die Pakete dieser Dateien aus dem `node_modules` des jeweiligen Frontends auf.
3. Importe nur nach unten: lib ← atoms ← molecules ← organisms ← templates ← Seiten der App.

## Integration

- Konsumenten: `tools/k3c-workbench/frontend/src/**`; geprüft von deren Typecheck (`include` mit `../../webkit`), Vitest (webkit-Tests laufen im Vitest von k3c-workbench, `task check:dev`) und `tests/atomicDesign.test.ts` (Ebenen, Importrichtung, Radix nur über webkit, rohe Controls nur in Atomen).
- Abhängigkeiten: `react`, `@radix-ui/themes` (aus dem Frontend), Browser-`localStorage`.
