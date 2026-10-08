# B-289 · Der Aggressionspool steigt mit Adern nicht zu schnell

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG1
- **Projekt:** BAL
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Unter Tage steigt der Aggressionspool je abgegebener Lieferung um `percentPerGather` (Höhle und Mine: 1 %, `data/biomes/*.json` › `cycle`), dazu 1 %/min. Seit W2.1 liefern Adern unendlich: 2 Bauern an einer Stein-Ader geben 6 Lieferungen/min ab, der Pool steigt damit um etwa 7 %/min statt 1 %/min, eine Welle kommt nach rund 14 statt 100 Minuten (Kupfer-Ader: 4 Lieferungen/min, etwa 5 %/min).

## Ziel

Der Wellen-Takt unter Tage passt zu den Zielkorridoren, auch wenn Adern laufen.

## Beteiligte und Zielgruppen

Spieler unter Tage; 🧑 entscheidet die Werte (Balancing B-099).

## Anforderungen

- Wellen-Abstand unter Tage mit 2 Bauern an einer Ader liegt im Zielkorridor (F1).

## Nicht-Ziele

Raten der Adern selbst (B-114, Startwerte).

## Regeln und Einschränkungen

Werte nur in `data/`; zählt eine Ader-Lieferung anders als eine endliche Ressource, ist es ein SIM-Ticket.

## Beispiele

2 Bauern an der Stein-Ader der Höhle → nächste Welle nicht vor dem Korridor-Minimum.

## Ausnahme- und Fehlerfälle

nicht relevant (Wertfrage).

## Akzeptanzkriterien

- **AC-01** `task balance` oder ein Sim-Lauf zeigt den Wellen-Abstand unter Tage mit laufender Ader im Zielkorridor.

## Offene Fragen

Weg (🧑): `percentPerGather` senken, Ader-Lieferungen nicht oder schwächer zählen, oder so lassen.

## Notizen

Gefunden in W2.1 (`gatherPressure`, `engine/sim/units.go`).
