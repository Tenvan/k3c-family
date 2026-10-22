# GEB11.3 · Grafik der Wachstumsreihe und Vorschau im Studio

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb11/3-grafik-wachstumsreihe
- **Abhängig von:** GEB11.1
- **Tickets:** B-458
- **Kriterien:** AC-03

## Ziel

Lagerfeuer, Zelt und Hütte (`lagerfeuer.json`, Stufen 1–3) und das Baugerüst (`hub-geruest.json`) liegen als Daten-JSON und PNG vor; das Studio zeigt die ganze Wachstumsreihe vom Lagerfeuer bis zur Kristallburg.

## Kontext

- Regeldatei `docs/standards/gebaeude-grafik.md` § 5 (Datenformat, `vorlage` Pflicht), § 6 (Stufen als Zusätze, nicht kumulativ), § 7 (Dichte, `fein`). Nur DB32.
- Reihenfolge und Bedingungen: `docs/rules/materialien-gebaeude.md` § 2.1 (GEB11.1). Die Burg 1–5 (`burg.json`, GEB4) bleibt unverändert.
- Die Reihe steht einmal im Spielcode (`src/scenes/render/hubGrowth.ts`, ohne Phaser) und das Studio liest sie über `@game`.
- Studio: `tools/k3c-studio/frontend/src/graphics/scenesLogic.ts`, `BuildingStrip.tsx`, Test `scenesLogic.test.ts` (liest den echten `index.json`).

## Erlaubte Dateien

- `public/grafik/gebaeude/lagerfeuer*.{json,png}`, `hub-geruest*.{json,png}`, `index.json`
- `src/scenes/render/hubGrowth.ts` (neu, reine Reihe) mit Test
- `tools/k3c-studio/frontend/src/graphics/` (Vorschau, Test, codemap)
- `docs/standards/gebaeude-grafik.md` (§ 1 Tabelle, § 4 Arten, § 6 Abschnitt Wachstum)

## Nicht-Ziele

Spiel-Zuordnung (GEB11.4), `burg.json`, `engine/`, `data/`.

## Schritte

1. Vorlage schreiben, Bausteine zeichnen (Feuer mit Grill und Holzstapel als Grundform, Zelt als Stufe 2, Hütte als Stufe 3; Gerüst 8 × 9 transparent).
2. `task gen:gebaeude` zweimal (gleiche Bytes), PNGs ansehen.
3. Reihe `HUB_GROWTH` mit Test; Studio-Streifen „Hub wächst (GEB11)“ mit Test.
4. Regeldatei nachziehen.

## Fertig, wenn

- [ ] AC-03: JSON und PNG (16/32/64/128) je Look, `go test ./tools/buildings` grün, Studio-Test prüft die Reihe gegen den echten `index.json`.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
