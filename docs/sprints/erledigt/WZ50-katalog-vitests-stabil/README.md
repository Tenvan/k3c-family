# WZ50 · DEV · Katalog-Vitests der Workbench bleiben unter Last grün

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-837
- **Start-Commit:** 28bdaab79
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13 (Ralf: autonome Tickets abarbeiten)

## Ausgangslage

Review WZ42.4: `check:dev` war zweimal rot, jeweils nur in Katalog-Vitests unter `tools/k3c-workbench/frontend/src/assets/catalog` (CatalogCache, CatalogRuhig); einzeln laufen sie grün (B-837).

## Ziel

Die Katalog-Vitests bleiben unter Last grün, `task check:dev` zweimal hintereinander grün.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-837 › Ziel`.

## Nicht-Ziele

Neue Katalog-Funktionen (WZ43).

## Regeln und Einschränkungen

Klasse A (Problem, DEV). Nach WZ43 (gleicher Bereich Katalog).

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/catalog/**`, `tools/k3c-workbench/frontend/vitest*.ts`
- **Abhängig von:** WZ43
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-837` Kriterien grün; `task check:dev` zweimal hintereinander grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ50.1 | `WZ50.1-katalog-vitests.md` | Umsetzung | autonom | fertig |
| WZ50.2 | `WZ50.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14, Claude. AC-01 geprüft (WZ50.1 › Ergebnis, WZ50.2 › Ergebnis): Katalog-Vitests 40 von 40 Mal grün unter Last, `task check:dev` zweimal hintereinander grün, `task check` und `go:test` grün.
Behoben: Die Katalog-Tests warten jetzt auf Bedingungen statt auf `flush()` und feste Zeiten (`catalog/catalogTesting.tsx`).
Neue Tickets (anzulegen, durch den Aufrufer): `readonly.test.tsx` unter Last, `TestDateiAenderungStartetNeu` unter Last.
Abnahme-Ticket für Sim, Balance und Gen: keins nötig, der Sprint ändert nur Frontend-Tests ohne Sim-, Balance- oder Gen-Bezug.
