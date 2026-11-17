# RU3.5 · Daten-Schemas Teil 2: übrige Dateien, Welten, Laden von Platte und eingebettet

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.4
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

`k3c-data` lädt alle Spiel-Daten, von der Platte oder eingebettet, wertgleich zu Go.

## Kontext

Wie RU3.4. Dateien dieser Session: `island-sizes`, `light`, `monarch`, `native`, `paper`, `render`, `ruins`, `ship`, `sounds`, `story`, `stresstests`, `troops`, `waves`, dazu `worlds/*.json` (vier Welten). Laden: `data/embed.go` nimmt den Ordner aus `K3C_DATA_DIR`, sonst die eingebetteten Dateien; eingebettet ist nur die Welt `standard` (`go:embed *.json biomes/*.json worlds/standard.json`). Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen).

## Erlaubte Dateien

- `rust/crates/k3c-data/**`
- `engine/internal/detvec/**`
- `testdata/rust/**`

## Nicht-Ziele

Werte ändern. Eine neue Abhängigkeit zum Einbetten (`include_str!` genügt).

## Schritte

1. Wie RU3.4, Schritte 1 bis 4, für die Dateien dieser Session; `data-tree.json.gz` um ihre Einträge ergänzen.
2. `src/source.rs`: `enum Source { Dir(PathBuf), Embedded }`; `Source::from_env()` liest `K3C_DATA_DIR` wie `data/embed.go`; eingebettet über `include_str!` genau die Dateien, die Go einbettet.
3. Test: Laden von Platte und eingebettet ergibt für jede eingebettete Datei denselben Baum; eine fehlende Datei ergibt denselben Fehlertext wie Go.
4. `cargo test -p k3c-data`, `go test ./engine/internal/detvec/`, `task rust:lint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-04: Für jede Spiel-Daten-Datei ist der JSON-Baum der Rust-Struktur gleich dem der Go-Struktur.
- [ ] AC-04: Laden von Platte und eingebettet ist getestet.

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

- AC-04 geprüft: `cargo test -p k3c-data` grün (25 Tests); `rest_wie_go` und `standard_welt_wie_go` vergleichen die Bäume mit `testdata/rust/data-tree.json.gz`, `jede_datei_aus_go_ist_geprueft` hält Baum und Rust-Tests gleich; `go test ./engine/internal/detvec/` grün, dort prüft `TestDataFilesComplete`, dass jede Spiel-Daten-Datei (ohne `asset-*`, `assets-*`, `.omc`) im Baum steht; `task check:diff` grün.
- AC-04 Laden: `k3c_data::Source` (`Dir`, `Embedded`, `from_env` mit `K3C_DATA_DIR` und data neben der EXE wie `data/embed.go`), eingebettet über `include_str!` genau `*.json biomes/*.json worlds/standard.json` (Test `eingebettet_wie_go_embed`); `load_all` lädt alles samt Prüfungen über mehrere Dateien; `platte_und_eingebettet_gleich`; `fehlende_datei_wie_go` mit Gos Texten (`file does not exist` bzw. Text des Betriebssystems über `os.DirFS`).
- Go-Strukturen je Datei: `island-sizes` → `struct{ Sizes []sim.IslandSize }` (engine/sim/island_sizes.go); `monarch` → `sim.monarchData` (monarch_data.go); `ruins` → `level.RuinRules` (engine/level/ruins.go); `ship` → `sim.shipFile` (ship.go); `story` → `sim.StoryLimits` (story.go); `troops` → `map[string]sim.TroopData` (data.go); `waves` → `sim.wavesData` (data.go); `worlds/standard.json` → `sim.rawWorld`/`rawCombat` (island_worlds.go, combat_data.go; Go-Test prüft vorher mit `ParseWorld`). Ohne Go-Struktur im Root-Modul, Vorbild ist der Aufbau der Datei: `light`, `native`, `paper`, `sounds`, `stresstests` (nur Teil-Leser im eigenen Modul native/k3c-game) und `render` (kein Go-Leser).
- Abweichungen vom Plan: Im Daten-Baum steht nur die Welt `standard`; die anderen drei Welten sind lokal und gitignored (welten.md § 6), im Worktree nicht vorhanden. Welten nur bis zum Kopf geprüft, Inseln roh. `story.json`: Go überliest Lesefehler, Rust meldet sie. Review-Befund (Biom-Liste im Rust-Test fest) behoben: Biome werden wie in Go aus `biomes/*.json` gelesen. Übrige Review-Befunde (Groß-/Kleinschreibung der Schlüssel, `null` in Zahlfeldern, Länge von `[N]T`) als bekannte Abweichungen in B-1024.
- Neue Tickets: keine; B-1024 ergänzt (Welten, Native-Leser, Lese-Abweichungen).
