# WE5.6 · Welten-MCP: Hafen im selben world_set, Ablehnung nennt erlaubte Dateien (B-985, B-986)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we5
- **Abhängig von:** –
- **Tickets:** B-985, B-986
- **Kriterien:** AC-06, AC-07

## Ziel

`world_set` setzt geplante Chunks (Hafen) und hängt Inseln im selben Aufruf an; Ablehnungen von `world_override_set` nennen erlaubte Dateien und gesperrte Felder (B-985, B-986).

## Kontext

Welten-MCP: `tools/k3c-workbench/internal/mcpsrv/tools_world.go` (Tools `world_set`, `world_chunks_set`, `world_override_set`, Beschreibungen und Schema), Logik in `tools/k3c-workbench/internal/world/` (`edit.go`, `edit_island.go`, `patch.go`, `override.go`, `values.go`). Die Workbench prüft über die eingebundene Engine (Aufruf `op: check`). Hafen-Regel: `engine/sim/island_worlds.go`. Erlaubte Override-Dateien und gesperrte Felder: `engine/sim/world_values.go`, `world_values_locked.go` (nur lesen, nicht ändern). Läuft parallel zu den SIM-Sessions (Lane `mcp`); sie ändern `engine/**`, diese Session nicht.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/world/**`
- `tools/k3c-workbench/internal/mcpsrv/tools_world.go`
- `tools/k3c-workbench/internal/mcpsrv/tools_world_test.go`

## Nicht-Ziele

Engine ändern, Oberfläche des Editors, neue Felder aus B-981 bis B-984 (WE6.3), Neubau der Workbench-EXE.

## Schritte

1. Tests zuerst (B-985): ein Patch setzt `plannedChunks` einer Insel und hängt eine weitere an; fehlender Hafen → Ablehnung mit Hinweis, Datei unverändert.
2. Insel-Eintrag um `plannedChunks`/`chunkCounts` erweitern, Prüfung erst nach dem ganzen Patch; Tool-Beschreibung mit Beispiel.
3. Tests zuerst (B-986): unbekannte Datei, gesperrtes Feld, unbekanntes Feld.
4. Meldungen ergänzen, Liste aus der Engine lesen; Tool-Beschreibung von `world_override_set`.
5. Review `k3c-workbench` bei mehr als etwa 150 Zeilen.

## Fertig, wenn

- [ ] AC-06: B-985/AC-01, AC-02 als Tests grün.
- [ ] AC-07: B-986/AC-01, AC-02 als Tests grün.
- [ ] `check_run task:check:dev` grün (B-985/AC-03, B-986/AC-03).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-06 geprüft: `TestSetAppendIslandWithHarborInOneCall`, `TestSetAppendWithoutHarborNamesIslandAndField`. AC-07 geprüft: `TestSetOverrideUnknownFileListsAllowedFiles`, `TestSetOverrideLockedAndUnknownFieldMessages`, `TestSetOverrideDeepPathUnderLockedField`, `TestOverrideFilesTextMatchesEngine`. Go-Tests, Lint 0, `task check:diff` grün; Review k3c-workbench, Befunde behoben. Commit 7c497ee35. Der Hafen im selben Aufruf ging schon, es fehlten Hinweis und Tests.
