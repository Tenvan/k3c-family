# B-530 · Wartende Monarchen nach Verbindungsverlust sind in der Simulation nicht unverwundbar

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BD7
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-10-22 (mit BD7)

## Ausgangslage

`docs/rules/bedienung.md` § 3 (Q04): Ein getrennter Monarch ist während der Frist unverwundbar und ausgeblendet. Im Go-Code setzt der Raum nur `sim.Player.Free`; `damagePlayer` (`engine/sim/common.go`) und die Zielwahl `candidates` (`engine/sim/enemies.go`) prüfen `Free` nicht, nur Boss-Fähigkeiten (`boss_abilities.go`). Aufgefallen in BD3.

## Ziel

Wartende (und freie) Monarchen werden nicht getroffen und nicht als Ziel gewählt, wie die Regel sagt.

## Beteiligte und Zielgruppen

Spielende online.

## Anforderungen

- Die Simulation behandelt einen freien Monarchen (`sim.Player.Free`; der Raum setzt es für `waiting` und `free`, `engine/room/room.go` › `syncFree`) wie einen geschützten (`Protected`, BD3): `damagePlayer` (`engine/sim/common.go`) zieht keine HP ab, `candidates` (`engine/sim/enemies.go`) und `aoeTargets` (`engine/sim/enemies_traits.go`) wählen ihn nicht.
- Umsetzung in SIM (eine Hilfsfunktion für „geschützt“), der Raum bleibt unverändert. Default gewählt (autonom): Die Regel § 3 nennt `waiting` und `free` gleich („bis dahin bleibt er ausgeblendet und unverwundbar“), `Free` deckt beide ab.
- Deterministisch; Golden unverändert (offline ist `Free` immer false).

## Nicht-Ziele

Ausblenden im Client.

## Regeln und Einschränkungen

`docs/rules/bedienung.md` § 3.

## Beispiele

Gerät verliert nachts die Verbindung, Gegner laufen über seinen Monarchen: keine HP-Verluste, kein Gegner wählt ihn als Ziel. Ein zweiter, besetzter Monarch daneben wird weiter getroffen.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Go-Test: Ein wartender Monarch verliert während der Frist keine HP.
- **AC-02** Go-Test: Ein freier Monarch ist kein Ziel der Gegner und nimmt keinen Flächenschaden; besetzte Monarchen unverändert.

## Offene Fragen

keine (vermutet, ungeprüft im laufenden Spiel).

## Notizen

Angelegt in BD3.1.
