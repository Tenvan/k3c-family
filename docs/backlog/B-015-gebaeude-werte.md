# B-015 · Gebäude-HP und -Kosten sind gebalanced

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Außer Turm und Treppen sind die Werte in `buildings.json` Platzhalter.

## Ziel

Gebäude-HP und -Kosten sind gebalanced. Nutzen: Falsche Werte machen Nächte zu leicht oder unschaffbar.

## Beteiligte und Zielgruppen

Familie als Spieler; 🧑 entscheidet in Workshops, der Agent bereitet vor.

## Anforderungen

- HP und Kosten jedes Gebäudes in `buildings.json` sind begründet.

## Nicht-Ziele

Neue Gebäude.

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/` verweisen darauf; `game-design.md` bleibt die Übersicht. Jede Regel gilt für 2+ Spieler.

## Beispiele

Mauer-HP wird geändert → die Begründung steht in `docs/rules/`, die Änderung nur in `buildings.json`.

## Ausnahme- und Fehlerfälle

Der Spieleabend zeigt, dass ein Wert kippt → Wert anpassen, Begründung ergänzen.

## Akzeptanzkriterien

- **AC-01** Die Werte sind in `docs/rules/` begründet.
- **AC-02** Ein Spieleabend (B-008) hat sie geprüft.

## Offene Fragen

keine

## Notizen

–
