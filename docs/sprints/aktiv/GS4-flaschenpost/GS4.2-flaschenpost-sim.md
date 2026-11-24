# GS4.2 · Engine: Flaschenpost, Ziel, Markierung, Spielstand (B-971)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/gs4
- **Abhängig von:** GS4.1
- **Tickets:** B-971
- **Kriterien:** AC-02, AC-03, AC-04

## Ziel

Die Simulation legt Flaschen deterministisch an den Strand, lässt sie aufheben oder von der Flut holen, wählt ein Ziel, führt eine Markierung je Insel und trägt alles im Spielstand.

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`):** `task k3c-workbench:dev:worktree`, MCP-Aufrufe mit `checkout`. Keine Simulation, kein Balancing; `task go:golden` ist hier erlaubt (gewollte Änderung, Begründung in den Commit).

Spec: `docs/backlog/B-971-flaschenpost.md` (R1–R9, AC-02 bis AC-09), Regeltext aus GS4.1 (`docs/rules/wirtschaft.md` › Flaschenpost). Code: Tageszyklus und `dawn`/`dusk` in `engine/sim/world_cycle.go`; Truhen und Aufheben in `engine/sim/economy.go` (`pk.Kind == "chest"`, Ereignis `chest`); Insel und Stufen in `engine/sim/island.go` (Insel-Schritt über alle Stufen), Inselwechsel `island_switch.go`; Spielstand `island_save.go` (`IslandSaveVersion`, nach GS3 um eins höher); Zustand für das Spiel `World.State`; Welt-Geschichte `engine/sim/story.go` (`story.bottles`, GS1). Zufall nur `engine/rng` (`rng.New(seed)`), **eigener Strom** aus Insel-Seed und Tag, damit Level- und Wellen-Zufall unverändert bleiben. Daten neu: `data/bottle.json` (eingebettet über `data/embed.go` › `*.json`). Mehrere Spieler im selben Takt: höchster Spieler-Index gewinnt (wie `applyChoices` in `island_targets.go`). Zielarten erweiterbar (heute nur Truhe). Sperren `golden` und Spielstand-Format. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/**`
- `data/bottle.json`
- `testdata/golden/**`

## Nicht-Ziele

Darstellung (GS4.4), Asset (GS4.3), Einsiedler und Gatter als Ziele, Balancing der Werte, Overrides je Welt.

## Schritte

1. Tests zuerst (neue Datei `engine/sim/bottle_test.go`): Erscheinen bei `dawn`, Seite deterministisch, Aufheben, Gleichstand, Flut, Ziel in anderer Stufe, Ende der Markierung, keine Flasche ohne Ziel oder während Markierung, Insel ohne Tiefe 0, Zeilen reihum, Spielstand, Inselwechsel, übrige Zufallsfolgen unverändert.
2. `data/bottle.json` und Lader.
3. Zustand an der Insel (Flasche, Markierung, letzter Tag, Zähler) in `engine/sim/bottle.go`; Schritt im Insel-Takt.
4. Ereignisse `bottleFound` (Spieler, Zeilen), `bottleLost`, `bottleTargetReached`; Zustand je Stufe für das Spiel.
5. Spielstand: Felder, Version + 1, Fixture.
6. `task go:golden`, Diff des Golden prüfen (nur Flaschen-Ereignisse und -Zustand), `check_run task:check:go` im Worktree.
7. Review-Agent (SIM, Spielstand) vor dem Commit; `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-02: Erscheinen, Aufheben, Flut, Insel ohne Tiefe 0, übrige Zufallsfolgen gleich (B-971/AC-02, AC-03, AC-08).
- [ ] AC-03: Ziel, Markierung, Zustand je Stufe, Zeilen reihum (B-971/AC-04 bis AC-06).
- [ ] AC-04: Spielstand, Version, Fixture, Golden gewollt neu, `check_run task:check:go` grün (B-971/AC-07, AC-09).
- [ ] Dev-Workbench des Worktrees gestoppt.

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
