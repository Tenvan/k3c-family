# B-219 · Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Mit B-216 gilt Tier-Gating 2/4/6 gelernte Skills der Linie (`docs/rules/monarch.md` § 3, `data/monarch.json › tierPoints`). `docs/game-design.md` (Abschnitt Skills) nennt noch 5/10/15. Seit S1.1 meldet auch der Monarch `strike` (Feld `from` = Spieler-ID); die Liste in `engine/sim/events.go` beschreibt `strike` noch als „Nahkampf-Schlag eines Gegners“. Beide Dateien lagen nicht in den erlaubten Dateien von S1.1.

## Ziel

Doku und Regelwerk widersprechen sich nicht.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Lesen der Doku.

## Anforderungen

- `docs/game-design.md` nennt Gating 2/4/6 bzw. verweist auf `docs/rules/monarch.md` § 3.
- Die Ereignisliste in `engine/sim/events.go` nennt `strike` für Gegner und Monarch (`from` = ID des Schlagenden).

## Nicht-Ziele

Änderungen an Werten oder Code-Verhalten.

## Regeln und Einschränkungen

`docs/rules/monarch.md` § 3 ist die Quelle; `events.go` nur Kommentar.

## Beispiele

nicht relevant (Doku-Abgleich).

## Ausnahme- und Fehlerfälle

nicht relevant (Doku-Abgleich).

## Akzeptanzkriterien

- **AC-01** Suche nach „ab 5, Tier 3 ab 10“ in `docs/game-design.md` ohne Treffer; `events.go` nennt den Schlag des Monarchen bei `strike`.

## Offene Fragen

keine

## Notizen

Gefunden in S1.1 (Sprint S1).
