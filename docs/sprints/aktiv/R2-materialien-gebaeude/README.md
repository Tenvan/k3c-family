# R2 · REG · Regelwerk I b – Materialien & Gebäude

- **Status:** aktiv
- **Domäne:** REG
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-109
- **Start-Commit:** e03409f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

R1 hat Besitz des Materials (je Insel) und Sammelraten beschlossen. Materialien, Gebäudeliste, Kosten, HP, Wirkung und Freischaltung sind offen; die Werte in `data/buildings.json` sind außer bei Turm und Treppen Platzhalter (B-015). Der Code kennt nur Bauplätze für Mauer, Turm, Werkstatt und Treppen (`ist-abgleich.md`).

## Ziel

Materialien und Gebäude sind beschlossen. Am Ende sichtbar: `docs/rules/materialien-gebaeude.md` mit Liste, Kosten, HP, Wirkung, Freischaltung und Zielkorridoren, `game-design.md` ohne Widerspruch, Umsetzungs-Tickets für SIM und CLI.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; SIM und CLI setzen später um.

## Anforderungen

B-109 › Anforderungen. Sprint-eigen: Je Beschluss mit Zahlen ein messbarer Zielkorridor (Kennzahl, Szenario, Grenzen) für den Balancing-Tester (B-099); der Sprint baut ihn nicht. Benennung: Regelwerk II (Monarch & Skills) und III (Gegner, Truppen, Bosse) bleiben so benannt (B-004); ihre Sprints heißen später R3 und R4.

## Nicht-Ziele

Gegner, Truppen-Werte, Bosse (Regelwerk III), Monarch und Skills (Regelwerk II), Code und Änderungen in `data/` (SIM-Tickets), Grafiken (B-010).

## Regeln und Einschränkungen

Workshops sind Sessions mit `Agent: Mensch`. Werte bleiben in `data/`, Regeln verweisen darauf. Jede Regel gilt für 2+ Spieler. Besitz des Materials und Aufbau Insel/Stufen sind beschlossen (`docs/rules/stufen.md`, Entscheidung 003); dieser Sprint ändert sie nicht.

## Beispiele

Workshop Gebäude → `docs/rules/materialien-gebaeude.md` nennt je Gebäude Kosten, HP, Bauzeit, Wirkung, Freischaltung je Stufe und Zielkorridor.

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Frage-Ticket, Thema im nächsten Workshop.

## Akzeptanzkriterien

- **AC-01** Die Materialien sind in `docs/rules/materialien-gebaeude.md` beschlossen (B-109/AC-01).
- **AC-02** Die Gebäude sind dort beschlossen, je mit Zielkorridor (B-109/AC-02).
- **AC-03** `game-design.md` stimmt mit den Beschlüssen überein (B-109/AC-03).
- **AC-04** Umsetzungs-Tickets für SIM und CLI liegen im Backlog, B-015 ist angepasst (B-109/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| R2.1 | `R2.1-ist-material-gebaeude.md` | Umsetzung | autonom | fertig |
| R2.2 | `R2.2-workshop-material-gebaeude.md` | Workshop | Mensch | offen |
| R2.3 | `R2.3-beschluss-tickets.md` | Umsetzung | autonom | offen |

## Abnahme

–
