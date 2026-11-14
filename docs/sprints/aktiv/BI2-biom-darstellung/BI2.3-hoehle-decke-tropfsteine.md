# BI2.3 · Höhle: Deckenhöhe und Tropfsteine

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/bi2
- **Abhängig von:** –
- **Tickets:** B-902, B-903
- **Kriterien:** AC-03, AC-04

## Ziel

Die Höhle zeigt Decke und Tropfsteine nach `ceilingHeight` und `stalactites` (B-902, B-903).

## Kontext

Werte in `World.Level.Nature.Traits`, Defaults in `World.Biome.Traits`. Zeichenliste in `props/plan.go` (Formen `shapeRect`, `shapeTri`; negatives `h` kippt das Dreieck nach unten). Sichtbare Höhe: `game.ViewUnits` = 420/32.

## Erlaubte Dateien

- `native/k3c-game/props/decor.go` (neu), `native/k3c-game/props/decor_test.go` (neu)
- `native/k3c-game/props/plan.go` (Aufruf in `plan`)

## Nicht-Ziele

Mine, Eisenstollen, Kristallhöhle (BI2.4).

## Schritte

1. Tests: Decke nur mit Eigenschaft, Unterkante steigt mit Wert; Tropfsteine 0 bei Wert 0, Anzahl steigt streng.
2. `decor.go`: Wert-Lesen, Decke, Tropfsteine; Aufruf vor allen anderen props.
3. `check:diff`, Commit.

## Fertig, wenn

B-902/AC-01, AC-02 und B-903/AC-01, AC-02 per Unit-Test erfüllt, `check:diff` grün.

## Prüfen

`check_run task:check:diff`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
