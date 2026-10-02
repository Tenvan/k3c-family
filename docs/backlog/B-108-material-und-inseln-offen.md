# B-108 · Material je Hub oder je Insel, Anzahl und Reihenfolge der Inseln sind entschieden

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Material je Hub, je Insel oder global? Wie viele Inseln, welche Reihenfolge? (🧑)

## Notizen

Aus R1.3 › Offen. Gehört in Regelwerk III oder einen kurzen Workshop vor B-100.
