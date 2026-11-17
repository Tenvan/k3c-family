# RU3.4 · Daten-Schemas Teil 1: Dateien bis hub.json

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.3
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

`k3c-data` lädt die Dateien `balance-hints` bis `hub` (alphabetisch) und die Biome in typisierte Strukturen, wertgleich zu Go.

## Kontext

Die Daten liegen unter `data/*.json`, `data/biomes/` und `data/worlds/`; `data/embed.go` bettet sie ein oder lädt sie von der Platte (`K3C_DATA_DIR`). Die Go-Strukturen dazu stehen verteilt in `engine/sim` (`data.go`, `*_data.go`), `engine/level` und `native/k3c-game`; die Struktur zu einer Datei findet `grep -rn '"<name>.json"' engine data native`. Dateien dieser Session: `balance-hints`, `balance-targets`, `bosses`, `buildings`, `difficulty`, `economy`, `enemies`, `events`, `goals`, `guards`, `hub`, dazu `biomes/*.json`. Nicht zu den Spiel-Daten gehören `asset-prompts`, `assets-recherche`, `assets-vorschlaege` (Workbench) und alles unter `data/.omc`. Go ignoriert unbekannte Felder; Rust tut das auch. Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen).

## Erlaubte Dateien

- `rust/crates/k3c-data/**`
- `engine/internal/detvec/**`
- `testdata/rust/**`

## Nicht-Ziele

Werte ändern. Felder umbenennen. Logik, die über Laden und Prüfen hinausgeht.

## Schritte

1. Je Datei die Go-Struktur suchen und ganz lesen, samt der Prüfungen beim Laden.
2. Go-Test in `engine/internal/detvec`: lädt jede Datei dieser Session in ihre Go-Struktur und schreibt den JSON-Baum der Struktur nach `testdata/rust/data-tree.json.gz` (Schlüssel = Dateiname; mit `-write` schreiben, sonst vergleichen).
3. Je Datei ein Modul `src/<name>.rs` mit den Strukturen nach Leitfaden § 3 und einer Funktion `load(…)`, die dieselben Prüfungen macht wie Go, mit denselben Fehlertexten.
4. Test je Datei: laden, `golden::tree` bilden, mit dem Eintrag aus `data-tree.json.gz` vergleichen.
5. `cargo test -p k3c-data`, `go test ./engine/internal/detvec/`, `task rust:lint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-04 (Teil 1): Für jede Datei dieser Session ist der JSON-Baum der Rust-Struktur gleich dem der Go-Struktur.
- [ ] Im Ergebnis steht je Datei die Go-Struktur mit Pfad.

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

- AC-04 (Teil 1) geprüft: `cargo test -p k3c-data` grün (17 Tests); je Datei lädt `load…`, `golden::tree` gleicht dem Eintrag aus `testdata/rust/data-tree.json.gz`; `go test ./engine/internal/detvec/` grün (`TestDataTree` vergleicht ohne `-write`); `task check:diff` grün. Mutationsprobe (Alias `hpFactor` entfernt) schlägt fehl.
- Go-Strukturen je Datei (Lese-Typ in Go, Rust-Modul):
  - `balance-hints.json`: `balance.Hints` über `LoadHints` + `checkGroups` (engine/balance/hints.go, groups.go) → `balance_hints.rs`
  - `balance-targets.json`: `balance.Targets` über `LoadTargets` + `checkRoles` (engine/balance/targets.go, seedmode.go) → `balance_targets.rs`
  - `bosses.json`: `{Bosses, EndBosses []sim.BossData}` = Vereinigung von `loadBosses`/`loadEndBosses` (engine/sim/boss_data.go) → `bosses.rs`
  - `buildings.json`: `map[string]sim.BuildingData` (engine/sim/data.go) → `buildings.rs`
  - `difficulty.json`: `struct{ Grades map[string]sim.Grade }` (engine/sim/island_options.go) → `difficulty.rs`
  - `economy.json`: `sim.economyData` (engine/sim/world_values.go) → `economy.rs`
  - `enemies.json`: `map[string]sim.EnemyData` (engine/sim/enemies_traits.go) → `enemies.rs`
  - `events.json`: `sim.eventsFile` (engine/sim/events_values.go), Prüfung `checkNightEvents` → `events.rs`
  - `goals.json`: `sim.Goals` (engine/sim/data.go) → `goals.rs`
  - `guards.json`: `level.GuardRules` (engine/level/guards.go) → `guards.rs`
  - `hub.json`: `sim.HubData` (engine/sim/data.go) → `hub.rs`
  - `biomes/*.json`: `level.Biome` über `LoadBiome` (engine/level/biome.go, nature_check.go), `checkPool` aus engine/sim/data.go → `biome.rs`, `ordered.rs` (`level.Ordered`)
- Vorgehen: Strukturen mit einem einmaligen Generator aus den Go-Typen (reflect) und den Schlüsseln der Daten erzeugt (nicht eingecheckt), Prüfungen von Hand. Ausgabe-Namen wie Go (`HPFactor`), gelesen über `alias` (Datei-Schlüssel); Balancing-Dateien mit `deny_unknown_fields` wie `DisallowUnknownFields`.
- Abweichungen: Fehlertexte von JSON-Fehlern kommen von `serde_json`; Prüfungen, die Go über Maps zufällig durchläuft, laufen sortiert. Nicht portiert, weil die Tabellen in engine/balance bzw. die Sim-Logik fehlen: Kennzahl/Bot/Spieler/Grad/Welt/Hilfen bei `balance-*`, `hub_validate` → B-1024 (Sprint RU25). `withShip` (Sim-Logik) nicht Teil des Ladens. `k3c_det::golden::go_float` öffentlich für Go-gleiche Zahlen in Fehlertexten.
- Neue Tickets: B-1024.
