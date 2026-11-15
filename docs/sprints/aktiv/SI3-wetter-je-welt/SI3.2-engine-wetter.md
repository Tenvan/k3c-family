# SI3.2 · Wetter-Katalog, Zeitplan, Taifun und Flut in der Engine (B-969)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si3
- **Abhängig von:** SI3.1
- **Tickets:** B-969
- **Kriterien:** AC-02

## Ziel

Wetter-Katalog, Feld `weather` der Welt-Datei, Zeitplan, Taifun und Flut sind in der Engine umgesetzt und getestet (B-969/AC-02 bis AC-09).

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`)**; keine Simulation, kein Balancing.

Regel: `docs/rules/wetter.md` (aus SI3.1), Spec `B-969`. Code: Nacht-Events als Muster in `engine/sim/events.go`, `events_moon.go` (jede n-te Nacht aus `Cycle.Day`), Tag und Nacht in `engine/sim/world_cycle.go`. Welt-Datei laden und prüfen: `engine/sim/island_worlds.go` (`rawIsland`, Prüfungen), Regeln `docs/rules/welten.md` § 2 Format und § 3 Prüfung. Daten einbinden: `data/embed.go` (von der Platte aus `K3C_DATA_DIR`, sonst `go:embed`), Lader in `engine/sim/data.go`. Tempo der Einheiten: `monarch.go`, `unit_common.go`, `enemies.go`; Reichweite der Schützen: `unit_archer.go`, `combat.go`; Portale: `portal.go`. Burg-Radius: `data/hub.json` › `castleRadiusUnits`. Der Zeitplan wird aus Insel-Seed und Tag berechnet (`engine/rng`, nie `math/rand`), nichts im Spielstand; braucht es doch ein Feld: anhalten und melden. Neue Dateien mit Präfix `weather_`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/**`
- `data/weather.json`
- `data/embed.go`
- `data/worlds/sengoku.json`
- `data/worlds/nil.json`
- `docs/rules/welten.md`
- `docs/rules/wetter.md`

## Nicht-Ziele

Schneesturm, Darstellung (SI3.4), Welten-Editor (SI7), Spielstand-Format, echte Abdrift der Pfeil-Flugbahn, Werte abstimmen.

## Schritte

1. `data/weather.json` (Katalog `taifun`, `flut`: `everyNights` 5, `firstNight` 3, `speedFactor` 0,7, `rangeFactor` 0,7, `floodUnits` 24), einbinden, laden, prüfen.
2. Welt-Datei: optionales Feld `weather`, Prüfung gegen den Katalog; `sengoku` und `nil` setzen; `welten.md` § 2 und § 3 ergänzen.
3. Zeitplan je Insel: Wetternacht, Vorboten-Tag, Ausweichen vor Vollmond und Blutmond; reine Funktion aus Seed und Tag.
4. Taifun: Tempo aller Einheiten und Reichweite der Schützen beider Seiten, nur Tiefe 0.
5. Flut: Randzonen unpassierbar (bis höchstens zum Burg-Radius), Einheiten laufen heraus, Portale darin halten ihre Gegner bis zum Morgen.
6. Zustand `weather` (Art, Phase, gesperrte Bereiche) für die Anzeige.
7. Tests je Kriterium, auch zwei Spieler auf verschiedenen Stufen und Laden in der Wetternacht.
8. Reviewer-Agent `k3c-welten` vor dem Commit.

## Fertig, wenn

- [ ] AC-02: B-969/AC-02 bis AC-08 je durch einen Go-Test belegt (Testname im Ergebnis).
- [ ] B-969/AC-09: `check_run task:check:diff` grün, `testdata/golden/` unverändert oder begründet.
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
