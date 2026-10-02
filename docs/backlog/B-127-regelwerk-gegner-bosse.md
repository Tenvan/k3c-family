# B-127 · Gegner, Wellen, Bosse und Events sind im Regelwerk beschlossen

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** R4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

Regelwerk I (Wirtschaft, Stufen), die Materialien und Gebäude (R2) sowie Monarch und Bürger (R3) sind beschlossen. Offen ist Regelwerk III (B-004/AC-03): Welche Gegner es in welcher Stufe gibt, ihre Werte und Eigenschaften, die Wellen im Zusammenspiel mit Spieleranzahl, Grad und Insel, die **Minibosse** (je Stufe) und den **Endboss** (je Insel), Belohnungen und Events (Vollmond-Wolf als Idee). Die Gegner stehen heute in `data/enemies.json` (10 Arten für Wald, Höhle, Mine), die Wellen in `data/waves.json`; Eisenstollen und Kristallhöhle haben keine Gegner, Bosse gibt es nicht (`docs/rules/ist-abgleich.md`, `stufen.md`).

## Ziel

Gegner, Wellen, Bosse und Events sind beschlossen: Pools je Stufe, Werte, Eigenschaften (Traits), Elite-Verhalten, Bosse mit Werten, Verhalten und Belohnungen, Events mit Auslösern; jeweils mit messbaren Zielkorridoren. Nutzen: SIM kann die Bosse (B-103), die neuen Stufen (B-115) und die Elite-KI (B-013) ohne Raten bauen.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; Familie als Spieler (2+ Monarchen); SIM und CLI setzen später um.

## Anforderungen

- Je Stufe der Insel 1 (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle) ein Gegner-Pool aus Standard- und Elite-Gegnern mit Werten und Traits; nach unten steigende Schwierigkeit.
- Wellen im Zusammenspiel mit Spieleranzahl (×(1 + 0,5 je Zusatzspieler)), Schwierigkeitsgrad, Insel-Tabelle und Aggressionspool; Anzahl der Portale, Ankunftsrhythmus.
- **Minibosse:** je Stufe einer; **Endboss:** in der tiefsten Stufe jeder Insel. Werte, Verhalten (Phasen, Fähigkeiten), Auslöser (wann erscheint er), Belohnung (Skill-Punkte 1/3, Material, Gold) und Wirkung des Endbosses auf den Inselwechsel.
- **Events:** Vollmond-Wolf und weitere (Auslöser, Wirkung, Häufigkeit), soweit gewünscht.
- Zielkorridore für Kennzahlen aus `docs/rules/ist-abgleich.md` § 6; fehlende Kennzahlen gehen an B-099.
- Je Thema eine Datei in `docs/rules/` (`gegner.md`, `bosse.md`).

## Nicht-Ziele

Umsetzung in Go (SIM-Tickets), Grafiken und Sound (B-010, B-011), Bürger und Monarch (R3, erledigt), Materialien und Gebäude (R2, erledigt).

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/`; jede Regel gilt für 2+ Spieler. Aufbau Insel/Stufen (Entscheidung 003), Schwierigkeitsgrade und Wellenfaktor (`wirtschaft.md`), Skill-Punkte-Quellen (`monarch.md`) und Truppen (`buerger.md`) sind beschlossen.

## Beispiele

Workshop Bosse → `docs/rules/bosse.md` nennt für den Miniboss der Höhle Werte, Fähigkeiten, Auslöser und Belohnung; ein Zielkorridor sagt, in wie vielen Seeds er bis Tag 8 besiegt wird.

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Ticket vom Typ Frage, Thema im nächsten Workshop. Eine Idee ohne Code → „beschlossen, später gebaut“ mit SIM-Ticket.

## Akzeptanzkriterien

- **AC-01** Gegner-Pools, Werte und Traits je Stufe der Insel 1 (inkl. Eisenstollen und Kristallhöhle) sind in `docs/rules/gegner.md` beschlossen, mit Zielkorridoren.
- **AC-02** Die Wellen (Tabelle, Portale, Rhythmus, Zusammenspiel mit Spieleranzahl, Grad, Insel, Aggressionspool) sind dort beschlossen.
- **AC-03** Minibosse und Endboss (Werte, Verhalten, Auslöser, Belohnung) sind in `docs/rules/bosse.md` beschlossen, mit Zielkorridoren.
- **AC-04** Events (Vollmond u. a.) sind beschlossen oder ausdrücklich verschoben.
- **AC-05** `game-design.md` stimmt mit den Beschlüssen überein.
- **AC-06** Umsetzungs-Tickets für SIM und CLI liegen im Backlog (B-013, B-103, B-115 sind angepasst).

## Offene Fragen

keine (die Fragen entstehen im Ist-Abgleich R4.1).

## Notizen

Gehört zu B-004/AC-03 (Regelwerk III). Verwandt mit B-013 (Restliche Gegner und Elite-KI), B-103 (Inseln und Bosse), B-115 (neue Stufen), B-099 (Messung).
