# B-615 · Gleichzeitiges Anlegen von Tickets vergibt nie dieselbe Nummer

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review WZ28.7 (2026-10-24): `tools/assetkit/ticket/ticket.go` (Create, store) vergibt die Nummer und schreibt den Index ohne Sperre. Zwei gleichzeitige Aufrufe (Doppelklick, Studio und Workbench, `plan_create` parallel) bekommen dieselbe B-Nummer; der zweite Index-Eintrag überschreibt den ersten.

## Ziel

Jedes angelegte Ticket hat eine eigene Nummer, auch bei gleichzeitigen Aufrufen aus mehreren Prozessen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Sperre über Prozesse hinweg (Sperrdatei im Backlog oder exklusives Anlegen einer Nummern-Reservierung).
- Index-Schreiben unter derselben Sperre.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test: N parallele Create-Aufrufe liefern N verschiedene Nummern und N Index-Zeilen.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
