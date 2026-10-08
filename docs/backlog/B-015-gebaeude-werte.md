# B-015 · Gebäude-HP und -Kosten sind gebalanced

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BR1
- **Projekt:** –
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint BR1

## Ausgangslage

Außer Turm und Treppen sind die Werte in `buildings.json` Platzhalter. Mit R2 (2026-10-02) gibt es beschlossene **Startwerte** (`docs/rules/materialien-gebaeude.md`); sie sind noch nicht in `data/` und nicht gemessen.

## Ziel

Gebäude-HP und -Kosten sind gebalanced. Nutzen: Falsche Werte machen Nächte zu leicht oder unschaffbar.

## Beteiligte und Zielgruppen

Familie als Spieler; 🧑 entscheidet in Workshops, der Agent bereitet vor.

## Anforderungen

- HP und Kosten jedes Gebäudes in `buildings.json` sind begründet.
- Startwerte stehen in `docs/rules/materialien-gebaeude.md` (R2): Mauer und Turm Stufen 1–5, Hub-Ausbau, Gebäude-Kosten; sie werden mit dem Balancing-Tester (B-099) gegen die Zielkorridore geprüft und fein justiert (Wertänderung nur mit Beschluss).

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

Nach R2: Umsetzung der Startwerte in B-112 bis B-116, Messung und Feintuning mit B-099; dieses Ticket schließt, wenn die Werte gemessen und freigegeben sind (AC-01, AC-02).
