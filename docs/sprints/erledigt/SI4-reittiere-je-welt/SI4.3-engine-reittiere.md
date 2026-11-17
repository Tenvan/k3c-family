# SI4.3 · Reittier-Katalog, Gatter, Kauf und Spielstand in der Engine (B-970)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si4
- **Abhängig von:** SI4.2
- **Tickets:** B-970
- **Kriterien:** AC-03

## Ziel

Reittier-Katalog, Feld `mount` der Welt-Datei, Gatter, Kauf, Mitfahren und Spielstand sind in der Engine umgesetzt und getestet (B-970/AC-02 bis AC-09).

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`)**; keine Simulation, kein Balancing.

Regel: `docs/rules/monarch.md` § 7 (aus SI4.1), Spec `B-970`. Code: `engine/sim/monarch_mount.go` – `MountOf(w, p)` liefert heute `w.values().Monarch.Mount` für jeden Spieler (kein Feld in Player, Snapshot oder Spielstand); Daten `data/monarch.json` › `mount` (`id`, `sprite`, `speedFactor`, `sprintFactor`), Lader `monarch_data.go`. Welt-Datei laden und prüfen: `engine/sim/island_worlds.go`, Regeln `docs/rules/welten.md` § 2 und § 3. Platz ohne Generator anlegen: Muster `addShipSite` in `engine/sim/ship.go`; Münzeinwurf: `economy_pay.go`. Mitnahme beim Inselwechsel: `island_travel.go`, `island_switch.go` (`NextIsland`), Raum `engine/room/`. Spielstand: `engine/sim/island_save.go` (`IslandSaveVersion`), Ablauf `docs/arbeitsweise.md` › Spielstand-Format ändern; keine Rücksicht auf alte Spielstände. Hat SI1 (Fracht) die Version schon erhöht, baut diese Session darauf auf. Nur `engine/rng`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/**`
- `engine/room/**`
- `data/monarch.json`
- `data/worlds/sengoku.json`
- `data/worlds/nil.json`
- `docs/rules/welten.md`
- `docs/rules/monarch.md`

## Nicht-Ziele

Rentier und Schnee-Bremse, Sprint-Ausdauer, Level-Generator, Anzeige (SI4.4), Welten-Editor (SI7), Werte abstimmen.

## Schritte

1. `data/monarch.json`: `mounts` (`sengokuHorse` mit `sprintFactor` 1,2; `camel` mit `speedFactor` 1,1; `price` 12; `sprite` wie in SI4.2 zugeordnet) und `mountGate.minHubDistanceUnits` 60; Lader und Prüfung.
2. Welt-Datei: optionales Feld `mount`, Prüfung gegen den Katalog; `sengoku` und `nil` setzen; `welten.md` § 2 und § 3 ergänzen.
3. Gatter je Insel an der Oberfläche, Position deterministisch aus dem Seed, Mindestabstand zum Hub.
4. Kauf am Gatter je Monarch; Feld am Spieler; `MountOf` liefert den Katalog-Eintrag, sonst das Standard-Reittier.
5. Mitnahme bei Stufen- und Inselwechsel, kein Frachtplatz; später Beitritt mit Standard-Pferd.
6. Zustand je Spieler (Reittier-ID) und Gatter für die Anzeige.
7. Spielstand: Feld, `IslandSaveVersion` erhöhen, Fixtures und Tests.
8. Tests je Kriterium; Reviewer-Agent `k3c-reviewer` vor dem Commit (Domäne SIM, Spielstand).

## Fertig, wenn

- [ ] AC-03: B-970/AC-02 bis AC-08 je durch einen Go-Test belegt (Testname im Ergebnis).
- [ ] B-970/AC-09: `check_run task:check:diff` grün, `testdata/golden/` unverändert oder begründet.
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
