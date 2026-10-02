# B-109 · Materialien und Gebäude sind im Regelwerk beschlossen

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** R2
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

R1 hat nur den Besitz des Materials (je Insel) und die Sammelraten festgelegt. Welche Materialien es gibt, welche Gebäude es gibt, was sie kosten, aushalten und bewirken und wann sie freigeschaltet werden, steht nur als Daten in `data/buildings.json`, `data/economy.json`, `data/hub.json` und `data/troops.json`; außer Turm und Treppen sind die Werte Platzhalter (B-015). Im Code gibt es nur Bauplätze für Mauer, Turm, Werkstatt und Treppen; Tor, Farm und Kaserne haben keinen Bauplatz.

## Ziel

Materialien und Gebäude sind beschlossen: Liste, Kosten, HP, Wirkung, Freischaltung je Stufe und Insel, Platzierung im Hub, mit messbaren Zielkorridoren. Nutzen: SIM und Balancing-Tester (B-099) arbeiten ohne Raten; die Wirtschaft bekommt Tiefe über Materialien und Ausbau.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; Familie als Spieler; SIM und CLI setzen um.

## Anforderungen

- Je Material und je Gebäude eine Regel mit Begründung, Verweis auf `data/` und Zielkorridor in `docs/rules/materialien-gebaeude.md`.
- Gilt mit 2+ Spielern, je Insel gemeinsamer Material-Vorrat (B-108), Hubs je Stufe, Bau über Bauern und Bauplätze.
- Zielkorridore nutzen Kennzahlen aus `docs/rules/ist-abgleich.md` › Messgrößen, fehlende Kennzahlen gehen an B-099.

## Nicht-Ziele

Gegner, Truppen-Werte und Bosse (Regelwerk III), Monarch und Skills (Regelwerk II), Code und Wertänderungen in `data/` (SIM-Tickets), Grafiken (B-010).

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/` verweisen darauf; jede Regel gilt für 2+ Spieler (`CLAUDE.md`, `docs/arbeitsweise.md`). B-015 (Werte) wird durch die Beschlüsse zu Startwerten und mit dem Balancing-Tester feinjustiert.

## Beispiele

Workshop Gebäude → `docs/rules/materialien-gebaeude.md` nennt je Gebäude Kosten, HP, Bauzeit, Wirkung, Freischaltung und einen Zielkorridor (z. B. „Mauer vor Ende Tag 1“).

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Ticket vom Typ Frage, Thema im nächsten Workshop. Ein Gebäude ohne Code → Beschluss „später gebaut“ mit SIM-Ticket.

## Akzeptanzkriterien

- **AC-01** Die Materialien (Liste, Quellen, Verwendung, Freischaltung je Stufe) sind in `docs/rules/materialien-gebaeude.md` beschlossen.
- **AC-02** Die Gebäude (Liste, Kosten, HP, Bauzeit, Wirkung, Freischaltung, Platzierung im Hub) sind dort beschlossen, je mit Zielkorridor.
- **AC-03** `game-design.md` stimmt mit den Beschlüssen überein.
- **AC-04** Umsetzungs-Tickets für SIM und CLI liegen im Backlog (B-015 wird angepasst).

## Offene Fragen

keine (die Fragen entstehen im Ist-Abgleich R2.1).

## Notizen

Gestartet am 2026-10-02 auf Wunsch von 🧑 („nochmal über Materialien und Gebäude reden“). Schließt B-015 nicht ab: B-015 bleibt das Ticket für das Feintuning der Zahlen mit dem Balancing-Tester.
