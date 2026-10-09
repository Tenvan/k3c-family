# R4 · REG · Regelwerk III – Gegner, Wellen, Bosse, Events

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-127
- **Start-Commit:** d65d93e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

Wirtschaft, Stufen, Materialien, Gebäude, Monarch und Bürger sind beschlossen (R1 bis R3). Gegner stehen als Daten in `data/enemies.json` (Wald: Greed, Wolf, Goblin, Goblin Archer; Höhle: Skelett, Fledermaus, Höhlentroll; Mine: Zombie, Rattenschwarm, Minengeist), Wellen in `data/waves.json`. Eisenstollen und Kristallhöhle haben keine Gegner, es gibt keine Bosse und keine Events (`ist-abgleich.md`, `stufen.md`). Das Regelwerk III steht aus (B-004/AC-03).

## Ziel

Gegner, Wellen, Bosse und Events sind beschlossen. Am Ende sichtbar: `docs/rules/gegner.md` und `docs/rules/bosse.md` mit Regel, Begründung, Verweis auf `data/` und Zielkorridoren, `game-design.md` ohne Widerspruch, Umsetzungs-Tickets für SIM und CLI.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; die SIM-Sprints (Bosse B-103, neue Stufen B-115, Elite-KI B-013) nutzen das Ergebnis.

## Anforderungen

B-127 › Anforderungen. Sprint-eigen: Je Beschluss mit Zahlen ein messbarer Zielkorridor (Kennzahl, Szenario, Grenzen) für den Balancing-Tester (B-099); der Sprint baut ihn nicht. Beschlossene Rahmen bleiben (Wirtschaft, Stufen, Materialien, Monarch, Bürger).

## Nicht-Ziele

Code und Änderungen in `data/` (SIM-Tickets), Grafiken und Sound (B-010, B-011), weitere Inseln über Insel 1 hinaus (Inhalt, später).

## Regeln und Einschränkungen

Workshops sind Sessions mit `Agent: Mensch`. Werte bleiben in `data/`, Regeln verweisen darauf. Jede Regel gilt für 2+ Spieler. Schwierigkeitsgrade ändern Wellen und Gegner (`wirtschaft.md` § 4); Wellenfaktor je Spieleranzahl und Insel-Tabelle sind beschlossen. Skill-Punkte von Bossen: Miniboss 1, Endboss 3 (`monarch.md` § 3).

## Beispiele

Workshop Bosse → `docs/rules/bosse.md` nennt für den Miniboss der Höhle Werte, Fähigkeiten, Auslöser und Belohnung und einen Zielkorridor.

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Frage-Ticket, Thema im nächsten Workshop.

## Akzeptanzkriterien

- **AC-01** Gegner-Pools, Werte und Traits je Stufe der Insel 1 sind in `docs/rules/gegner.md` beschlossen, mit Zielkorridoren (B-127/AC-01).
- **AC-02** Die Wellen (Tabelle, Portale, Rhythmus, Zusammenspiel mit Spieleranzahl, Grad, Insel, Aggressionspool) sind dort beschlossen (B-127/AC-02).
- **AC-03** Minibosse und Endboss sind in `docs/rules/bosse.md` beschlossen, mit Zielkorridoren (B-127/AC-03).
- **AC-04** Events (Vollmond u. a.) sind beschlossen oder ausdrücklich verschoben (B-127/AC-04).
- **AC-05** `game-design.md` stimmt mit den Beschlüssen überein (B-127/AC-05).
- **AC-06** Umsetzungs-Tickets für SIM und CLI liegen im Backlog, B-013, B-103 und B-115 sind angepasst (B-127/AC-06).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| R4.1 | `R4.1-ist-gegner-bosse.md` | Umsetzung | autonom | fertig |
| R4.2 | `R4.2-workshop-gegner-wellen.md` | Workshop | Mensch | fertig |
| R4.3 | `R4.3-workshop-bosse-events.md` | Workshop | Mensch | fertig |
| R4.4 | `R4.4-beschluss-tickets.md` | Umsetzung | autonom | fertig |

## Abnahme

- 2026-10-02, Doku-Sprint ohne Review (R4.4 schließt ab): AC-01 und AC-02 (R4.2 › Ergebnis), AC-03, AC-04 und AC-05 (R4.3 › Ergebnis), AC-06 (R4.4 › Ergebnis); R4.1 lieferte Ist-Stand und Fragen. Beschlüsse von 🧑 in den Workshops am 2026-10-02.
- Neu gegenüber der Spec: neue Gegner-Pools für Eisenstollen und Kristallhöhle, mehrere Events (Vollmond, Blutmond, Händler-Überfall).
- Neue Tickets: B-128 bis B-132. Regelwerk I bis III sind damit beschlossen (B-004 erledigt).
