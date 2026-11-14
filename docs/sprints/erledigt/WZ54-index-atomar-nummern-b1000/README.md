# WZ54 · DEV · Planungs-Index: Statuswechsel ohne Zeilenverlust, Ticket-Nummern ab B-1000

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-862, B-863, B-865, B-866
- **Start-Commit:** 7ce5b2894
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Befunde WZ47.4: Ein Statuswechsel schreibt Haupt- und Archiv-Index nicht atomar (B-862), die Werkzeuge brechen ab vierstelligen Nummern (B-863).

## Ziel

Kein Statuswechsel verliert eine Index-Zeile; Nummern ab B-1000 funktionieren.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-862 › Anforderungen`, `B-863 › Anforderungen`.

## Nicht-Ziele

Umnummerierung (B-861), `plan_index` (B-864).

## Regeln und Einschränkungen

Klasse A (Schuld, DEV). Nach WZ49 (gleicher Bereich `planning`, Sperre exe).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/assetkit/ticket/**`, `tests/planning*.test.ts`
- **Abhängig von:** WZ49
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-862/AC-01` grün.
- **AC-02** `B-863/AC-01` grün.
- **AC-03** `B-865/AC-01` grün (Fahrplan-Heilung nur bei Sprint- oder Session-Änderungen).
- **AC-04** `B-866/AC-01` grün (Testmuster mit Oder und Ankern).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ54.1 | `WZ54.1-index-atomar-b1000.md` | Umsetzung | autonom | fertig |
| WZ54.2 | `WZ54.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14, Claude (autonom). AC-01 bis AC-04 geprüft (WZ54.1, WZ54.2); `task check`, `task check:go`, `go:test`, `task check:dev` grün.
Behoben im Review: Rollback löscht nur neue Dateien und meldet unvollständiges Zurücknehmen; Test-Regexe ab B-1000.
Neue Tickets: keine angelegt (Vorschläge im Bericht: `remove` ohne Rollback, Task-Mindestversion für CLI_ARGS-Quoting).
Abnahme-Ticket: keins, kein Sim-, Balance- oder Gen-Bezug.
