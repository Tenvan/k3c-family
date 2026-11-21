# tools/webkit/lib/

## Responsibility
Unterste Ebene von `@webkit` ohne eigene Oberfläche: reine Funktionen, Hooks und Kontexte, die Atome, Moleküle, Organismen und die Seiten der Wails-Frontends teilen (Fehlertext, gemerkte Einstellungen, Splitter-Rechnung, Listen-Filter, Welt-Filter, Werteingabe, Seitenwechsel, Diagramm-Skalen, Ticket-Baustein) sowie das Vite-Plugin für den Dev-Proxy.

## Design
- `errors.ts`: `errorText(e)` (Wails lehnt mit dem Go-Fehlertext als String ab, der Mock mit einem `Error`).
- `prefs.ts`: `createPrefs(prefix)` → `loadPref`, `loadText`, `savePref`, `loadJSON`, `saveJSON` auf `localStorage` mit Schlüssel `<app>.<seite>.<was>`, Zugriffsfehler werden geschluckt; `isStringList`.
- `useDebounced.ts`: `useDebounced(value, ms)` zieht den Wert erst nach Ruhe nach.
- `split.ts`: `splitShare`, `clampSplit` (20–80 %), `parseSplit`, Typ `SplitStore`, Hook `useSplit(store, key, dflt)` für `atoms/Splitter` und `molecules/SplitStack`.
- `listFilter.ts` (B-599, ohne React): Typen `FilterDimension`, `SortKey`, `ListFilterConfig`, `ListFilterState`, `ChipOption`, `ItemGroup`; `emptyState`, `valuesOf`, `filterItems`, `chipOptions`, `sortItems`, `groupItems`, `buildView` (alles in einem Schritt), `toggleChip`, `isListFilterState` – Grundlage von `molecules/ListFilter`.
- `weltFilter.ts` (B-1169, WZ62.2, ohne React): Welt-Filter für Asset-Listen; Typen `WeltModus` (`gesamt`/`exklusiv`, `WELT_MODI`), `WeltFilterState` (`welt`, `modus`, `gruppe`, `art`, `geaendert`), `WELT_VORGABE`, `WeltSichtLike`, `WeltZuordnung`; `zuordnung(sicht, modus, key)` (Sichtbarkeit, Herkunftswelt, exklusiv, geerbt), `parseWeltHash` und `formatWeltHash` (Zustand in der Adresse, `#welt=…&modus=…`, fremde Parameter bleiben, Vorgaben entfallen).
- `useWeltFilter.ts`: Hook `useWeltFilter()` → `[state, change]`; liest `location.hash` beim Start, schreibt jede Änderung per `history.replaceState` (ohne neuen Verlaufseintrag) zurück und folgt `hashchange`; bei gesperrtem Zugriff gilt der Zustand nur im Speicher.
- `navigate.ts` (WZ62.4, B-1171): `navigateTo(page)` sendet das Fenster-Ereignis `k3c:navigate` (`NAVIGATE_EVENT`), `useNavigateRequests(handler)` hört darauf; so wechselt eine Seite zu einer anderen, ohne die Navigation zu kennen.
- `valueEdit.ts` (B-1173, WZ62.7, ohne React): Werteingabe in Tabellen; `valueKind` (Zahl, Ja/Nein, Text, JSON nach dem Basiswert), `valueText`, `parseEntry` (Prüfung beim Tippen, Komma als Dezimaltrenner, `ja`/`nein`, JSON gleichen Typs), `stepSize`/`stepText` (Pfeiltasten, Umschalt ×10, gerundet), `deviation` (Abweichung zum Standardwert mit Prozent); die endgültige Prüfung bleibt im Backend.
- `chartScale.ts` (ohne Abhängigkeiten): `extent`, `niceTicks`, `niceExtent`, `scaleLinear`, `formatTick`, `linePath` für `LineChart`, `BarChart`, `CorridorBar`.
- `ticket.ts` (B-595): Listen `TICKET_TYPES`, `TICKET_PRIOS`, `TICKET_DOMAINS` und Typen `TicketInput`, `TicketCreated`, `TicketContextInfo`, `TicketPageContext` (JSON-Vertrag zum Go-Paket `tools/assetkit/ticket`).
- `ticketContext.tsx`: `TicketProvider` (App, Version, Vorgaben, `create`/`recent` als Backend-Aufrufe, sichtbare Seite und Auswahl), `useTicket`, `useTicketOptional` (ohne Provider `null`), `useTicketPage(page, selection, active)`.
- `devProxy.ts`: Vite-Plugin `waitForBackend` (hält `/api` im Dev-Server zurück, bis das Backend lauscht; `portOpen`); nur in `vite.config.ts` (Node), mit Node-Typen per Referenz.

## Flow
- Einstellungen: `tools/k3c-workbench/frontend/src/lib/prefs.ts` bindet `createPrefs('k3c-workbench.')` → Seiten nutzen → `loadJSON`/`saveJSON`.
- Tickets: `App.tsx` → `TicketProvider(create, recent)` → Seiten melden sich per `useTicketPage` → `organisms/TicketDialog` liest `useTicket` → `create(input)` → Go-Binding `TicketCreate`.
- Listen: `ListFilter` → `buildView` (`filterItems` → `sortItems` → `groupItems`).
- Welten: `useWeltFilter` ↔ Adresse (`weltFilter.ts`) → `molecules/WeltFilter`; Seiten fragen `zuordnung` je Asset; `navigateTo('<reiter>')` → `useNavigateRequests` der App.
- Werte: `molecules/ValueField` → `parseEntry`/`stepText` → `onCommit(value)`; `deviation` zeigt die Abweichung.

## Integration
- Konsumenten: `tools/webkit/atoms`, `molecules`, `organisms`; `tools/k3c-workbench/frontend/src/**` über `@webkit/lib/<datei>`; `devProxy.ts` von `tools/k3c-workbench/frontend/vite.config.ts`.
- Abhängigkeiten: `react` (Hooks, Kontext), Browser-`localStorage`, `vite` und `node:http`/`node:net` nur in `devProxy.ts`.
- Tests: `chartScale.test.ts`, `devProxy.test.ts`, `listFilter.test.ts`, `ticketContext.test.tsx`, `valueEdit.test.ts`, `weltFilter.test.ts`.
