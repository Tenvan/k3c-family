# WZ50 · DEV · Katalog-Vitests der Workbench bleiben unter Last grün

- **Status:** aktiv
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
| WZ50.2 | `WZ50.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
