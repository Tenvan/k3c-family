# tools/webkit/testing/

## Responsibility
Test-Helfer für Auswahllisten (`atoms/SelectField`, Radix Select) in happy-dom: Radix rendert kein `<select>`, die Einträge stehen erst nach dem Öffnen in einem Portal am `<body>`. Die Helfer öffnen, lesen und wählen Einträge so, wie ein Nutzer es täte. Nur für Tests, nie im Bundle einer App.

## Design
- `select.ts`:
  - `selectTrigger(root, label)`: Auslöser mit diesem `aria-label` (`role="combobox"`).
  - `pickOption(root, label, text)`: öffnet die Auswahl und klickt den Eintrag mit genau diesem Text.
  - `selectedText(root, label)`: angezeigter Text des gewählten Eintrags.
  - `selectedValue(root, label)` / `pickValue(root, label, value)`: arbeiten mit `data-value` an Auslöser und Einträgen (nur `SelectField`).
- `options.ts` (baut auf `select.ts` auf):
  - Typ `ShownOption` (Text, Wert, gesperrt).
  - `shownOptions(root, label)`: alle Einträge in Anzeige-Reihenfolge; `optionTexts` nur deren Texte.
  - `pickWhere(root, label, match)`: wählt den ersten Eintrag, dessen Text die Bedingung erfüllt (lange oder errechnete Beschriftungen).
- Aktionen laufen in `act` (React), damit Zustandswechsel vor der nächsten Prüfung durch sind.

## Flow
Test rendert Seite → `pickOption(container, 'Gruppe', 'Gebäude')` → Auslöser klicken → Portal-Eintrag suchen → klicken → `selectedText` bzw. `selectedValue` prüfen.

## Integration
- Konsumenten: Vitest-Tests von `tools/webkit/atoms` (`SelectField.test.tsx`) und Tests unter `tools/k3c-workbench/frontend/src/**` (Import über `@webkit/testing/<datei>`).
- Abhängigkeiten: `react` (`act`), happy-dom als Test-Umgebung (je Testdatei per `// @vitest-environment happy-dom`).
