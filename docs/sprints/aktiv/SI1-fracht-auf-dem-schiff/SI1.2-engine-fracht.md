# SI1.2 · Laderaum, Ladeplätze und Fracht in der Engine (B-967)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si1
- **Abhängig von:** SI1.1
- **Tickets:** B-967
- **Kriterien:** AC-02

## Ziel

Laderaum, Ladeplätze, Laden, Ankunft der Fracht und Spielstand sind in der Engine umgesetzt und getestet (B-967/AC-02 bis AC-09).

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`)**; die Wurzel-Workbench bleibt unberührt, keine Simulation, kein Balancing.

Regel: `docs/rules/stufen.md` § 1, `docs/rules/wirtschaft.md` § 8 (aus SI1.1), Spec `B-967`. Code: Schiff als Platz `ship` des Hafens in `engine/sim/ship.go` (`withShip`, `addShipSite`, `harbor`, `shipComplete`, `ShipNextCost`; Kiel = Bau, Rumpf und Segel = Stufe 2 und 3), Daten `data/ship.json` (`stages`). Inselwechsel in `engine/sim/island_switch.go` (`stepIslandSwitch`, `NextIsland`), Mitnahme in `engine/sim/island_travel.go`, Vorrat in `island_storage.go`, Startvorrat `data/hub.json` › `islandStartStock`, Start-Truppen `startTroops`. Münzeinwurf an Plätzen: `economy_pay.go`. Spielstand: `engine/sim/island_save.go` (`IslandSaveVersion` 9), ältere Lader `island_save_v*.go`; Ablauf `docs/arbeitsweise.md` › Spielstand-Format ändern; keine Rücksicht auf alte Spielstände. Der Raum tauscht die Insel in `engine/room/`. Nur `engine/rng`, nie `math/rand`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; neue Dateien mit Präfix `ship_` (z. B. `ship_cargo.go`).

## Erlaubte Dateien

- `engine/sim/**`
- `engine/room/**`
- `data/ship.json`
- `docs/rules/stufen.md`
- `docs/rules/wirtschaft.md`

## Nicht-Ziele

Anzeige (SI1.4), Assets (SI1.3), Entladen, Koop-Multiplikator für Plätze, Werte abstimmen, Simulationsläufe.

## Schritte

1. Heutige Mitnahme beim Inselwechsel prüfen (Monarchen-Münzen, `island_travel.go`) und im Ergebnis festhalten.
2. `data/ship.json`: `cargo` (`slots` 6, `holdSlots` 4, `purseCoins` 20, `crateUnits` 50) und Abschnitt `hold` (`optional: true`, 25 Münzen, 60 Holz, 40 Stein, 16 s); Lader und Prüfung, Abfahrt verlangt `hold` nicht.
3. Fracht-Zustand an der Insel (Plätze mit Art und Inhalt, offene Träger-Aufträge) und die drei Ladeplätze am Steg, nutzbar ab fertigem Segel.
4. Laden: Münzen, Truppe (nächststehende, keine Bürger), Material (größter Vorrat, Bauer trägt, Material reserviert); Ablehnen mit Ereignis `cargoFull`; feste Reihenfolge nach Spieler-Index.
5. Verlust des Abschnitts `hold`: überzählige Fracht in umgekehrter Reihenfolge zurückgeben.
6. `NextIsland`: Material in den Vorrat, Truppen an den Hafen, Beutel als Münzen am Steg; offene Aufträge verfallen.
7. Zustand und Ereignisse für die Anzeige (Plätze, Belegung, Ladeplätze).
8. Spielstand: Felder, `IslandSaveVersion` erhöhen, Fixtures und Tests.
9. Tests je Kriterium; weicht die Umsetzung von der Regel ab, Regeltext nachziehen.
10. Reviewer-Agent `k3c-welten` vor dem Commit (Domäne SIM, Spielstand).

## Fertig, wenn

- [ ] AC-02: B-967/AC-02 bis AC-08 je durch einen Go-Test belegt (Testname im Ergebnis).
- [ ] B-967/AC-09: `check_run task:check:diff` grün, `testdata/golden/` unverändert oder begründet.
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
