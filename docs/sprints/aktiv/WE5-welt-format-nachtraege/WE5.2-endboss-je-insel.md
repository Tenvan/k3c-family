# WE5.2 · Endboss je Insel wählbar, Bau in der tiefsten Stufe (B-981)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we5
- **Abhängig von:** –
- **Tickets:** B-981
- **Kriterien:** AC-02, AC-08

## Ziel

Eine Insel wählt ihren Endboss (`endBoss`), der Bau liegt in ihrer tiefsten Stufe in jedem Biom; Regeln und Tests liegen vor (B-981).

## Kontext

Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Heute: `engine/sim/island_data.go` › `checkIsland` verlangt `endBoss(deepest)`; `engine/sim/boss_data.go` lädt `data/bosses.json` (`bosses` = Minibosse je Tiefe, `endBosses` = nur `crystalHeartWarden`, Tiefe 4); Endboss-Logik `engine/sim/boss_endboss.go`, Eskorte `unit_escort.go`. Der Bau (`lair`: `triggerUnits`, `noticeUnits`, `escortUnits`) steht nur in `data/biomes/crystal.json` und wird vom Level-Generator gelesen (`engine/level/biome.go`, `level.go` › `safeCampSlots`, `sites.go`, `validate.go`). Welt-Datei streng gelesen in `engine/sim/island_worlds.go` (`rawIsland.def`). Regeln: `docs/rules/bosse.md` § 1, `welten.md` § 2 und § 3, `stufen.md`. `engine/level` importiert nichts aus `engine/sim`. Nur `engine/rng`. Golden (`testdata/golden/`) dürfen sich nicht ändern: Die Standard-Welt endet in Tiefe 4.

## Erlaubte Dateien

- `engine/sim/**`
- `engine/level/**`
- `data/bosses.json`
- `data/biomes/**`
- `docs/rules/welten.md`
- `docs/rules/bosse.md`
- `docs/rules/stufen.md`

## Nicht-Ziele

Spiel (WE6.1), Editor und MCP (WE6.3), Boss-Grafik, neue Fähigkeiten, Balancing, Simulation, `task go:golden`.

## Schritte

1. Tests zuerst: Insel mit tiefster Stufe 0–3 lädt; `endBoss` wählt eine ID; Fehlerfälle (unbekannt, Miniboss).
2. `data/bosses.json`: Endbosse für Tiefen 0–3 (Basisart der Stufe, Faustregel aus `bosse.md` § 1, nur vorhandene Fähigkeiten, im `notes`-Feld als vorläufig markiert).
3. Insel-Feld `endBoss` lesen und prüfen; Auswahl: Feld, sonst Endboss der tiefsten Stufe.
4. Bau vom Biom lösen: gemeinsame Bau-Werte, der Level-Generator bekommt je Stufe übergeben, ob sie den Bau trägt; Camp-Abstand und Längen-Prüfung für jedes Biom, Test über 500 Seeds je Biom.
5. Regeln nachziehen; Review durch `k3c-welten` und `k3c-reviewer` vor dem Commit.

## Fertig, wenn

- [ ] AC-02: B-981/AC-01 bis AC-03 als Tests grün.
- [ ] AC-02: B-981/AC-05 Regeln nachgezogen.
- [ ] AC-08: `testdata/golden/` unverändert (B-981/AC-04).
- [ ] Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

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
