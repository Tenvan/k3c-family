# tools/k3c-workbench/frontend/src/balancing/editor/

## Responsibility

Lauf-Editor der Balancing-Seite (B-661): Ausgangspunkt wählen, Szenario und Spieler festlegen, Varianten als Tabelle
(fest oder Bereich zum Abstimmen) gegen die Werte aus `data/` prüfen, Dauer schätzen und den Balancing-Auftrag starten.

## Design

- `RunEditor.tsx`: Wurzel; `OriginPicker` oben, Abschnitte aus `Sections.tsx`, `VariantTable` mit `DataBrowser`
  daneben, feste `EditorFooter`. Braucht eine feste Höhe vom Elternelement; nur der Inhalt scrollt.
- `OriginPicker.tsx`: Ausgangspunkt Leer, Beispiel (`EXAMPLES` aus `../help.ts`), Basis-Lauf, Voreinstellung (löschbar
  über `dropPreset`) oder Bericht; die Wahl übernimmt alle Felder.
- `Sections.tsx`: `Card`, `ScenarioSection` (Welt, Tiefe, Dev-Hilfen, Grad, Tage), `PlayersSection` (Spieler 1–4,
  Bot-Profil gemeinsam oder je Spieler), Zusammenfassungen wie `scenarioSummary`.
- `VariantTable.tsx`: Zeilen mit Basis, Neu, Abweichung, Art fest/Bereich; „Als Text bearbeiten“ im Zeilenformat
  `datei.json:pfad=wert` bzw. `=min..max`; ab einem Bereich `TuneBox` (Modus Suche oder Sweep mit Budget bzw.
  Schritten); Option „je Variante“.
- `DataBrowser.tsx`: alle Zahlenwerte der variierbaren Dateien, Suche (`searchValues`, jedes Wort in Datei oder Pfad,
  höchstens `MAX_HITS = 200`), je Wert „+ fest“ und „+ Bereich“; Beschreibung der Datei über `backend.balanceDataDoc`.
- `Choice.tsx`: Segmented Control statt Auswahlliste für höchstens fünf Werte (leerer Wert unter Ersatzschlüssel).
- Reine Logik:
  - `variants.ts`: `VariantRow`, `parseLine`, `lineOf`, Text ↔ Zeilen; Unlesbares bleibt als `raw`.
  - `variantCheck.ts`: Prüfung der Zeilen wie in Go (`tools/k3c-workbench/internal/simrun/spec.go`), Datei und Pfad
    gegen den `ValueIndex` (`buildIndex`, `keyOf`), Vorschläge per Levenshtein (`distance`), Grenzen `MAX_RANGES = 2`,
    `MAX_FEST = 20`. Fehler sperren das Starten, Hinweise nicht.
  - `variantForm.ts`: Zeilen ↔ `BalanceForm` (feste Zeilen in `vary`, Bereiche in `tune.params`), `canPerVariant`;
    den Auftrag baut weiter `specFromForm` aus `../spec.ts`.
  - `estimate.ts`: Umfang (`workOf`), Seeds (`ALL_SEEDS = 100`), Minuten je Seed aus früheren Läufen gleicher Welt und
    Tage (Median), sonst Faustregel aus `../help.ts`.
- Hooks: `useEditorState.ts` (Felder, Zeilen, „je Variante“), `useDataValues.ts` (lädt einmal
  `backend.balanceDataValuesAll`).

## Flow

1. `BalancingPage` öffnet `RunEditor initial origin baselines runs presets`.
2. `OriginPicker.pick` -> `exampleForm`/`baselineForm` (`variantForm.ts`, über `formFromExample`/`formFromBaseline`) oder
   Voreinstellung -> `onLoad` -> `useEditorState` -> `rowsFromForm`.
3. Tabelle/Browser ändern Zeilen -> `variantCheck` gegen `useDataValues` -> Fehler und Hinweise je Zeile.
4. `EditorFooter` zeigt Zusammenfassung und Schätzung (`estimate.ts`); „Starten“ -> `formWithRows` -> `specFromForm`
   -> `onStart(spec)`; „Voreinstellung speichern“ -> `savePreset` (`../spec.ts`).

## Integration

- Konsument: `../BalancingPage.tsx`.
- Abhängigkeiten: `../spec.ts`, `../tune.ts`, `../help.ts`, `../HelpButton.tsx`, Typen aus `src/api`
  (`BalanceForm`-Umfeld, `BalanceDataFileValue`, `TuneMode`), `@webkit/atoms|molecules`.
- Stile: `../../styles/balancing-editor.css`.
