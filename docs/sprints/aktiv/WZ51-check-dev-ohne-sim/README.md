# WZ51 · DEV · check:dev testet die Sim-Pakete der Workbench nicht, solange die Sim-Sperre gilt

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-857
- **Start-Commit:** cab18b488
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Befund WZ48.4 (B-857): `k3c-workbench:test` läuft lokal über `./...` und nimmt die Sim-Pakete der Workbench trotz Sperre mit.

## Ziel

`task check:dev` und `dev:test` lassen die Sim-Pakete aus, solange die Sperre gilt.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-857 › Anforderungen`.

## Nicht-Ziele

Sim-Tests selbst.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/Taskfile.yml`, `tools/checkdiff/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-857/AC-01` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ51.1 | `WZ51.1-check-dev-paketliste.md` | Umsetzung | autonom | fertig |
| WZ51.2 | `WZ51.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
