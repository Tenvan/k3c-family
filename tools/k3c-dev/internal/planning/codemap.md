# tools/k3c-dev/internal/planning/

## Responsibility

Liest und schreibt die Planung des Repos (`docs/`: Tickets, Sprints, Sessions, Roadmap) als Markdown-Dateien. Stellt die Lese-Sicht für die Oberfläche und die Schreiboperationen hinter `plan_*` bereit, inklusive Vorlagenpflicht und Index-/Roadmap-Synchronisation.

## Design

- Domänenmodell `planning.go`: `Ticket`, `Sprint`, `Session`, `Data`; Parser `ParseSprint`/`ParseTicket`, `Load`, `Doc`, `Available`.
- Transaktionales Schreiben `store.go`: `changeSet` sammelt Writes (`read`/`write`/`apply`), `writeAtomic` erhält CRLF; `resolve`/`ref` löst IDs auf Pfade auf; `States` = geplant, aktiv, erledigt.
- Schreib-API: `Create` (`create.go`, aus `docs/vorlagen/*.md`, `NewDoc`), `Set`/`Section` (`edit.go`, Feldprüfung gegen Vorlage via `checkValue`), `Delete`, `Get`, `List` mit `Filter` (`list.go`).
- Konsistenz `tables.go`: Markdown-Tabellen (`mdTable`, `placeRow`, `removeRow`) für `syncIndex` (Backlog-README), `syncRoadmap`, `syncSessionRow`.
- Priorität/Reihenfolge `priority.go`: `rank` setzt je Sprint die höchste Prio seiner Tickets, liest je Session `Umgebung` und `Abhängig von` (`sessionMeta`) und leitet daraus `Sprint.Deps` ab; `Order` sortiert generisch topologisch nach Abhängigkeit, dann Prio (Voraussetzung erbt die Prio ihrer Abnehmer, Zyklus wird gebrochen); `maxPrio`, `sprintOf`. In `list.go`: `SprintPrio`, `sortByPrio`, `prioRank`.
- `watch.go`: `Stamp` (Änderungs-Fingerprint) und `Watch` (Polling) für Live-Refresh; `worktree.go`: `Worktrees` (git worktree list --porcelain) und `markWorktrees`.

## Flow

1. `Set(root, id, values)`: `resolve` -> `changeSet.read` -> `setFields` (Wert gegen Vorlage prüfen) -> `follow` (Index-/Roadmap-Zeilen, ggf. Verzeichniswechsel nach `States`) -> `apply`.
2. `Create`: `fromTemplate` kopiert die Vorlage, `nextTicket` vergibt die ID, `syncIndex`/`syncRoadmap` tragen die Zeile ein.
3. `Load(root)`: Sprints aus `docs/sprints/{aktiv,geplant,erledigt}` und Tickets parsen, `rank` (Prio, Deps, Reihenfolge offener Sprints), Worktrees markieren.
4. `Watch` ruft `onChange`, sobald sich `Stamp` ändert.

## Integration

- Konsumenten: MCP-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`, `plan_delete` (`internal/mcpsrv/tools_planning.go`); Wails-Bindings `PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningSet` (`app_planning.go`), `app.go`.
- Abhängigkeiten: `internal/proc` (git), Dateisystem unter `docs/`; Vorlagen `docs/vorlagen/*.md` (geprüft von `tests/planning.test.ts`).
