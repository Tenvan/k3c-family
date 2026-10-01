# B-074 · Golden-Läufe decken Tragen, Bauen, Bögen, Truhen und Münz-Rückgabe ab

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP06
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SP05.3 vergleicht die Wirtschaft in Go mit den Golden-Läufen ohne Gegner (`sim-forest-tag` komplett,
`sim-forest-nacht` bis Tick 960, `sim-cave-aggression` bis Tick 1770). In diesem Bereich kommen Rekrutieren,
Markieren, Sammeln (`gather`), Skill-Punkte, Morgen-Gold und fallengelassene Münzen vor. Es fehlen: Tragen bis zum Hub
(`carry`, Vorrat > 0, Aggression durch Sammeln), Material abziehen (`stepSites`), Bauen (`waitingWorker`, `build`),
Werkstatt-Bögen und `fetchBow`, Bogenschützen auf dem Turm, Truhen (`rng.Int` und verstreute Münzen) und die
Rückgabe nicht vollendeter Zahlungen (`refundPending`). Code für diese Fälle ist in `engine/sim` portiert, aber nicht
gegen TS geprüft.

## Ziel

Jeder portierte Pfad der Wirtschaft und der Truppen ist durch mindestens einen Golden-Lauf belegt, damit stille
Abweichungen zwischen Go und TS auffallen.

## Beteiligte und Zielgruppen

Entwickler oder Agent (SIM).

## Anforderungen

- Neue oder verlängerte Golden-Läufe in `tests/golden.test.ts`, die die Pfade aus der Ausgangslage durchlaufen.

## Nicht-Ziele

Neue Mechaniken; Änderungen an `src/world/`.

## Regeln und Einschränkungen

Wie B-043. Bestehende Golden-Dateien ändern sich nur, wenn ihr Lauf bewusst angepasst wird.

## Beispiele

Lauf mit vielen Bäumen in Hub-Nähe und Gold für Mauer und Werkstatt → Bauern tragen Holz, bauen die Mauer,
die Werkstatt fertigt Bögen, ein Bauer wird Bogenschütze.

## Ausnahme- und Fehlerfälle

Ein Pfad ist mit dem Skript nicht erreichbar → im Ticket begründen.

## Akzeptanzkriterien

- **AC-01** Ein Test zählt je Pfad aus der Ausgangslage, in wie vielen Snapshots ohne Gegner er vorkommt. Jeder Pfad kommt mindestens einmal vor, und `go test ./engine/sim/` ist grün.

## Offene Fragen

keine

## Notizen

Abdeckung in SP05.3 per Auswertung der Golden-Snapshots bestimmt (Ergebnis der Session SP05.3).
