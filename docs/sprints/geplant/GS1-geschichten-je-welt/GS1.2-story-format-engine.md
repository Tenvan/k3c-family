# GS1.2 · Engine: Feld story, Prüfung und Wendepunkt-Funktionen (B-965)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs1
- **Abhängig von:** GS1.1
- **Tickets:** B-965
- **Kriterien:** AC-02, AC-03

## Ziel

Die Engine liest und prüft `story` in Welt-Dateien und nennt zu jedem Wendepunkt die Zeilen über reine Funktionen; Golden und Spielstand bleiben unverändert.

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`):** `task k3c-workbench:dev:worktree`, MCP-Aufrufe mit `checkout`. Keine Simulation, kein Balancing.

Spec: `docs/backlog/B-965-geschichte-kunstsprache.md` (R1–R7, AC-02 bis AC-06), Regeltext aus GS1.1 (`docs/rules/welten.md` § 2, § 3, § 6.2). Code: `engine/sim/island_worlds.go` (`WorldDef`, strikter Decoder, `idPattern`), `engine/sim/island_data.go` (`IslandDef`, `checkIsland`), `engine/sim/island_targets.go` (`etappeOf`, `done`, `targetIndexes` – für „Etappe abgeschlossen“ und „letzte Insel“), `engine/sim/island_victory.go`, Tests `island_worlds_test.go`, `island_worlds_etappe_test.go`. Daten sind eingebettet über `data/embed.go` (`*.json`). Ereignisse der Wendepunkte gibt es schon: `bossSpawned`, `bossDefeated`, `islandSwitch`; es entstehen **keine neuen Ereignisse** und kein neuer Zustand. Fallstrick: Unbekannte Felder werden abgelehnt, also müssen `story` und seine Unterfelder dem Decoder bekannt sein; Kunstwörter können Apostroph, Bindestrich und Sonderzeichen enthalten (`akh'et`, `sen-hotep`, `hjørn`). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/**`
- `data/story.json`

## Nicht-Ziele

Neue Ereignisse, Änderung am Spielstand, Anzeige (GS1.3), Werkzeuge (GS1.4), Texte in Welt-Dateien (GS1.5), Mechanik der Flaschenpost (GS4; hier nur Feld `bottles` lesen und prüfen).

## Schritte

1. Tests zuerst: Laden mit und ohne `story`, Tabellentest der Prüfungen (B-965/R4), Wendepunkte für Etappen 1, 2, 2.
2. `data/story.json` mit Obergrenzen (Zeilenlänge 64, Wortschatz 24, vorläufig) und Lader.
3. Typen `WorldStory`, `IslandStory` in neuer Datei `engine/sim/story.go`; Felder an `WorldDef` und `IslandDef`, Decoder erweitern.
4. Prüfung in `engine/sim/story_check.go` (Meldung mit Welt, Insel, Feld, Zeile).
5. Wendepunkt-Funktionen (Start, Ankunft, Endboss ausgelöst, Endboss besiegt mit Etappe bzw. Ende) ohne Zustandsänderung; exportiert für das Spiel.
6. Test: Spielstand mit und ohne Geschichte byte-gleich; `check_run task:check:go` im Worktree.
7. Review-Agent (Domäne SIM) vor dem Commit.

## Fertig, wenn

- [ ] AC-02: Welt mit `story` lädt, ohne `story` unverändert; jede Prüfung lehnt mit Welt, Feld und Zeile ab; Obergrenzen aus `data/story.json` (B-965/AC-02, AC-03, AC-06).
- [ ] AC-03: Wendepunkt-Funktionen liefern die Zeilen aus B-965 › Beispiele, deterministisch, ohne Zustand; Spielstand byte-gleich, Golden unverändert (B-965/AC-04, AC-05).
- [ ] `check_run task:check:diff` grün; Dev-Workbench des Worktrees gestoppt.

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
