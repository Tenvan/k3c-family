# R3 · REG · Regelwerk II – Monarch, Bürger, Klassen, Level, Skills

- **Status:** erledigt
- **Domäne:** REG
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-110, B-017
- **Start-Commit:** 2038474
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

`game-design.md` beschreibt einen Skill-Baum mit vier Linien (Tank, Zauberer, Heiler, Dieb), Stats je Level (Monarch Level 1 bis 20) und Presets (`data/monarch.json`). Im Code gibt es davon nur Grundwerte und Verteidigung: keinen Monarch-Angriff, kein Level, keine Skills, keine Klassen (`docs/rules/ist-abgleich.md` § 4). Für **Bürger** (Bauern, Bogenschützen, Krieger, Elite, Landstreicher) gibt es keine Entwicklung außer Elite-Upgrades in den Daten ohne Code. Ob Spieler frei skillen oder eine Klasse als Preset bekommen, ist offen (B-017). Das Regelwerk II steht noch aus (B-004/AC-02).

## Ziel

Skillung, Klassen und Level von Monarchen und Bürgern sind beschlossen. Am Ende sichtbar: `docs/rules/monarch.md` (und `buerger.md` oder ein Abschnitt darin) mit Regel, Begründung, Verweis auf `data/` und Zielkorridoren, `game-design.md` ohne Widerspruch, Umsetzungs-Tickets für SIM und CLI.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; Familie als Spieler (2+ Monarchen, lokal und online); SIM und CLI setzen später um.

## Anforderungen

B-110 › Anforderungen und B-017 › Anforderungen. Sprint-eigen: Je Beschluss mit Zahlen ein messbarer Zielkorridor (Kennzahl, Szenario, Grenzen) für den Balancing-Tester (B-099); der Sprint baut ihn nicht. Beschlossene Rahmen bleiben (Wirtschaft, Stufen, Materialien: `docs/rules/wirtschaft.md`, `stufen.md`, `materialien-gebaeude.md`, Entscheidung 003).

## Nicht-Ziele

Gegner, Bosse und Truppen-Werte (Regelwerk III, B-004/AC-03), Materialien und Gebäude (R2, erledigt), Code und Änderungen in `data/` (SIM-Tickets), Grafiken und Reittiere (B-010).

## Regeln und Einschränkungen

Workshops sind Sessions mit `Agent: Mensch`. Werte bleiben in `data/`, Regeln verweisen darauf. Jede Regel gilt für 2+ Spieler. Taste B bleibt unbelegt, View + Menu sind reserviert, Taste X ist für Skills frei, das Skill-Menü liegt nicht auf View (`stufen.md` § 6). Mit Inseln und Stufen (pro Spieler frei begehbar) gilt: Fortschritt eines Spielers hängt nicht an einer gemeinsamen Reise.

## Beispiele

Workshop Monarch → `docs/rules/monarch.md` nennt Level-Kurve, Skill-Punkte je Stufe, Linien und Tiers, Slots und Tasten; ein Bürger-Abschnitt nennt, wie ein Bauer zum Bogenschützen wird und was ein Level bringt.

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Frage-Ticket, Thema im nächsten Workshop.

## Akzeptanzkriterien

- **AC-01** Der Monarch (Klassen, Level, Erfahrung, Skill-Punkte, Skill-Slots und Tasten) ist in `docs/rules/monarch.md` beschlossen, mit Zielkorridoren (B-110/AC-01).
- **AC-02** Die Bürger (Berufe, Level, Skills, Kosten) sind beschlossen, mit Zielkorridoren (B-110/AC-02).
- **AC-03** Klassen-Presets oder freie Wahl sind entschieden (B-017/AC-01).
- **AC-04** `game-design.md` stimmt mit den Beschlüssen überein (B-110/AC-03).
- **AC-05** Umsetzungs-Tickets für SIM und CLI liegen im Backlog, B-007 und B-017 sind angepasst (B-110/AC-04).

## Offene Fragen

- **Wer sind „Bürger“?** Angenommen Bauern, Bogenschützen, Krieger (Elite später) und Landstreicher; weitere Figuren (Händler, Handwerker) bestätigt 🧑 im Ist-Abgleich R3.1 oder im Workshop R3.3 (B-110 › Offene Fragen). Nicht blockierend für die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| R3.1 | `R3.1-ist-monarch-buerger.md` | Umsetzung | autonom | fertig |
| R3.2 | `R3.2-workshop-monarch.md` | Workshop | Mensch | fertig |
| R3.3 | `R3.3-workshop-buerger.md` | Workshop | Mensch | fertig |
| R3.4 | `R3.4-beschluss-tickets.md` | Umsetzung | autonom | fertig |

## Abnahme

- 2026-10-02, Doku-Sprint ohne Review (R3.4 schließt ab): AC-01 und AC-03 (R3.2 › Ergebnis), AC-02 und AC-04 (R3.3 › Ergebnis, R3.1 › Ergebnis), AC-05 (R3.4 › Ergebnis). Beschlüsse von 🧑 in den Workshops am 2026-10-02.
- Neu gegenüber der Spec: kein Level für Monarchen und Bürger, Schlag auf X, Wiederbeleben, Aktionen-Overlay, Handwerker und Händler.
- Neue Tickets: B-118 bis B-126. Offen: Regelwerk III (Gegner, Truppen-Werte, Bosse, B-004/AC-03), Zielkorridore der Stufen 4 und 5 (B-099).
