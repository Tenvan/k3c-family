# B-075 · Der Golden-Spielstand enthält einen gebauten und veränderten Hub

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W7
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`testdata/golden/campaign-abstieg.json` speichert und lädt kurz nach der Ankunft in cave (SP06.2). Im Spielstand sind
alle Bauplätze `unpaid`, `nodesGone` und `nodesMarked` leer, die Truppen nur die Start-Truppen. `applyHub`,
`applySite` und das Entfernen gefällter Bäume und geöffneter Truhen in `engine/sim/save.go` sind daher nur durch
Lesen gegen `src/world/sim/campaign.ts` und durch die Regel-Tests in `rules_test.go` abgesichert, nicht durch
Golden-Daten (Review SP06.4).

## Ziel

Laden und Speichern eines ausgebauten Hubs ist gegen TS geprüft. Nutzen: Ein Spielstand verliert in Go nichts, was
TS behält.

## Beteiligte und Zielgruppen

Spieler (Spielstände); Umsetzung durch Entwickler oder Agent (SIM).

## Anforderungen

- Ein Golden-Spielstand mit gebauten und bezahlten Bauplätzen, Bögen, Bauern und Bogenschützen, gefällten bzw.
  markierten Bäumen und geöffneter Truhe, dazu der Weiterlauf nach dem Laden.

## Nicht-Ziele

Spielstand-Version 2; Änderungen an `src/world/`.

## Regeln und Einschränkungen

Wie B-043; bestehende Golden-Dateien ändern sich nicht, Golden-Daten gesamt höchstens 8 MB (SP06).

## Beispiele

Aufbau-Bot aus `tests/golden.test.ts` in einer Kampagne, Speichern nach dem Bau, Laden, 600 Ticks weiter →
Go und TS identisch.

## Ausnahme- und Fehlerfälle

nicht relevant: reiner Testausbau.

## Akzeptanzkriterien

- **AC-01** Ein Golden-Spielstand mit nicht leerem `nodesGone`, `nodesMarked`, `pickupsTaken`, gebauten Bauplätzen und
  Bogenschützen wird in Go wie in TS geschrieben, geladen und weitergerechnet (`go test ./engine/sim/` grün).

## Offene Fragen

keine

## Notizen

Golden-Daten liegen nach SP06 bei 7,87 MB; ein Spielstand allein ist klein, der Weiterlauf sollte kurz bleiben.
