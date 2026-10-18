# tools/k3c-studio/frontend/src/lib/

## Responsibility
Kleine, seitenunabhängige Hilfen: Fehlertext, gemerkte Einstellungen in `localStorage` und ein Entprell-Hook.

## Design
- `errors.ts`: `errorText` macht aus einem abgelehnten Promise einen Anzeigetext.
- `prefs.ts`: `loadPref` (mit Whitelist), `loadText`, `savePref`, `loadJSON`, `saveJSON`, `isStringList`; Schlüssel `k3c-studio.<seite>.<was>`; jeder Zugriff ist try/catch-gesichert (gesperrte Website-Daten → Vorgabe, nichts gemerkt). Freitext-Filter werden nicht gemerkt.
- `useDebounced.ts`: Wert zieht verzögert nach.
- Test: `alias.test.ts` (Aliase `@game` und `@data` lösen auf).

## Flow
Eine Seite liest ihren Zustand beim Start mit `loadPref(...)` und schreibt Änderungen mit `savePref`; Textfelder laufen über `useDebounced`, bevor eine Anfrage startet.

## Integration
- Konsumenten: `../App.tsx`, `../world/`, `../planer/`, `../stage/`, `../assets/`.
- Abhängigkeiten: nur `react` und Browser-APIs; Kopie aus dem Schwester-Tool k3c-dev (Präfix `k3c-studio.`).
