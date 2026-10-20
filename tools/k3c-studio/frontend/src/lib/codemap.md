# tools/k3c-studio/frontend/src/lib/

## Responsibility
Bindung der gemerkten Einstellungen des Studios an `localStorage`; Fehlertext und Entprell-Hook liegen jetzt in `tools/webkit/` (`@webkit/errors`, `@webkit/useDebounced`).

## Design
- `prefs.ts`: ruft `createPrefs('k3c-studio.')` aus `@webkit/prefs` auf und exportiert `loadPref` (mit Whitelist), `loadText`, `savePref`, `loadJSON`, `saveJSON` und `isStringList`; Schlüssel `k3c-studio.<seite>.<was>`; jeder Zugriff ist try/catch-gesichert (gesperrte Website-Daten → Vorgabe, nichts gemerkt), die Logik steht in `tools/webkit/prefs.ts`. Freitext-Filter werden nicht gemerkt.
- Geteilt über `@webkit` (nicht hier): `tools/webkit/errors.ts` (`errorText`), `tools/webkit/useDebounced.ts` (Wert zieht verzögert nach), `tools/webkit/parts.tsx` (Bausteine), `tools/webkit/theme.css`.
- Test: `alias.test.ts` (Aliase `@game` und `@data` lösen auf).

## Flow
Eine Seite liest ihren Zustand beim Start mit `loadPref(...)` und schreibt Änderungen mit `savePref`; Textfelder laufen über `useDebounced` aus `@webkit/lib/useDebounced`, bevor eine Anfrage startet.

## Integration
- Konsumenten: `../App.tsx`, `../assets/`, `../graphics/`, `../sound/`, `../world/` (`GameLauncher`); `assets/` gibt `loadText`/`savePref` als Ablage an `useSplit` (`@webkit/lib/split`).
- Abhängigkeiten: `@webkit/lib/prefs` (`tools/webkit/lib/prefs.ts`, geteilt mit k3c-dev, das sein eigenes Präfix `k3c-dev.` bindet) und Browser-APIs.
