# B-528 · Entscheidung: Regeldetails der Raum-Pause (Couch-Raum, Schutz, Frist)

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR1
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BD3 setzt die Raum-Pause aus `docs/rules/bedienung.md` § 1 um. Die Regel lässt Einzelheiten offen; Claude hat sie im autonomen Lauf mit Default entschieden (`engine/room/pause.go`, `engine/sim`).

## Ziel

🧑 bestätigt die Defaults oder wählt die Alternative; bei Abweichung entsteht ein Umsetzungs-Ticket.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

`docs/rules/bedienung.md` § 1, § 3.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat jede der drei Fragen unter Notizen beantwortet.

## Offene Fragen

Siehe Notizen (drei Punkte, je Default und Alternative).

## Notizen

1. **Couch-Raum:** Default: Der Raum steht, wenn das pausierte Gerät das einzige verbundene ist (wartende Monarchen getrennter Geräte zählen nicht). Alternative: Der Raum steht auch, wenn alle verbundenen Geräte pausiert haben.
2. **Unverwundbar:** Default: Gegner wählen den geschützten Monarchen nicht als Ziel und Schaden trifft ihn nicht (`sim.Player.Protected`), er blockt also keine Gegner. Alternative: Gegner greifen ihn an, Treffer machen 0 Schaden (er wäre 60 s ein Schild an der Mauer).
3. **Frist:** Default: 60 s = 1800 Ticks des Raums, gezählt ab dem ersten Tick, in dem das Gerät pausiert ist und der Raum rechnet (aus einer Couch-Pause, zu der ein zweites Gerät kommt, beginnt sie dann voll); ein erneutes `pause` nach `resume` beginnt eine neue Frist. Alternative: Frist nach Wanduhr ab `pause` wie `WaitFor` bzw. ein Kontingent je Nacht.
