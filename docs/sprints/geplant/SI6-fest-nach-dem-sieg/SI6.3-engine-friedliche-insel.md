# SI6.3 · Friedliche Insel und Fest-Zustand in der Engine (B-973)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si6
- **Abhängig von:** SI6.2
- **Tickets:** B-973
- **Kriterien:** AC-03

## Ziel

Die friedliche Insel und der Fest-Zustand sind in der Engine umgesetzt und getestet; geänderte Golden-Läufe sind begründet (B-973/AC-02 bis AC-08).

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`)**; keine Simulation, kein Balancing.

Regel: `docs/rules/kampf.md` „Friedliche Insel“ (aus SI6.1), Spec `B-973`. Code: `Island.EndbossDefeated` (gesetzt beim Sieg über den Endboss, `engine/sim/boss_endboss.go`, `island.go`), Wechselpunkt `island_switch.go`, Kampagnensieg `island_victory.go` (`Island.Won`). Gegner-Quellen: Wellen `enemies_waves.go`, Portale `portal.go`, Aggressionspool unter Tage und Camps (`enemies*.go`, `enemies_guard.go`), Nacht-Events `events.go`, `events_moon.go`, `events_merchant.go` (Händler-Überfall; der Händler selbst `merchant.go` kommt weiter). Flucht der Gegner: vorhandenes Verhalten bei Tagesanbruch suchen (`unit_flee.go`, `enemies.go`) und wiederverwenden. Kronenraub: `crown.go`. Dev-Aktionen `dev_spawn.go` bleiben erlaubt. Der Frieden wird aus `EndbossDefeated` abgeleitet, kein neues Spielstand-Feld. Golden: gewollte Regeländerung; `task go:golden` nur hier, Begründung im Commit (`docs/arbeitsweise.md` › Golden aktualisieren). Neue Dateien mit Präfix `island_peace` oder `festival_`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/**`
- `testdata/golden/**`
- `docs/rules/kampf.md`
- `docs/rules/bosse.md`

## Nicht-Ziele

Darstellung und Musik (SI6.4, SI6.5), Belohnungen, Wert des Korridors „Schiff fertig“, Spielstand-Format, Simulationsläufe.

## Schritte

1. Heutiges Verhalten nach dem Sieg prüfen (kommen Wellen und Events weiter?) und im Ergebnis festhalten.
2. Eine Funktion „Insel friedlich“ (aus `EndbossDefeated`, bis zum Inselwechsel) und an allen Gegner-Quellen abfragen: Wellen, Portale, Aggressionspool, Camps, Vollmond, Blutmond, Händler-Überfall.
3. Beim Sieg: lebende Gegner aller Stufen fliehen und verschwinden, getragene Krone fällt, kein Schaden mehr.
4. Zustand `festival` und Ereignis `festivalStart` (genau einmal); nach `NextIsland` zurückgesetzt.
5. Tests je Kriterium: drei Tage und eine Vollmond-Nacht ohne Gegner, Flucht, Arbeit und Schiffsbau laufen weiter, Händler ohne Überfall, zwei Spieler auf verschiedenen Stufen, Speichern und Laden.
6. Golden prüfen; bei Änderung `task go:golden` und Begründung.
7. Reviewer-Agent `k3c-reviewer` vor dem Commit (Domäne SIM).

## Fertig, wenn

- [ ] AC-03: B-973/AC-02 bis AC-07 je durch einen Go-Test belegt (Testname im Ergebnis).
- [ ] B-973/AC-08: `check_run task:check:diff` grün, Änderungen unter `testdata/golden/` im Commit begründet.
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
