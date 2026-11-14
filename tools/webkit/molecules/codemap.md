# tools/webkit/molecules/

## Responsibility
Moleküle (`@webkit/molecules`): kleine Gruppen aus Atomen mit genau einer Aufgabe – Hinweise, Felder mit Beschriftung, Chip-Leisten, Menüs, Listen-Filter, Abspieler, geteilte Bereiche und SVG-Diagramme. Radix Themes liefert dazu Karten, Callouts, Gruppen und Popover.

## Design
- `index.ts`: Re-Export der Radix-Moleküle (`Callout`, `Card`, `CheckboxCards`, `CheckboxGroup`, `DataList`, `HoverCard`, `Inset`, `Popover`, `RadioCards`, `RadioGroup`) und der eigenen Moleküle.
- Hinweise und Felder: `NoticeCard.tsx` (Callout mit Ton), `ToggleField.tsx` (`CheckField`, `SwitchField`: Kontrollkästchen bzw. Schalter mit klickbarer Beschriftung), `LoadingNote.tsx` (Lade-Anzeige als Zeile oder Überlage, Kennung `data-loading`).
- Auswahl: `ChipSwitch.tsx` (Umschalter aus Chips, `aria-pressed`, erneuter Klick meldet den Wert erneut), `FocusBar.tsx` (B-677: gespeicherte Filter als Chips mit Zähler, eigene mit ✕, Knopf „Ansicht merken“), `ActionMenu.tsx` (Knopf mit Aktionsliste im Popover, `MenuAction`).
- Listen: `ListFilter.tsx` (B-599: Suchfeld, Chips je Dimension, Sortierung, Gruppierung, Zähler „n von m“, Zurücksetzen; Logik aus `lib/listFilter.ts`, Merken über `{loadJSON, saveJSON}` aus `createPrefs`).
- Layout und Abspielen: `SplitStack.tsx` (B-875: zwei Bereiche übereinander mit waagerechtem Splitter, Anteil 20–80 % gemerkt), `FramePlayer.tsx` (WB2: Abspielen, Frame-Schieber und Zähler mit festen Breiten, `counterWidth`).
- Diagramme als reines SVG: `LineChart.tsx` (mehrere Serien), `BarChart.tsx` (Balken mit Ton), `CorridorBar.tsx` (Wert gegen Zielkorridor, grün/rot), `ChartBandShape.tsx` (grünes Korridorband), `chartTheme.ts` (`SERIES_COLORS`, `TONE_FILL`, `GRID`, `AXIS_TEXT`, `BAND_FILL`, `BAND_LINE` nur über Radix-CSS-Variablen; Typen `ChartBand`, `ChartTone`); Skalen aus `lib/chartScale.ts`.

## Flow
- Listen-Seite → `<ListFilter items itemKey renderItem|renderList storageKey prefs>` → `buildView` → Gruppen (einklappbar) mit `renderItem` je Eintrag bzw. `renderList` je Gruppe → Auswahl über `saveJSON` gemerkt.
- Diagramm → `extent`/`niceExtent` → `scaleLinear` → `linePath` bzw. Balken-Rechtecke → `ChartBandShape` für den Korridor.

## Integration
- Importiert nur `../atoms` und `../lib`; Konsumenten: `organisms/` (z. B. `FacetRail`, `TicketDialog`) und die Seiten unter `tools/k3c-workbench/frontend/src/**`.
- Abhängigkeiten: `react`, `@radix-ui/themes`.
- Tests: `ChipSwitch`, `FramePlayer`, `ListFilter`, `LoadingNote`, `SplitStack` (`*.test.tsx`), `charts.test.tsx`.
