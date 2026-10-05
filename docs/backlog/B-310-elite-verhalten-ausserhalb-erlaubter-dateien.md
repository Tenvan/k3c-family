# B-310 · Elite-Werte wirken nur mit Änderungen außerhalb der Erlaubten Dateien von W4.3b

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W4.3b (Sprint W4) soll laut Kontext beim Elite-Upgrade HP, Schaden und Reichweite durch die Werte aus
`data/troops.json` ersetzen: Der Elite-Krieger nutzt das Krieger-Verhalten aus W4.3a, der Elite-Bogenschütze
`stepArcher`. Im Code geht das an zwei Stellen nicht, und beide liegen außerhalb der Erlaubten Dateien von W4.3b:

1. `engine/sim/archer.go:70`: `stepArcher` liest fest `troops["archer"]` (Schaden, Reichweite, Angriffe je Sekunde).
   Ein Elite-Bogenschütze schösse mit den Werten des normalen Bogenschützen. `archer.go` steht nicht in den Erlaubten
   Dateien.
2. `engine/sim/units.go` › `stepTroops`: Nur `warrior` geht in `stepWarrior`, jede andere Kämpfer-Art in
   `stepArcher`. Ein Elite-Krieger würde schießen statt im Nahkampf zu stehen. W4.3b darf in `units.go` nur
   Beförderung und `makeArcher` ändern.

Beides ist je eine Zeile: `a := troops[t.Kind]` in `stepArcher` und `case "warrior", "eliteWarrior":` in `stepTroops`.

## Ziel

W4.3b kann AC-04 und AC-06 (Elite laut Daten) mit grünem `task check:go` erfüllen.

## Beteiligte und Zielgruppen

Entwickler (SIM); 🧑 entscheidet über die Erweiterung der Erlaubten Dateien.

## Anforderungen

- Elite-Bogenschütze schießt mit Schaden, Reichweite und Angriffstempo von `eliteArcher`.
- Elite-Krieger kämpft mit dem Krieger-Verhalten (Posten, Nahkampf) und den Werten von `eliteWarrior`.

## Nicht-Ziele

Weitere Änderungen an Bogenschützen oder Türmen; Balancing (BR1, B-099).

## Regeln und Einschränkungen

Session nur in ihren Erlaubten Dateien (`docs/arbeitsweise.md`); Domäne SIM bleibt gewahrt.

## Beispiele

Ein Elite-Bogenschütze trifft einen Gegner → Schaden 23 laut `troops.json`, nicht 15.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Freigabe-Frage, kein Laufzeitfall.

## Akzeptanzkriterien

- **AC-01** W4.3b darf `engine/sim/archer.go` (nur Werte je Art in `stepArcher`) und in `engine/sim/units.go` den
  Fall für `eliteWarrior` in `stepTroops` ändern, oder 🧑 legt einen anderen Weg fest; W4.3b wird danach fortgesetzt.

## Offene Fragen

Welcher Weg (entscheidet 🧑)? (a) Beide Zeilen in die Erlaubten Dateien von W4.3b aufnehmen (empfohlen, SIM, je eine
Zeile); (b) Elite nur über HP und Rüstung, Schaden und Reichweite folgen später (widerspricht W4-AC-04/AC-06).

## Notizen

Gefunden zu Beginn von W4.3b (Schritt 1, Code-Stand prüfen); umgesetzt ist in W4.3b noch nichts.
