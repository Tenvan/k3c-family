# B-110 · Skillung, Klassen und Level von Monarchen und Bürgern sind im Regelwerk beschlossen

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** R3
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

`game-design.md` beschreibt einen Skill-Baum mit vier Linien (Tank, Zauberer, Heiler, Dieb), Stats je Level (Monarch Level 1 bis 20) und Presets (`data/monarch.json`). Im Code gibt es davon nur Grundwerte und Verteidigung; es gibt keinen Monarch-Angriff, kein Level, keine Skills und keine Klassen (`docs/rules/ist-abgleich.md` § 4). Für **Bürger** (Bauern, Bogenschützen, Krieger, später Elite) gibt es keine Entwicklung über Level oder Skills, nur Elite-Upgrades in den Daten (ohne Code). Das Regelwerk dazu steht noch aus (B-004/AC-02).

## Ziel

Skillung, Klassen und Level von Monarchen und Bürgern sind beschlossen: wie Fortschritt entsteht, was Klassen unterscheiden, wie Skill-Punkte verteilt werden, was Bürger lernen können. Nutzen: Grundlage für die SIM-Umsetzung (B-007) und die Balance im Koop; der Monarch kämpft aktiv mit, das unterscheidet das Spiel von K2C.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; Familie als Spieler (2+ Monarchen, lokal und online); SIM und CLI setzen um.

## Anforderungen

- **Monarch:** Klassen (heute: vier Linien und Presets), Level und Erfahrung (Quelle, Kurve, Höchststufe), Skill-Punkte (Quelle, Verteilung, Tier-Gating, Respec), Skill-Slots und Tasten (Taste X ist frei, das Skill-Menü liegt nicht auf View).
- **Bürger:** Welche Bürger es gibt (Bauer, Bogenschütze, Krieger, Elite), ob und wie sie Level, Skills oder Klassen bekommen (Berufe, Ausbildung, Verbesserungen), und was das kostet (Material und Gold je Insel, B-108).
- **Klassen im Koop:** Presets oder freie Skillung je Spieler (B-017), wie sich Rollen ergänzen (Tank, Heiler, Zauberer, Dieb), Verhalten bei 2 bis 4+ Spielern.
- **Messbar:** Zielkorridore für Fortschritt (z. B. Level je Tag, Skill-Punkte je Stufe) mit Kennzahlen aus `ist-abgleich.md` § 6; fehlende Kennzahlen gehen an B-099.
- Je Thema eine Datei in `docs/rules/` (`monarch.md`, Bürger dort oder in `buerger.md`) mit Regel, Begründung, Verweis auf `data/` und Zielkorridor.

## Nicht-Ziele

Umsetzung in Go oder im Client (eigene Tickets), Grafiken und Reittiere, Gegner und Bosse (Regelwerk III), Materialien und Gebäude (B-109).

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/`; jede Regel gilt für 2+ Spieler. Aufbau Insel/Stufen und Material je Insel sind beschlossen (`stufen.md`, B-108). Taste B bleibt unbelegt, View + Menu sind reserviert, X ist für Skills frei.

## Beispiele

Workshop Skills → `docs/rules/monarch.md` nennt Level-Kurve, Skill-Punkte je Stufe, Linien und Tiers; ein Bürger-Abschnitt nennt, wie ein Bauer zum Bogenschützen wird und was ein Level bringt.

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Ticket vom Typ Frage. Eine Idee ohne Code → „beschlossen, später gebaut“ mit SIM-Ticket.

## Akzeptanzkriterien

- **AC-01** Monarch (Klassen, Level, Erfahrung, Skill-Punkte, Skill-Slots) ist in `docs/rules/monarch.md` beschlossen, mit Zielkorridoren.
- **AC-02** Bürger (Berufe, Level, Skills, Kosten) sind beschlossen, mit Zielkorridoren.
- **AC-03** `game-design.md` stimmt mit den Beschlüssen überein.
- **AC-04** Umsetzungs-Tickets für SIM und CLI liegen im Backlog (B-007, B-017 sind angepasst).

## Offene Fragen

- **Wer sind „Bürger“?** Angenommen: Bauern, Bogenschützen und Krieger (Elite später) sowie Landstreicher. 🧑 bestätigt im Ist-Abgleich oder im Workshop, ob auch weitere Figuren (Händler, Handwerker o. ä.) gemeint sind.
- Reihenfolge der Regelwerke: vor oder nach R2 (Materialien & Gebäude), weil Bürger-Kosten Material nutzen. Entscheidet 🧑.

## Notizen

Entstanden am 2026-10-02 auf Hinweis von 🧑 („Monarchen und Bürger Skillung, Klassen, Level stehen noch aus“). Gehört zu B-004/AC-02 (Regelwerk II), verwandt mit B-007 (Skill-Baum, SIM), B-017 (Klassen-Presets, REG-Frage) und B-014 (Krieger, Elite). Der zugehörige Sprint (vorläufig R3) ist noch nicht geplant.
