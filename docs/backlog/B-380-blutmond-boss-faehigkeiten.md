# B-380 · Der Blutmond verstärkt auch die Flächenangriffe der Bosse

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Blutmond multipliziert den Schaden nur in `attack` (`engine/sim/enemies.go` über `moonDamage`). Flächenangriff des Bosses (`engine/sim/boss.go`, `applyDamageBy` mit `e.Damage`) und Splitter-Geschosse (`engine/sim/boss_abilities.go`) bleiben ohne Faktor (Review K3.3).

## Ziel

Im Blutmond trifft jeder Gegnerschaden mit dem Faktor aus `data/events.json`.

## Beteiligte und Zielgruppen

Spieler; REG für Werte.

## Anforderungen

- Boss-Flächenangriff und Splitter nutzen `moonDamage`; deterministisch.

## Nicht-Ziele

Werte (BR2).

## Regeln und Einschränkungen

`docs/rules/bosse.md` § 2; Golden-Läufe ohne Insel unverändert.

## Beispiele

Boss-Splitter mit Schaden 20 in Nacht 13 → 26.

## Ausnahme- und Fehlerfälle

Nacht 14 → wieder 20.

## Akzeptanzkriterien

- **AC-01** Test: Boss-Flächenangriff und Splitter treffen im Blutmond mit dem Faktor, sonst unverändert.

## Offene Fragen

Soll der Blutmond Bosse überhaupt verstärken? (🧑)

## Notizen

Gefunden im Review K3.3.
