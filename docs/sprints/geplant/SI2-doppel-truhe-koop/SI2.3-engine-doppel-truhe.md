# SI2.3 · Doppel-Truhe in der Engine (B-968)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si2
- **Abhängig von:** SI2.2
- **Tickets:** B-968
- **Kriterien:** AC-03

## Ziel

Die Doppel-Truhe ist in der Engine umgesetzt und getestet; Solo-Läufe bleiben unverändert (B-968/AC-02 bis AC-08).

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`)**; keine Simulation, kein Balancing.

Regel: `docs/rules/koop.md` (aus SI2.1), Spec `B-968`. Code: Truhen sind Pickups der Art `chest`; das Öffnen steht in `engine/sim/economy.go` (Zweig `pk.Kind == "chest"`: Gold aus `Economy.ChestGold`, Ereignis `chest`, `chestSkillPoint`), die Fontäne in `engine/sim/economy_fountain.go` (`fountainCoins`, `scaledGold`). Truhen erzeugt der Level-Generator (`engine/level/level.go`, `eventKinds`); er bleibt unverändert. Das Kennzeichen „doppelt“ wird aus Insel-Seed, Stufe und Position berechnet (`engine/rng`, nie `math/rand`) und nicht gespeichert. „Gesteuert“ meint Monarchen mit Spieler am Gerät (vorhandenen Begriff im Code suchen, z. B. in `island_switch.go` „lebende, gesteuerte Spieler“). Braucht die Umsetzung doch `engine/level/**` oder ein Spielstand-Feld: anhalten und melden.

## Erlaubte Dateien

- `engine/sim/**`
- `data/economy.json`
- `docs/rules/koop.md`

## Nicht-Ziele

Level-Generator, Spielstand-Format, Anzeige (SI2.4), weitere Koop-Aktionen, Werte abstimmen.

## Schritte

1. `data/economy.json` › `doubleChest` (`everyNth` 3, `goldFactor` 3), Lader und Prüfung.
2. Kennzeichen-Funktion: deterministisch aus Seed, Stufe, Position; Test auf Stabilität.
3. Öffnen: wirkt das Kennzeichen (zwei gesteuerte Monarchen), nur wenn zwei Monarchen derselben Stufe im Aufheb-Radius stehen; Gold × Faktor über die Fontäne; Zusatz der normalen Truhe für beide.
4. Zustand: wartende Doppel-Truhe mit Index des wartenden Monarchen; Kennzeichen sichtbar für die Anzeige.
5. Tests je Kriterium (Solo unverändert, ein Monarch, beide, Spieler geht und kommt, Determinismus).
6. Reviewer-Agent `k3c-reviewer` vor dem Commit (Domäne SIM).

## Fertig, wenn

- [ ] AC-03: B-968/AC-02 bis AC-07 je durch einen Go-Test belegt (Testname im Ergebnis); `testdata/golden/` unverändert.
- [ ] B-968/AC-08: `check_run task:check:diff` grün.
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

–
