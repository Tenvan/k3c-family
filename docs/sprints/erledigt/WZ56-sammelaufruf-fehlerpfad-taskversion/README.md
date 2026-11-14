# WZ56 · DEV · Sammelaufruf: Fehlerantwort mit Teilergebnis, Task-Version gemerkt

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-870, B-871
- **Start-Commit:** 5d8fb8ac2
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Befunde WZ55.2: Fehlerantwort von `assets_set` beim Abbruch des Sammelaufrufs ohne Teilergebnis (B-870), `task --version` bei jedem Lauf (B-871).

## Ziel

Fehlerantwort mit Teilergebnis, Task-Version gemerkt.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-870 › Anforderungen`, `B-871 › Anforderungen`.

## Nicht-Ziele

Neue Funktionen.

## Regeln und Einschränkungen

Klasse A (Problem und Schuld, DEV).

- **Bereich:** `tools/k3c-workbench/internal/paperassets/**`, `tools/paper/fixtures/mcp/**`, `tools/k3c-workbench/internal/mcpsrv/check_taskversion*.go`
- **Abhängig von:** WZ55
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-870/AC-01` grün.
- **AC-02** `B-871/AC-01` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ56.1 | `WZ56.1-fehlerpfad-taskversion.md` | Umsetzung | autonom | fertig |
| WZ56.2 | `WZ56.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14: AC-01 (B-870) und AC-02 (B-871) erfüllt, Nachweise in WZ56.1 und WZ56.2 › Ergebnis. Behoben im Review: Fallback-diff nach Zeitlimit, Neustart-Hinweis, Fixture-Test. Neue Tickets: keine, Abnahme-Ticket entfällt (kein Sim-, Balance- oder Gen-Bezug). Tickets B-870 und B-871 noch auf eingeplant (Archivieren folgt mit neuer Workbench-EXE).
