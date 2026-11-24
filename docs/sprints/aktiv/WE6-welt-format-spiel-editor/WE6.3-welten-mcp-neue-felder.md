# WE6.3 · Welten-MCP: endBoss, skins, events und hub › sites setzen und lesen (B-987)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we6
- **Abhängig von:** –
- **Tickets:** B-987
- **Kriterien:** AC-03

## Ziel

Die Welten-MCP-Tools setzen und lesen `endBoss`, `skins`, `overrides › events` und `hub › sites`; kein Feld geht beim Schreiben verloren (B-987, Go-Anteil).

## Kontext

Welten-MCP `tools/k3c-workbench/internal/mcpsrv/tools_world.go`, Logik `tools/k3c-workbench/internal/world/` (`world.go`, `edit.go`, `edit_island.go`, `patch.go`, `override.go`, `values.go`). Neue Felder aus WE5: Insel-Feld `endBoss` (B-981), Welt-Feld `skins` (B-984), Override-Datei `events` (B-982), entsperrt `hub.sites`/`hub.islandSites` (B-983); Format in `docs/rules/welten.md` § 1 und § 2. WE5.6 hat `world_set` um `plannedChunks` erweitert. Die Workbench rechnet die Engine im Prozess; Prüfung über `op: check`.

## Erlaubte Dateien

- `tools/k3c-studio/internal/world/**`
- `tools/k3c-studio/internal/mcpsrv/**`

## Nicht-Ziele

Frontend (WE6.4), Engine ändern, Assets anlegen.

## Schritte

1. Test zuerst: Rundlauf einer Welt mit allen neuen Feldern durch `world_set`, `world_override_set`, `world_chunks_set` verliert nichts.
2. `world_set`: `endBoss` (ID oder `null`), `skins` (Schlüssel setzen, `null` entfernt); `world_get` zeigt beides.
3. `world_override_set` für `events` und `hub › sites` (Listen-Merge wie die Engine); Tool-Beschreibungen nachziehen.
4. Review `k3c-workbench` vor dem Commit.

## Fertig, wenn

- [ ] AC-03: B-987/AC-01, AC-02 als Go-Tests grün.
- [ ] `check_run task:check:dev` grün.

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

AC-03 teilweise: `endBoss` (Insel) und `skins` (Welt) lassen sich über world_set setzen und bleiben erhalten, Go-Tests in `tools/k3c-studio/internal/world` grün (`edit_endboss_skins_test.go`). Offen: `overrides › events` und `hub › sites` über world_override_set; der Override-Pfad des Studios schreibt nur Blätter der Werte-Ansicht und kennt keinen Listen-Merge. Eigenes Ticket anlegen oder Session ergänzen. Erlaubte Dateien auf die Studio-Pfade umgestellt (Ursprung: Workbench-Pfade, dort gibt es kein Welten-Paket).
