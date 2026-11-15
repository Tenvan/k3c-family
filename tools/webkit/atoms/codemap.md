# tools/webkit/atoms/

## Responsibility
Atome (`@webkit/atoms`), die kleinsten Bausteine der Wails-Oberflächen im Atomic Design (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme): Re-Export der Radix-Grund-Atome und eigene Atome. Nur in diesem Ordner dürfen rohe HTML-Controls stehen.

## Design
- `index.ts`: Re-Export der Radix-Themes-Atome (Layout `Box`, `Flex`, `Grid`, `Container`, `Section`, `ScrollArea`; Text `Text`, `Heading`, `Code`, `Kbd`, `Link` …; Controls `Button`, `IconButton`, `Checkbox`, `Radio`, `Select`, `Slider`, `Switch`, `TextField`, `TextArea`, `SegmentedControl`, `Tooltip` …) und der eigenen Atome.
- Eigene Atome:
  - `ActionButton.tsx`: Knopf mit asynchroner Aktion; während sie läuft `loading` und gesperrt.
  - `CardButton.tsx`: Karte als Knopf; `LinkButton.tsx`: Knopf in Link-Optik.
  - `SelectField.tsx`: Radix Select aus Einträgen (`SelectOption`, `plainOptions`), leerer Wert erlaubt, Wert als `data-value` an Auslöser und Einträgen (für `testing/select.ts`).
  - `Slider.tsx`: `ValueSlider` (Radix Slider für genau einen Zahlenwert, `onChange(number)` innerhalb `min … max`).
  - `Splitter.tsx`: ziehbarer Trenner mit Tastatur (`SPLIT_KEY_STEP`, `splitKeyDelta`), Rechnung aus `lib/split.ts`.
  - `StatusBadge.tsx`: Badge mit Punkt in der Farbe des Tons.
  - `Tip.tsx`: Tooltip; ohne Inhalt nur das Kind.
  - `tone.ts`: Typ `Tone` und `TONE_COLOR` (Radix-Farbe je Ton).

## Flow
Seite oder höhere Ebene → `import { Flex, ActionButton, … } from '@webkit/atoms'` → Radix-Komponente oder eigenes Atom → Styles aus Radix Themes und `tools/webkit/theme.css`.

## Integration
- Darf nur `lib/` importieren (Importe nur nach unten); Konsumenten: `molecules/`, `organisms/`, `templates/` und `tools/k3c-workbench/frontend/src/**`.
- Abhängigkeiten: `react`, `@radix-ui/themes` (aus dem `node_modules` des Frontends).
- Regeln geprüft von `tests/atomicDesign.test.ts` (Radix nur über webkit, rohe Controls nur in Atomen).
- Tests: `SelectField.test.tsx`, `Slider.test.tsx`, `Splitter.test.tsx`.
