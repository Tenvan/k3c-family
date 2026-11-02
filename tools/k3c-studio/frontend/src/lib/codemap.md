# tools/k3c-studio/frontend/src/lib/

## Responsibility
Bindung der gemerkten Einstellungen des Studios an `localStorage` sowie Lizenz-Regeln und Größenangaben der Dateien. Fehlertext und Entprell-Hook liegen in `tools/webkit/` (`@webkit/errors`, `@webkit/useDebounced`).

## Design
- `prefs.ts`: ruft `createPrefs('k3c-studio.')` aus `@webkit/prefs` auf und exportiert `loadPref` (mit Whitelist), `loadText`, `savePref`, `loadJSON`, `saveJSON` und `isStringList`; Schlüssel `k3c-studio.<seite>.<was>`; jeder Zugriff ist try/catch-gesichert (gesperrte Website-Daten → Vorgabe, nichts gemerkt), die Logik steht in `tools/webkit/prefs.ts`. Freitext-Filter werden nicht gemerkt.
- `license.ts`: Lizenz-Marke einer Datei (`licenseMark`: frei, sa, prüfen, gesperrt; gleiche Regeln wie im Go-Backend), Beschriftung `MARK_LABEL` und `formatSize`. Test `license.test.ts`.
- Geteilt über `@webkit` (nicht hier): `tools/webkit/errors.ts` (`errorText`), `tools/webkit/useDebounced.ts` (Wert zieht verzögert nach), `tools/webkit/parts.tsx` (Bausteine), `tools/webkit/theme.css`.
- Test: `alias.test.ts` (Aliase `@game` und `@data` lösen auf).

## Flow
Eine Seite liest ihren Zustand beim Start mit `loadPref(...)` und schreibt Änderungen mit `savePref`; Textfelder laufen über `useDebounced` aus `@webkit/lib/useDebounced`, bevor eine Anfrage startet. Die Seite Assets merkt Ansicht und Teilung über `prefs`.

## Integration
- Konsumenten: `../App.tsx`, `../assetpage/` (`viewState`, `Inspector`, `SceneView`, `Belegung`, `AssetsPage`), `../chat/`, `../graphics/`, `../monarch/`, `../planner/`, `../sound/`, `../sources/`, `../world/` (`GameLauncher`). Die Seite Assets (`assetpage/`) nutzt `license.ts` für Marke und Größe.
- Abhängigkeiten: `@webkit/lib/prefs` (`tools/webkit/lib/prefs.ts`, geteilt mit k3c-workbench, das sein eigenes Präfix `k3c-workbench.` bindet) und Browser-APIs.
