# B-004 · Regelwerk ist ausführlich diskutiert und ausgearbeitet

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1; Revision 2 (messbare Zielkorridore für den Balancing-Tester B-099) auf Zuruf von Ralf am 2026-10-02

## Ausgangslage

Die Regeln stehen verstreut in `docs/game-design.md`, im TS-Code (`src/world/sim/`) und in `data/`.

## Ziel

Regelwerk ist ausführlich diskutiert und ausgearbeitet. Nutzen: Grundlage für alle SIM-Sprints in Go; ohne Regelwerk wird beim Portieren und Erweitern geraten.

## Beteiligte und Zielgruppen

Familie als Spieler; 🧑 entscheidet in Workshops, der Agent bereitet vor.

## Anforderungen

- Regeln mit Zahlen nennen einen messbaren Zielkorridor (Kennzahl, Szenario, Grenzen) für den Balancing-Tester (B-099).
- Je Thema eine Datei in `docs/rules/` mit Regel, Begründung und Verweis auf die Werte in `data/`.
- Widerspruchsfrei zu `game-design.md`, das die kurze Übersicht bleibt.
- Reihenfolge: Regelwerk I (Wirtschaft, Koop & Besitz, Stufen & Niederlage), II (Monarch & Skills), später Gegner & Truppen.

## Nicht-Ziele

Keine Umsetzung in Code; keine Wertänderung ohne Beschluss.

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/` verweisen darauf; `game-design.md` bleibt die Übersicht. Jede Regel gilt für 2+ Spieler. Workshops: der Agent bereitet vor und fragt einzeln, 🧑 entscheidet.

## Beispiele

Eine SIM-Session portiert die Wirtschaft → findet Regel und Wert in `docs/rules/wirtschaft.md`, statt im TS-Code zu raten.

## Ausnahme- und Fehlerfälle

Regel widerspricht dem heutigen Code → Widerspruch als Ticket, Beschluss im Workshop.

## Akzeptanzkriterien

- **AC-01** Regelwerk I (R1): `docs/rules/wirtschaft.md` und `docs/rules/stufen.md` sind beschlossen.
- **AC-02** Regelwerk II: Monarch & Skills sind beschlossen.
- **AC-03** Gegner & Truppen sind beschlossen.

## Offene Fragen

keine

## Notizen

Workshops: Claude bereitet vor und fragt einzeln, der Mensch entscheidet.

**Stand 2026-10-02 (R1 abgeschlossen):** Regelwerk I ist beschlossen (`docs/rules/wirtschaft.md`, `stufen.md`), **B-004/AC-01 ist erfüllt**. **AC-02 (Monarch, Skills, Bürger) ist mit R3 erfüllt** (2026-10-02, `docs/rules/monarch.md` und `buerger.md`). Offen: AC-03 (Gegner & Truppen inkl. Bosse, Vollmond-Event, Elite, Truppen-Limit, Regelwerk III). B-108 (Material je Insel, Inselfolge) ist am 2026-10-02 entschieden.
