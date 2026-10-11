# WZ3.1 · Umsetzung der Review-Nachträge zu WZ2

- **Status:** blockiert
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz3/1-umsetzung
- **Abhängig von:** –
- **Tickets:** B-403
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Die Review-Nachträge aus B-403 sind umgesetzt: Die Projekt-Ansicht zählt ausgefilterte Sprints nicht mit, die Lücken in `ticketnr.go`, `tables.go`, `check_job.go` und `check_background_test.go` sowie der Splitter-Test sind geschlossen. Am Ende sind `task check:dev` und die Tests grün.

## Kontext

- Entscheidungen (🧑, 2026-10-11): M1 → Sprints ausgefilterter Projekte verschwinden aus der Projekt-Ansicht und zählen nicht mit. Splitter → Höhe des Detailbereichs reicht, kein zweiter Balken.
- `ticketnr.go` (N1): Schlägt `isRepoRoot` fehl (Junction, `subst`) oder liefert `git worktree list` einen Fehler, muss die Antwort das nennen, statt Worktrees und Refs stillschweigend wegzulassen.
- `tables.go` (N4): Ist `Projekt` eines Sprints leer statt `–`, springt die Fahrplan-Zeile bei jedem `plan_set` ans Tabellenende. Ein Test fehlt.
- `check_job.go` (N5): Jeder Job hält die volle Ausgabe (bis 20 Jobs). Es sollen nur Fehlerzeilen bleiben.
- `check_background_test.go` (N6): Sleep-Schleife, soll über einen Kanal warten.
- `Splitter.test.tsx` (AC-08): prüft nur statisch. Ziehen und Doppelklick fehlen; der Test „fehlt in der Projekt-Ansicht“ ist tautologisch.
- `SprintsBacklog.tsx` und `projects.ts` (M1): Die Zählung „Sprints x von y“ berücksichtigt ausgefilterte Projekte aktuell.
- Komplexitätsbudget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Keine Spielcode-Änderung.

## Erlaubte Dateien

- `tools/k3c-dev/internal/planning/` (ticketnr.go, tables.go und ihre Tests)
- `tools/k3c-dev/internal/mcpsrv/check_job.go`, `check_background_test.go`
- `tools/k3c-dev/frontend/src/planning/SprintsBacklog.tsx`, `projects.ts`, `Splitter.test.tsx` (und ggf. `Splitter.tsx`)
- `docs/backlog/B-403-wz2-review-nachtraege.md` (Status- und Notiz-Felder)

## Nicht-Ziele

- Keine Spec-Korrektur von B-400 („Breiten“ → „Höhe“); das ist ein separater Schritt des Nutzers.
- Keine Änderung der Doku „Hardware-Bahn“ in `B-186` und `PF1.4` (eigenes Ticket).
- Kein Commit ohne Auftrag, keine Freigabe, keine manuelle Abnahme.

## Schritte

1. Sprint-Branch `sprint/wz3` von `origin/develop` anlegen, Session-Branch `wz3/1-umsetzung` darauf.
2. N1: `ticketnr.go` um die Fehlermeldung bei fehlgeschlagenem `isRepoRoot` und `git worktree list` ergänzen, Test dazu (AC-02).
3. N4: Test für leeres `Projekt` in `tables.go` schreiben, Fehler beheben (AC-03).
4. N5: `check_job.go` nur Fehlerzeilen halten (AC-04).
5. N6: `check_background_test.go` mit Kanal statt Zeit warten lassen (AC-05).
6. M1: `projects.ts` und `SprintsBacklog.tsx` so anpassen, dass ausgefilterte Projekte nicht mitzählen; Test (AC-01).
7. AC-08: `Splitter.test.tsx` um Ziehen und Doppelklick erweitern, Tautologie entfernen (AC-06).
8. `task check:dev` ausführen und grün bekommen.

## Fertig, wenn

- [ ] AC-01: Ausgefilterte Projekte fehlen in der Zählung „Sprints x von y“ (Test in `projects`/`SprintsBacklog`).
- [ ] AC-02: `ticketnr.go` nennt fehlende Worktrees und Refs im Fehlerfall (Go-Test).
- [ ] AC-03: Leeres `Projekt` springt nicht ans Tabellenende (Go-Test in `tables.go`).
- [ ] AC-04: `check_job.go` hält nur Fehlerzeilen (Go-Test).
- [ ] AC-05: `check_background_test.go` wartet über Kanal (Test läuft ohne Zeit-Schleife).
- [ ] AC-06: `Splitter.test.tsx` prüft Ziehen und Doppelklick, kein tautologischer Test.
- [ ] Technisch: `task check:dev` grün.

## Prüfen

```bash
task check:dev
```

Über k3c-dev: `check_run` mit `checkout` der Repo-Wurzel (`C:\WORKSPACE\K3C`).

## Ergebnis

**Blockiert** (Stand des Laufs, 2026-10-11): AC-06 (Splitter-Test) ist nicht umgesetzt. Grund: Die Frontend-Tests
laufen ohne DOM-Umgebung (kein jsdom, kein happy-dom im Root-Vitest). Ziehen und Doppelklick des Splitters
(`tasks/TaskBars.tsx`, nicht in den erlaubten Dateien) lassen sich so nicht prüfen. Entscheidung nötig: DOM-Umgebung
als neue Abhängigkeit, oder Splitter-Logik in eine reine Funktion auslagern (`TaskBars.tsx` wird dann geändert).

- AC-01 geprüft: `projects.test.ts` (Zählung ohne Sprints ausgeblendeter Projekte) grün; `tsc --noEmit` für `tools/k3c-dev/frontend` grün.
- AC-02 geprüft: `TestTicketNummerKeineRepoWurzelNennt` grün (`ticketnr.go` nennt die fehlende Repo-Wurzel).
- AC-03 geprüft: `TestFahrplanOhneProjektBehaeltPlatz` rot vor dem Fix (`X2 X1`), grün danach (`tables.go`, `projectCell`).
- AC-04 geprüft: `TestCheckJobHaeltNurFehlerzeilen` grün; Job hält Fehlerzeilen und `fallbackLines` Endzeilen.
- AC-05 geprüft: `TestCheckRunLangerLaufLiefertLaufID` wartet auf `job.done`, keine Sleep-Schleife mehr.
- AC-06 **offen**: siehe Blocker oben.
- Abweichungen: `task check:dev` nicht gelaufen. k3c-dev-MCP war während des Laufs getrennt; stattdessen einzeln
  `go test ./internal/planning/` und `./internal/mcpsrv/` (grün), `go vet` (grün), vitest für `projects.test.ts` (8/8),
  `tsc --noEmit` (grün).
- Beobachtung (neues Ticket nötig, nicht Teil dieser Session): In `docs/sprints/README.md` steht die Zeile WZ3 in
  „Aktiv“ mit Projekt `–`, obwohl das Kopffeld `WZG` lautet.
- Manuell geprüft: niemand. Neue Tickets: keine angelegt (k3c-dev war nicht verbunden).
