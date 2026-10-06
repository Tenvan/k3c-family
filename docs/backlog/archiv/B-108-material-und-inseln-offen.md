# B-108 · Material je Hub oder je Insel, Anzahl und Reihenfolge der Inseln sind entschieden

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1 (Workshop)

## Ausgangslage

Heute gehört das Baumaterial dem Hub je Stufe (`World.stock`). Die Planung nennt n Inseln, die Anzahl über die erste hinaus und ihre Reihenfolge sind offen.

## Ziel

Entscheidung, ob das Material je Hub, je Insel oder für den ganzen Spielstand zählt, und wie viele Inseln es gibt und in welcher Reihenfolge. Nutzen: SIM kann Insel- und Stufenumbau (B-100, B-103) ohne Raten umsetzen.

## Beteiligte und Zielgruppen

🧑 entscheidet (Workshop, Agent bereitet vor).

## Anforderungen

- Entscheidung zum Material mit Begründung in `docs/rules/stufen.md`.
- Entscheidung zu Anzahl und Reihenfolge der Inseln.

## Nicht-Ziele

Umsetzung (B-100, B-103).

## Regeln und Einschränkungen

Werte in `data/`, Regeln in `docs/rules/`; jede Regel für 2+ Spieler. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

nicht relevant – reine Entscheidung, kein Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Entscheidung, kein Verhalten.

## Akzeptanzkriterien

- **AC-01** Die Entscheidungen stehen in `docs/rules/stufen.md`.

## Offene Fragen

Material je Hub, je Insel oder global? Wie viele Inseln, welche Reihenfolge? (🧑, entschieden, siehe Notizen)

## Notizen

Workshop 2026-10-02 mit 🧑: **Material gehört der Insel** (alle Stufen teilen einen Vorrat; neue Insel beginnt leer). **Inseln:** klassisch Insel 1 bis n in fester Reihenfolge (n offen, bis Insel 1 spielbar ist); **später** Variante „Ebenen“: mehrere Inseln je Ebene, freie Reihenfolge, nächste Ebene nach **mindestens k** besiegten Inseln der Ebene (k je Ebene in den Daten), Wechsel gemeinsam. Beschluss steht in `docs/rules/stufen.md` § 1.
