# WZ48 · INF, DEV · Tests und CI: check:go ohne Sim, Workbench-Go-Tests in der CI, stabile und schnelle Tests

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** INF, DEV
- **Reife:** bereit
- **Tickets:** B-832, B-828, B-829, B-830, B-835
- **Start-Commit:** 30dcdee51
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13 (Ralf: autonom abarbeitbare Tickets abarbeiten)

## Ausgangslage

Aus den Reviews von Welle 5b (WZ41.3, WZ42.4, CU1.6) sind Test- und CI-Lücken offen: `task check:go` führt ohne `-short` die gesperrten Sim- und Lasttests aus (B-832), der CI-Job `k3c-workbench` fährt keine Workbench-Go-Tests (B-828), `TestDateiAenderungStartetNeu` wackelt (B-829), `mcpsrv` und `planning` sind mit `-short` zu langsam (B-830), und `cmd/k3c-server/k3c-server.exe` liegt als Binary im Repo (B-835).

## Ziel

Prüfläufe respektieren die Sim-Sperre, die CI prüft die Workbench-Go-Tests, die Workbench-Tests sind stabil und schnell, das Repo enthält keine gebaute EXE.

## Beteiligte und Zielgruppen

Claude (Umsetzung, autonom, Klasse A); 🧑 liest das Ergebnis.

## Anforderungen

`B-832 › Ziel`, `B-828 › Ziel`, `B-829 › Ziel`, `B-830 › Ziel`, `B-835 › Ziel`.

## Nicht-Ziele

Sim, Balancing, Lasttests (gesperrt); Katalog-Vitests (B-837, Welle 7, Bereich von WZ43).

## Regeln und Einschränkungen

Klasse A (Problem und Schuld, Domänen DEV und INF, ohne Spielregeln, Protokoll, Spielstand, neue Abhängigkeit). Sim und Balancing gesperrt (Beschluss 🧑 2026-11-11).

- **Bereich:** `Taskfile.yml`, `engine/Taskfile.yml`, `.github/workflows/ci.yml`, `tools/checkdiff/**`, `tools/k3c-workbench/internal/mcpsrv/**/*_test.go`, `tools/k3c-workbench/internal/planning/**/*_test.go`, `cmd/k3c-server/**`, `.gitignore`, Tests des Datei-Watch im Server (`engine/watch/**`, `cmd/k3c-server/*_test.go`)
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `task check:go` führt keine Sim-, Balance- oder Lasttests aus; Nachweis im Ergebnis (B-832).
- **AC-02** Die CI fährt die Workbench-Go-Tests mit `-short` (B-828).
- **AC-03** `TestDateiAenderungStartetNeu` stabil oder Ursache belegt (B-829).
- **AC-04** `mcpsrv` und `planning` mit `-short` im Rahmen von B-830.
- **AC-05** Keine gebaute EXE mehr im Repo, `.gitignore` verhindert neue (B-835).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ48.1 | `WZ48.1-check-go-ci-workbench.md` | Umsetzung | autonom | fertig |
| WZ48.2 | `WZ48.2-workbench-tests-stabil-schnell.md` | Umsetzung | autonom | fertig |
| WZ48.3 | `WZ48.3-server-exe-aus-repo.md` | Umsetzung | autonom | offen |
| WZ48.4 | `WZ48.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
