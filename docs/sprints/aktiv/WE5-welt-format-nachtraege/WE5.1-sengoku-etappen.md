# WE5.1 · Sengoku: Hafen auf Insel 3 und Etappen 1, 2, 2 (B-979)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** welt
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/we5
- **Abhängig von:** –
- **Tickets:** B-979
- **Kriterien:** AC-01

## Ziel

Sengoku hat auf Insel 3 einen Hafen in Tiefe 0 und die Etappen 1, 2, 2; `world_check sengoku` ist fehlerfrei (B-979).

## Kontext

Welt-Datei `data/worlds/sengoku.json` (gitignored, existiert nur im Hauptcheckout; Kopie im Spielpaket `dist/k3c-game/data/worlds/` entsteht beim Build). Drei Inseln: island1 (Tiefen 0, 4), island2 (0, 1, 4), island3 (0, 3, 4, Bergfestung, ohne Hafen). Regel: jede Insel außer der einzigen der letzten Etappe braucht einen Hafen-Chunk in Tiefe 0 (`engine/sim/island_worlds.go` › `hasHarbor`, `soleOfLastEtappe`; `docs/rules/welten.md` § 6.1). Etappen: lückenlos ab 1, nicht fallend. Geändert wird nur über die MCP-Tools `world_get`, `world_chunks_set`, `world_set`, `world_check` der Workbench (muss laufen). Kein Worktree: die Datei liegt nur in der Repo-Wurzel.

## Erlaubte Dateien

- `data/worlds/sengoku.json`

## Nicht-Ziele

Code in Engine oder Workbench ändern (Fehler → Ticket), andere Welten, Spielpaket von Hand kopieren, Simulation.

## Schritte

1. `world_get sengoku`: geplante Chunks der Tiefe 0 von island1/island2 ansehen (Platz und Form des Hafen-Chunks).
2. `world_chunks_set` für island3, Tiefe 0: Hafen-Chunk wie auf island1/island2; lehnt die Prüfung ab, `chunkCounts` der Tiefe 0 um 1 erhöhen.
3. `world_set`: `etappe` 1, 2, 2 für island1, island2, island3.
4. `world_check sengoku`, Ergebnis ins Session-Ergebnis.

## Fertig, wenn

- [ ] AC-01: `world_get sengoku` zeigt Etappen 1, 2, 2 und den Hafen auf island3 (B-979/AC-01).
- [ ] AC-01: `world_check sengoku` ohne Fehler (B-979/AC-02).
- [ ] B-979/AC-03 (Sichtprüfung 🧑) ist an B-996 verwiesen, nicht Teil dieser Session.

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
