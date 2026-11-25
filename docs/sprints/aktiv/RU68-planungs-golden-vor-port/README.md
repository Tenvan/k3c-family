# RU68 · DEV · Planungslogik der Workbench: Golden-Tests als Orakel vor dem Rust-Port

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

Entwurf. Die Planungslogik der Workbench (Tickets, Sprints, Sessions, Projekte, Index, Fahrplan, Wellen, `plan_next`) hat nur Go-Tests und keine Golden-Dateien; ein Rust-Port hätte kein Orakel. Kann sofort laufen, unabhängig von RU65.

## Ziel

Das heutige Verhalten der Planungslogik ist als Golden-Fälle festgehalten (Eingabe-Dateibaum, Aufruf, erwartete Ausgabe und Dateien), lesbar auch für einen Rust-Port; keine Verhaltensänderung.

## Beteiligte und Zielgruppen

Agenten schreiben die Golden-Fälle; sie sind später das Orakel für den Rust-Port (RU69).

## Anforderungen

Golden-Tests (Go) für die Planungslogik der Workbench in `tools/k3c-workbench/internal/planning`: je Fall ein Dateibaum unter `testdata/planung/<fall>/vorher`, ein Aufruf (`plan_create`, `plan_set`, `plan_sections`, `plan_delete`, `plan_next`, `plan_waves`, `plan_queue`, `plan_list`, `plan_check`, `plan_index`) als `aufruf.json` und die erwartete Antwort samt Dateibaum `nachher`. Abgedeckt sind Ticket, Sprint, Session und Projekt samt Index, Fahrplan, Projekt-Tabelle, Archiv und Verschieben nach Status. Der Test liest die Fälle ohne Go-spezifische Typen, sodass ein Rust-Test dieselben Dateien lesen kann.

## Nicht-Ziele

Verhalten ändern oder Fehler beheben (Fehler werden als Ticket erfasst, das Golden hält den Ist-Stand fest). Der Rust-Port selbst (RU69).

## Regeln und Einschränkungen

Golden schreibt nur ein bewusster Lauf (`task go:golden`-ähnlich, Begründung im Commit). Datei ≤ 400, Funktion ≤ 60 Zeilen. Kein Netz, keine Uhrzeit im Golden (feste Daten).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `testdata/planung/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** golden

## Beispiele

`plan_set B-001 Status erledigt` in einem Dateibaum mit Index und Archiv ergibt exakt die erwarteten Dateien unter `nachher`.

## Ausnahme- und Fehlerfälle

Fehlerfälle (unbekanntes Feld, ungültiger Wert, Zyklus bei Relationen) stehen als eigene Fälle mit erwarteter Fehlermeldung.

## Akzeptanzkriterien

- **AC-01** Mindestens ein Fall je Werkzeug und je Dokumentart, Fehlerfälle inklusive; `check_run go:test` grün.
- **AC-02** Der Golden-Test liest nur Dateien unter `testdata/planung/` (kein Go-Typ im Format).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU68.1 | `RU68.1-golden-format-lader.md` | Umsetzung | autonom | fertig |
| RU68.2 | `RU68.2-golden-faelle-tickets-sprints.md` | Umsetzung | autonom | fertig |
| RU68.3 | `RU68.3-golden-faelle-sessions-projekte-wellen.md` | Umsetzung | autonom | offen |
| RU68.4 | `RU68.4-review.md` | Review | autonom | offen |

Stichpunkte (Reife Entwurf): 1. Format und Lader (AC-02). 2. Fälle Tickets und Sprints (AC-01). 3. Fälle Sessions, Projekte, Wellen, Queue, Next (AC-01). 4. Review.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
