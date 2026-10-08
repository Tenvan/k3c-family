# B-327 · Golden-Läufe decken Eisenstollen und Kristallhöhle ab

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** K1
- **Projekt:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Golden-Läufe (`engine/sim/sim_test.go`, `testdata/golden/sim-*.json`) rechnen Wald, Höhle und Mine. Für Eisenstollen (Tiefe 3) und Kristallhöhle (Tiefe 4) gibt es nur Level-Golden (`level-ironhold.json`, `level-crystal.json`), keinen Simulationslauf; die neuen Gegner und Pools aus K1.3 sind nur durch Unit-Tests abgesichert. K1.3 sollte das nicht selbst entscheiden.

## Ziel

Regeländerungen an den tiefen Stufen fallen im Golden-Diff auf.

## Beteiligte und Zielgruppen

Entwicklung (SIM); 🧑 entscheidet.

## Anforderungen

- Je tiefer Stufe ein Lauf mit vollem Aggressionspool (wie `sim-mine-welle`), deterministisch, 2 Spieler.

## Nicht-Ziele

Balancing der Werte (BR2), Bosse (K2).

## Regeln und Einschränkungen

Golden-Ablauf (B-137, `docs/arbeitsweise.md`), `rng.json` nie anfassen.

## Beispiele

`sim-ironhold-welle`: Welle aus `lavaSlime`, `ironBeetle`, `fireSpirit` an 3 Portalen.

## Ausnahme- und Fehlerfälle

nicht relevant: neue Testdaten.

## Akzeptanzkriterien

- **AC-01** `testdata/golden/sim-ironhold-*.json` und `sim-crystal-*.json` liegen vor, `task check:go` grün.

## Offene Fragen

Lohnen eigene Läufe, oder genügen die Unit-Tests aus K1.3? 🧑

## Notizen

–
