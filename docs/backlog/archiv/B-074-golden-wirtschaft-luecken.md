# B-074 · Golden-Läufe decken Tragen, Bauen, Bögen, Truhen und Münz-Rückgabe ab

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SP06
- **Projekt:** –
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP06: campaign-abstieg, Abdeckung ≥ 90 %, B-059 in SP06.2, Golden ≤ 8 MB)

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

- Neue Golden-Läufe in `tests/golden.test.ts`, die die Pfade aus der Ausgangslage durchlaufen. Maßstab ist die
  Go-Abdeckung von `engine/sim` durch Tests, die jeden Snapshot vollständig mit TS vergleichen.

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

- **AC-01** `go test -cover ./engine/sim/` ist grün und meldet mindestens 90 % der Anweisungen; die Pfade aus der Ausgangslage sind ausgeführt (`go tool cover -func`), nicht erreichbare sind begründet.

## Offene Fragen

keine

## Notizen

Abdeckung in SP05.3 per Auswertung der Golden-Snapshots bestimmt (Ergebnis der Session SP05.3). Umsetzung: SP06.3, erledigt: Abdeckung 94,5 % mit sechs neuen Läufen und Regel-Tests aus TS (Ergebnis SP06.3).
