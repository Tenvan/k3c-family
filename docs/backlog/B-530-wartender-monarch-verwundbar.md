# B-530 · Wartende Monarchen nach Verbindungsverlust sind in der Simulation nicht unverwundbar

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/rules/bedienung.md` § 3 (Q04): Ein getrennter Monarch ist während der Frist unverwundbar und ausgeblendet. Im Go-Code setzt der Raum nur `sim.Player.Free`; `damagePlayer` (`engine/sim/common.go`) und die Zielwahl `candidates` (`engine/sim/enemies.go`) prüfen `Free` nicht, nur Boss-Fähigkeiten (`boss_abilities.go`). Aufgefallen in BD3.

## Ziel

Wartende (und freie) Monarchen werden nicht getroffen und nicht als Ziel gewählt, wie die Regel sagt.

## Beteiligte und Zielgruppen

Spielende online.

## Anforderungen

Raum setzt für wartende Monarchen den Schutz aus BD3 (`sim.Player.Protected`) oder die Simulation behandelt `Free` gleich.

## Nicht-Ziele

Ausblenden im Client.

## Regeln und Einschränkungen

`docs/rules/bedienung.md` § 3.

## Beispiele

Gerät verliert nachts die Verbindung, Gegner laufen über seinen Monarchen: keine HP-Verluste.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Go-Test: Ein wartender Monarch verliert während der Frist keine HP.

## Offene Fragen

keine (vermutet, ungeprüft im laufenden Spiel).

## Notizen

Angelegt in BD3.1.
