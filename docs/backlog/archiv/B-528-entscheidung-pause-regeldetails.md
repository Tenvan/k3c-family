# B-528 · Entscheidung: Regeldetails der Raum-Pause (Couch-Raum, Schutz, Frist)

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
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

**Entscheidung 🧑 (KR1.1, 2026-10-23):** alle drei Defaults bestätigt.

1. **Couch-Raum:** Der Raum steht, wenn das pausierte Gerät das einzige verbundene ist (wartende Monarchen getrennter Geräte zählen nicht).
2. **Unverwundbar:** Gegner wählen den geschützten Monarchen nicht als Ziel, Schaden trifft ihn nicht (`sim.Player.Protected`); er blockt keine Gegner.
3. **Frist:** 60 s = 1800 Raum-Ticks ab dem ersten Tick, in dem das Gerät pausiert ist und der Raum rechnet; ein erneutes `pause` nach `resume` beginnt eine neue Frist.

Alternativen (verworfen): Raum steht, wenn alle verbundenen Geräte pausiert haben; Gegner greifen an, Treffer machen 0 Schaden; Frist nach Wanduhr bzw. Kontingent je Nacht.
