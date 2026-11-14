# B-863 · Die Planungs-Werkzeuge verarbeiten Ticket-Nummern ab B-1000

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ54
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

WZ47.4: `reTicketID` (`\d{3}`) und Schlüssel `[:5]` in `internal/planning` brechen ab vierstelligen Nummern; höchste Nummer heute B-864.

## Ziel

Planungs-Werkzeuge und Planungstests verarbeiten Nummern ab B-1000 wie dreistellige.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- Muster `\d{3,}`, Schlüssel ohne feste Länge.
- Sortierung numerisch.
- Gleiches in `tests/planning*.test.ts`.

## Nicht-Ziele

Umnummerierung.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/assetkit/ticket/**`, `tests/planning*.test.ts`
- **Abhängig von:** WZ47
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Test-Repo mit B-999 und B-1000 → `plan_create` vergibt B-1001, Index sortiert richtig.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Go- und Vitest-Test mit B-1000.

## Offene Fragen

keine

## Notizen

Befund WZ47.4 (2026-11-14).
