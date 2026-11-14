# tools/k3c-workbench/internal/planning/

## Responsibility

Liest und schreibt die Planung des Repos (`docs/backlog`, `docs/sprints`, `docs/projekte`, Fahrplan) als Markdown-Dateien: Lese-Sicht für die Oberfläche, Schreiboperationen hinter `plan_*` mit Vorlagenpflicht und Index-/Fahrplan-Synchronisation, dazu nächste Session, Arbeitswellen, Queue, Prüfung, Index-Verdichtung und Sprint-Zusammenfassungen.

## Design

- Domänenmodell `planning.go`: `Ticket`, `Sprint`, `Session`, `Data`; Parser `ParseSprint`/`ParseTicket`, `Load`, `Doc`, `Available`.
- Transaktionales Schreiben `store.go`: `changeSet` sammelt Writes (`read`/`write`/`apply`), `writeAtomic` erhält CRLF; `resolve`/`ref` löst IDs auf Pfade auf; `States` = geplant, aktiv, erledigt. `store_remove.go` (B-867): ein gescheitertes Entfernen wird byte-gleich zurückgenommen.
- Schreib-API: `Create` (`create.go`, aus `docs/vorlagen/*.md`, `NewDoc`; `similar.go` meldet ähnliche Titel), `Set`/`Section`/`Sections` (`edit.go`, Feldprüfung gegen Vorlage via `checkValue`; `Sections` prüft alle Abschnitte, dann ein Schreibvorgang), `Delete`, `Get`, `List` mit `Filter` (`list.go`).
- Ticket-Nummern: `ticketid.go` (B-863, Nummern ohne feste Länge, Sortierung über die Zahl), `ticketnr.go` (B-387): `nextTicket` nimmt die größte Nummer über Wurzel, alle Worktrees (samt Archiv) und lokale Refs `origin/main`, `origin/sprint/*` (`highestTicket`, `checkoutHighest`, `remoteHighest`).
- Relationen `relations.go` (B-696 A): „Duplikat von“ und „Ersetzt“ nur im neueren Ticket, Gegenrichtung berechnet `Get`. Index `index.go` (B-696 B): `RebuildIndex` schreibt `docs/backlog/README.md` (offen, eingeplant) und `docs/backlog/archiv/README.md`. `digest.go` (B-696 C): `Digest` erzeugt je erledigtem Sprint eine `ZUSAMMENFASSUNG.md`.
- Nächste Session `next.go` (B-518): `Next` mit `NextFilter` nach arbeitsweise.md › Autonomer Ablauf (Projekt-Rang → aktive Sprints → offen, autonom, Abhängigkeiten erledigt, keine Kollision); `next_lanes.go` prüft Kollisionen wie die Wellen (Bereich, Sperre), `next_origin.go` (B-519) überlagert Status aktiver Sprints aus dem lokalen Ref `origin/sprint/<präfix>`.
- Arbeitswellen `waves.go`, `waves_nodes.go`, `waves_spec.go` (B-783): `Waves` mit `WavesFilter` aus Ebenen der Abhängigkeiten, je Welle höchstens `maxSlots` (4), kein gleicher Bereich oder gleiche Sperre; Angaben aus „Regeln und Einschränkungen“ (Bereich, Abhängig von, Braucht 🧑, Größe, Sperre). `queue.go`: `Queue` listet die offenen Mensch-Sessions je Gerät. `check.go`: `Check` meldet Mängel eines Tickets oder Sprints.
- Konsistenz `tables.go`: Markdown-Tabellen (`mdTable`, `placeRow`, `removeRow`) für `syncIndex`, `syncRoadmap`, `syncSessionRow`; `roadmap_refresh.go` (B-842) schreibt nach jedem schreibenden Aufruf die übrigen Teile von `docs/sprints/README.md`; `roadmap_order.go` (B-366) wählt die Fahrplan-Zeile; `domains.go` leitet `Domäne` des Sprints aus den Sessions ab (B-356).
- Reihenfolge `priority.go`: `rank` liest je Session `Umgebung`, `Domäne`, `Abhängig von` und leitet `Sprint.Deps` ab; `Order` sortiert topologisch, dann nach Projekt-Rang und Platz in der Sprint-Tabelle.
- Projekte `projects.go` (B-357): `Project`, `ParseProject`, `createProject`, `setProject`, `rerank`, `deleteProject`, `checkProjectRef`; `projects_tables.go`: `syncOverview` (`docs/projekte/README.md`), `syncProjectSprint`.
- Worktrees: `worktree.go` (`Worktrees` über `git worktree list --porcelain`, `markWorktrees`), `mergebase.go` (`mergeBase`, `loadAt` entpackt `docs/` eines Commits), `wtdiff.go` (`CheckWorktrees` → `WorktreeDiff`/`Change`, nur lesend), `wtflight.go` (bündelt parallele `CheckWorktrees` einer Wurzel, Cache der Merge-Base-Stände `maxBases`), `gitrepo.go` (`inRepo` spart git-Aufrufe außerhalb eines Checkouts).
- `watch.go`: `Stamp` (Änderungs-Fingerabdruck) und `Watch` (Polling) für Live-Refresh.

## Flow

1. `Set(root, id, values)`: `resolve` → `changeSet.read` → `setFields` (Wert gegen Vorlage) → `follow` (Index-, Fahrplan- und Projektzeilen, ggf. Ordnerwechsel nach `States`) → `apply` → `refreshRoadmap`.
2. `Create`: `fromTemplate` kopiert die Vorlage, `nextTicket` vergibt die ID, `syncIndex`/`syncRoadmap` tragen die Zeile ein; ohne Projekt und Sprint lehnt es ein Ticket ab.
3. `Load(root)`: Projekte, Sprints aus `docs/sprints/{aktiv,geplant,erledigt}` und Tickets parsen, `rank`, Worktrees markieren.
4. `Next`/`Waves`/`Queue`: `Load` → Knoten bilden → Kollisionen prüfen → Text für das MCP-Tool.
5. `CheckWorktrees`: über `wtflight` je Worktree `mergeBase` → `loadAt` → `diffData`. `Watch` ruft `onChange`, sobald sich `Stamp` ändert.

## Integration

- Konsumenten: MCP-Tools `plan_list`, `plan_get`, `plan_next`, `plan_create`, `plan_set`, `plan_section`, `plan_sections`, `plan_delete`, `plan_check`, `plan_queue`, `plan_waves`, `plan_index`, `plan_digest` (`internal/mcpsrv/tools_planning.go`, `tools_planning_waves.go`, `tools_planning_index.go`, `tools_planning_digest.go`); Bindings in `app_planning.go` (`PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningGet`, `PlanningSet`, `WorktreeCheck`), `app.go` (Wächter `planning:changed`).
- Abhängigkeiten: `git` über `internal/proc` (`runGit`, `gitOut`), Dateisystem unter `docs/`; Vorlagen `docs/vorlagen/*.md` (geprüft von `tests/planning.test.ts`).
