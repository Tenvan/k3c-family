# B-189 · Ein besiegter Gegner verschwindet nicht im Portal, sondern lässt sein Gold fallen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** LV1
- **Projekt:** –
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`stepEnemies` (`engine/sim/enemies.go`) setzt `Fleeing` für Gegner mit `fleesAtHalfHp`, sobald `HP < MaxHP/2`, auch bei `HP <= 0`. Am Ende von `stepEnemies` verschwinden flüchtende Gegner nahe ihrem Portal (`|X - HomeX| < 0.3`). Das läuft vor `removeDeadEnemies` (`engine/sim/waves.go`, `Step` in `world.go`). Ein Goblin, der nahe seinem Portal besiegt wird, verschwindet deshalb ohne Gold, ohne Rohstoff-Drop, ohne Aggressions-Zuwachs und ohne `kill`-Ereignis. Außerdem bewegt sich ein toter flüchtender Gegner noch einen Schritt. Gefunden in F3.1 (Test `TestEventsKillMitGold` musste `HomeX` weit weg legen).

## Ziel

Ein besiegter Gegner zählt immer als besiegt: Gold, Drop, Aggression und `kill` wie überall sonst.

## Beteiligte und Zielgruppen

Spieler (Gold für einen Kill geht verloren); Balancing (Kill-Zählung in B-099).

## Anforderungen

- Ein Gegner mit `HP <= 0` flieht nicht und verschwindet nicht im Portal; er wird in `removeDeadEnemies` behandelt.
- Deterministisch, mit 2 Spielern.

## Nicht-Ziele

Neue Flucht-Regeln, Werte in `data/enemies.json`.

## Regeln und Einschränkungen

Domäne SIM. Golden-Daten ändern sich nur, wenn ein Lauf den Fall enthält; dann `task golden:update` mit Begründung.

## Beispiele

Goblin mit `HomeX` 10 wird bei X 10,1 auf HP 0 geschlagen → im selben Tick Münzen bei X 10,1 und ein `kill`-Ereignis.

## Ausnahme- und Fehlerfälle

Lebender Goblin unter halber HP nahe dem Portal → verschwindet weiter mitsamt geklautem Gold (Regel bleibt).

## Akzeptanzkriterien

- **AC-01** Ein Go-Test: toter Goblin direkt am Portal → `kill`-Ereignis und gestreute Münzen; lebender flüchtender Goblin am Portal → verschwindet ohne `kill`. `task check:go` grün.

## Offene Fragen

keine

## Notizen

Entdeckt in F3.1 (2026-10-03).
