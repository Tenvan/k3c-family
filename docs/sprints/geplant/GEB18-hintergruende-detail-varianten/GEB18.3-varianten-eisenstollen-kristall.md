# GEB18.3 · Fünf detailreiche Varianten für Eisenstollen und Kristallhöhle, Studio-Szene Umgebung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb18/3-varianten-eisenstollen-kristall
- **Abhängig von:** GEB18.1
- **Tickets:** B-618
- **Kriterien:** AC-02

## Ziel

Eisenstollen und Kristallhöhle haben je fünf Varianten; alle Varianten aller fünf Stufen stehen als Slots im Katalog und in der Studio-Szene „Umgebung“.

## Kontext

Wie GEB18.2; Katalog `data/assets.json` (Slots `bg:<biom>:<n>`), Studio `tools/k3c-studio/frontend/src/graphics/Backgrounds.tsx` und `scenesLogic.ts` (`hintergrundStreifen`).

## Erlaubte Dateien

- `public/grafik/gebaeude/hg-<eisenstollen|kristall>-*`
- `data/assets.json` (Hintergrund-Slots je Variante)
- `tools/k3c-studio/frontend/src/graphics/**` (Szene „Umgebung“ mit Varianten, Tests)

## Nicht-Ziele

Client des Spiels, Level-Generator.

## Schritte

1. Varianten Eisenstollen und Kristall wie in GEB18.2.
2. Slots je Variante im Katalog, Test: jede Stufe hat fünf Varianten mit drei Schichten.
3. Studio-Szene „Umgebung“ zeigt je Biom alle Varianten, Vitest.

## Fertig, wenn

- [ ] AC-02: Katalog-Test und Studio-Vitest grün; alle 5 × 5 × 3 Schichten vorhanden.

## Prüfen

Gezielte Tests (`task:test` mit Muster `assets`, Studio-Vitest `graphics`), vor dem Commit `task check:diff`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
