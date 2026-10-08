# PJ2.2 · Sessions mit Domäne und verworfen, Sprint-Domänen abgeleitet

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** pj2/2-sessions-domaene-verworfen
- **Abhängig von:** PJ2.1
- **Tickets:** B-357, B-338
- **Kriterien:** AC-04, AC-05

## Ziel

Die plan-Tools lesen und schreiben das Session-Feld `Domäne` und den Status `verworfen`, leiten das Sprint-Feld `Domäne` (und die Überschrift) aus den Sessions ab und ordnen Sprints mit Projekt nicht mehr nach einer abgeleiteten Prio.

## Kontext

- Stand nach PJ2.1: Projekte in `Data.Projects`, `plan_*` kennen `kind: projekt`.
- Regeln (PJ1, `docs/arbeitsweise.md` › Domänen, Projekte und Rang; `tests/planningProjects.test.ts`): Sprint-Feld `Domäne` = Domänen seiner Sessions in Reihenfolge des ersten Auftretens, kommagetrennt, gilt bei `Reife: bereit`; die Überschrift `# ID · <Domänen> · Titel` trägt denselben Wert (`tests/planning.test.ts`). Session-Status `verworfen` zählt beim Abschließen wie `fertig`.
- Code: `edit.go` (`allowed["session"]["Status"]` ohne `verworfen`; `allowed["session"]` ohne `Domäne`; `Set` lehnt `Domäne` an Sprints ab: „neuen Sprint anlegen“), `create.go` (`createSprint` setzt `Prio` aus `SprintPrio`, Default `Einschiebbar: nein`), `priority.go` (`rank`: Prio aus Tickets, `Order` nach Prio), `list.go` (`sortByPrio`, `SprintPrio`). `Session` in `planning.go` hat kein Feld `Domain`.
- **Übergang Sprint-Prio (von 🧑 mit der Spec-Freigabe 2026-10-08 bestätigt):** `tests/planning.test.ts` (INF) verlangt weiter `Prio = höchste Ticket-Prio` für jeden Sprint mit Tickets, Vorlage und Planungstest ändert diese SRV-Session nicht. Deshalb: Die Tools **ordnen** Sprints mit `Projekt` ≠ `–` nach Rang des Projekts und Platz in seiner Sprint-Tabelle, nie nach Prio; das Feld `Prio` wird nur noch für Sprints mit `Projekt: –` aus den Tickets abgeleitet (Übergang wie in der Arbeitsweise) und für Sprints mit Projekt beim Schreiben unverändert gelassen. `Einschiebbar` schreibt `plan_create` nicht mehr aktiv (bleibt der Wert aus der Vorlage, solange sie das Feld hat). Wegfall der Felder in Vorlage und Test: Ticket für INF (Schritt 5), falls B-359 es nicht abdeckt.
- Tests: `write_test.go`, `priority_test.go`, `planning_test.go`.

## Erlaubte Dateien

- `tools/k3c-dev/internal/planning/` (Code und Tests), `tools/k3c-dev/internal/mcpsrv/tools_planning.go` (Beschreibungen)
- `tools/k3c-dev/frontend/src/api/types.ts`, `tools/k3c-dev/frontend/src/api/mockPlanning.ts` (nur neues Feld `domain` je Session, damit `dev:test` grün bleibt)
- `docs/sprints/aktiv/PJ2-projekte-k3c-dev/` (Status, Ergebnis), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Vorlagen und `tests/planning*.ts` (INF); Oberfläche (PJ2.3); Sprints umziehen oder Prio-Felder aus den Dateien löschen (PJ3).

## Schritte

1. Branch `sprint/pj2` holen, `git merge origin/develop`, `Status: in Arbeit`, committen, pushen.
2. `Session.Domain` aus der Session-Datei lesen (`sessionMeta`); `allowed["session"]`: `Domäne` mit den sechs Kürzeln, `Status` mit `verworfen`. `plan_create {kind: session}` verlangt `Domäne`.
3. Ableitung: Nach jeder Änderung an einer Session (anlegen, `Domäne` setzen, löschen) berechnet das Tool die Domänen des Sprints und schreibt Feld `Domäne`, Überschrift und Fahrplan-Spalte `Domäne`. Direktes Setzen von `Domäne` am Sprint bleibt abgelehnt (Grund: abgeleitet).
4. Prio: `rank` und `sortByPrio` ordnen Sprints mit Projekt nach (Rang, Tabellenplatz) vor Sprints ohne Projekt, diese weiter nach Prio; `Set`/`createSprint` leiten `Prio` nur bei `Projekt: –` ab.
5. Ticket (SRV/INF nach Lage) für den Wegfall von `Prio`/`Einschiebbar` in Vorlage und Planungstest, falls nicht in B-359; im Ergebnis nennen.
6. Tests: `plan_set PJx.y {"Status": "verworfen"}` angenommen und Session-Tabelle nachgezogen (B-338/AC-02); Sprint mit Sessions SIM, SRV, SIM → `Domäne: SIM, SRV`, Überschrift gleich; Domäne einer Session ändern → Sprint folgt; Reihenfolge: Sprint mit Projekt Rang 1 vor Sprint ohne Projekt mit Prio `hoch`; Sprint ohne Projekt behält Prio-Ableitung.
7. Prüfen, Ergebnis je Kriterium, `Status: fertig`, Commit `feat(srv): Domäne je Session und verworfen in den plan-Tools (PJ2.2)`.

## Fertig, wenn

- [ ] AC-04: Tests für `Domäne` je Session (lesen, schreiben), abgeleitete Sprint-Domänen samt Überschrift und Ordnung nach Rang statt Prio grün.
- [ ] AC-05: Test `plan_set` mit `Status: verworfen` an einer Session grün.
- [ ] `check_run dev:test` und `check_run task:check` grün.

## Prüfen

```bash
task check:dev
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
