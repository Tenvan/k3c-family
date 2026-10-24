# GEB18.5 · Protokoll meldet die Variante, der Client zeichnet sie und lädt nur Benötigtes

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb18/5-protokoll-client-varianten
- **Abhängig von:** GEB18.4
- **Tickets:** B-618
- **Kriterien:** AC-04, AC-06

## Ziel

Das Level meldet die Hintergrund-Varianten je Abschnitt (additives Feld, `docs/protocol.md`), der Client zeichnet sie und lädt nur die Varianten der gezeigten Stufen.

## Kontext

Protokoll `docs/protocol.md`, Server `engine/net`, Client `src/online/`, `src/model/types.ts`, Zeichnen `src/scenes/render/worldSprites.ts` und der Hintergrund-Aufbau in `src/scenes/render/`; Texturen in Dichte 128 mit Zoom 0,25 gegenüber heute.

## Erlaubte Dateien

- `engine/net/**` (nur das Feld), `docs/protocol.md`, Beispiel-Dateien des Protokolls
- `src/model/types.ts`, `src/online/**`, `src/scenes/render/**` und Tests
- Aufräumen: alte `public/grafik/gebaeude/hg-*-16.png`/`-32.png` ohne Variante, ihre Einträge in `public/grafik/gebaeude/index.json`, `data/assets.json`, Doku, Fixtures und Mocks

## Nicht-Ziele

Auswahl der Varianten (GEB18.4), Grafik.

## Schritte

1. Feld im Level additiv, Version bleibt, Beispiel-Datei.
2. Client: Variante je Abschnitt zeichnen, Rückfall Variante 1 bei fehlender Datei (Log 📄).
3. Laden nur für gezeigte Stufen; Speicher vorher/nachher über `sim_test` oder Messung im Browser festhalten.
4. Vitest der Abbildung und des Ladens.
5. Alte Schichten ohne Variante löschen, nachdem eine Suche belegt, dass nichts sie mehr nutzt (AC-06).

## Fertig, wenn

- [ ] AC-04: Vitest grün, `docs/protocol.md` aktuell, Messung im Ergebnis.
- [ ] AC-06: alte Schichten gelöscht, Suche ohne Treffer, Tests grün.

## Prüfen

Gezielte Tests, vor dem Commit `task check:diff`; Reviewer (Protokoll).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
