# tools/k3c-workbench/internal/planning/

## Responsibility

Liest und schreibt die Planung des Repos (`docs/`: Tickets, Sprints, Sessions, Roadmap) als Markdown-Dateien. Stellt die Lese-Sicht für die Oberfläche und die Schreiboperationen hinter `plan_*` bereit, inklusive Vorlagenpflicht und Index-/Roadmap-Synchronisation.

## Design

- Domänenmodell `planning.go`: `Ticket`, `Sprint`, `Session`, `Data`; Parser `ParseSprint`/`ParseTicket`, `Load`, `Doc`, `Available`.
- Transaktionales Schreiben `store.go`: `changeSet` sammelt Writes (`read`/`write`/`apply`), `writeAtomic` erhält CRLF; `resolve`/`ref` löst IDs auf Pfade auf; `States` = geplant, aktiv, erledigt.
- Schreib-API: `Create` (`create.go`, aus `docs/vorlagen/*.md`, `NewDoc`), `Set`/`Section`/`Sections` (`edit.go`, Feldprüfung gegen Vorlage via `checkValue`; `Sections` prüft alle Abschnitte, dann ein Schreibvorgang), `Delete`, `Get`, `List` mit `Filter` (`list.go`).
- Nächste Session `next.go` (B-518): `Next` mit `NextFilter` wählt nach arbeitsweise.md › Autonomer Ablauf, Schritt 1 (aktive Projekte nach Rang → aktive Sprints der Projekt-Tabelle → offen, autonom, Abhängigkeiten aus Sessions und Tickets erledigt, Domäne nicht durch eine Session `in Arbeit` gesperrt) und liefert Session-Datei, Kriterien der Sprint-README und parallel mögliche Sessions anderer Domänen.
- Konsistenz `tables.go`: Markdown-Tabellen (`mdTable`, `placeRow`, `removeRow`) für `syncIndex` (Backlog-README), `syncRoadmap`, `syncSessionRow`.
- Reihenfolge `priority.go`: `rank` liest je Session `Umgebung`, `Domäne` und `Abhängig von` (`sessionMeta`) und leitet daraus `Sprint.Deps` ab; `Order` sortiert generisch topologisch nach Abhängigkeit, dann `sprintOrder` (Projekt-Rang und Platz in der Sprint-Tabelle, Sprints ohne Projekt dahinter; eine Voraussetzung erbt den Platz ihrer Abnehmer, Zyklus wird gebrochen); `sprintOf`. Sprints haben keine Prio (DV1).
- Projekte `projects.go` (B-357): `Project`, `ParseProject`, `loadProjects` (`Data.Projects`), `createProject`, `setProject` (Status, Rang, Sprints), `rerank`/`rankOrder` (Ränge der aktiven Projekte lückenlos, `ABN` ohne Rang), `deleteProject`, `checkProjectRef` (Feld `Projekt` an Sprint und Ticket). `projects_tables.go`: `syncOverview` (`docs/projekte/README.md` aus den Projekt-Dateien), `reorderSprints`, `syncProjectSprint` (Sprint-Tabelle des Projekts folgt `Projekt` und Status des Sprints).
- Projekt-Reihenfolge im Fahrplan `roadmap_order.go` (B-366): `roadmapKey` (Projekt-Platz, Sprint-Platz), `roadmapPlace`/`roadmapInsertAt` wählen die Zeile für einen Sprint. `domains.go`: `syncSprintDomain` leitet `Domäne`, Überschrift und Fahrplan-Spalte aus den Sessions ab (B-356).
- Ticket-Nummern `ticketnr.go` (B-387): `nextTicket` nimmt die größte Nummer über Wurzel, alle Worktrees (samt Archiv) und lokale Refs `origin/develop`, `origin/sprint/*` (`highestTicket`, `checkoutHighest`, `remoteHighest`, `samePath` per `os.SameFile`).
- Worktree-Vergleich `mergebase.go` (`runGit`, `mergeBase`, `loadAt` entpackt `docs/` eines Commits per tar, `loadWithArchive`) und `wtdiff.go` (`CheckWorktrees` -> `WorktreeDiff`/`Change`: was ein Worktree an Projekten, Sprints, Sessions und Tickets gegenüber der Merge-Base geändert hat, nur lesend).
- `watch.go`: `Stamp` (Änderungs-Fingerprint) und `Watch` (Polling) für Live-Refresh; `worktree.go`: `Worktrees` (git worktree list --porcelain) und `markWorktrees`.

## Flow

1. `Set(root, id, values)`: `resolve` -> `changeSet.read` -> `setFields` (Wert gegen Vorlage prüfen) -> `follow` (Index-/Roadmap-Zeilen, ggf. Verzeichniswechsel nach `States`) -> `apply`.
2. `Create`: `fromTemplate` kopiert die Vorlage, `nextTicket` vergibt die ID, `syncIndex`/`syncRoadmap` tragen die Zeile ein.
3. `Load(root)`: Sprints aus `docs/sprints/{aktiv,geplant,erledigt}` und Tickets parsen, `rank` (Prio, Deps, Reihenfolge offener Sprints), Worktrees markieren.
4. `CheckWorktrees`: je Worktree `mergeBase` -> `loadAt` -> `diffData` gegen die Planung des Worktrees.
5. `Watch` ruft `onChange`, sobald sich `Stamp` ändert.

## Integration

- Konsumenten: MCP-Tools `plan_list`, `plan_get`, `plan_next`, `plan_create`, `plan_set`, `plan_section`, `plan_sections`, `plan_delete` (`internal/mcpsrv/tools_planning.go`); Wails-Bindings `PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningSet` (`app_planning.go`), `app.go`.
- Abhängigkeiten: `git` über `os/exec` (`runGit`, `gitOut`) und `internal/proc`, Dateisystem unter `docs/`; Vorlagen `docs/vorlagen/*.md` (geprüft von `tests/planning.test.ts`).
