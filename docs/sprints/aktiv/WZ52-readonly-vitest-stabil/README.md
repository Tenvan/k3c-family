# WZ52 · DEV · readonly-Vitest der Workbench bleibt unter Last grün

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-858
- **Start-Commit:** d0477bdb5
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Befund WZ50.1 (B-858): `readonly.test.tsx` ist unter Last wackelig, gleiche Ursache wie die Katalog-Tests (B-837).

## Ziel

Der readonly-Vitest bleibt unter Last grün.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-858 › Anforderungen`.

## Nicht-Ziele

Readonly-Logik.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/readonly.test.tsx`, `tools/k3c-workbench/frontend/src/assets/testing.tsx`, `tools/k3c-workbench/frontend/src/assets/catalog/catalogTesting.tsx`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-858/AC-01` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ52.1 | `WZ52.1-readonly-vitest.md` | Umsetzung | autonom | fertig |
| WZ52.2 | `WZ52.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
