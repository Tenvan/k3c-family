# tools/portcheck/

## Responsibility
CLI `portcheck` (RU60.1): gleicht die Go-Tests des Tags `go-referenz-final` mit den Rust-Funktionsnamen im Arbeitsbaum ab und schreibt den Bericht `docs/messung/port-testabgleich.md`. Es liest nur `git` und Dateien, nie einen Compiler; „fehlt“ heißt „kein gleicher Name“, nicht „Verhalten fehlt“. Nur Standardbibliothek, im Wurzel-Modul.

## Design
- `main.go`: Flags `-tag` (Vorgabe `go-referenz-final`) und `-out`; `run` listet per `git ls-tree` die Dateien des Tags, sammelt die Go-Tests, liest je Rust-Ordner einmal die Funktionsnamen, vergleicht je Bereich und schreibt den Bericht; Fehler → `💥 portcheck`, Exit 1. `git` ruft das Programm im aktuellen Ordner auf.
- `areas.go`: Tabelle `areas` ordnet je Bereich einen Go-Ordner im Tag einem Rust-Ordner zu (`native/k3c-game/<hud|props|view|world|fx|actors|menu|local|game|input|audio|art|devtools|e2e>` → `rust/crates/k3c-game/src/…` bzw. `tests`, `native/k3c-rig` → `k3c-rig`, `engine/sim|level|room|store|balance` → `k3c-sim|k3c-level|k3c-room|k3c-balance`). `collectGo` liest jede `*_test.go` des Tags per `git show` und findet `func Test…` per Regex; Tests ohne Bereich landen unter dem Schlüssel `\x00<ordner>`. `rustFns` sammelt alle `fn`-Namen der `.rs`-Dateien eines Ordners; `withoutTarget` zählt die Go-Tests je Ordner ohne Rust-Bereich.
- `names.go`: `snake` macht aus `TestFooBar` den Rust-Namen `foo_bar` (Präfix `Test` entfällt, Großbuchstabenfolgen bleiben zusammen, `_` bleibt Trenner); `themes`/`theme` ordnen einen fehlenden Test nach Dateiname einem Sprint-Thema zu (Krone, Portale, Seekarte, Handwerker, Endboss …), sonst „allgemein“.
- `report.go`: `compare` zählt je Bereich Go-Tests und Treffer (`snake(name)` unter den Rust-Namen; Subtests zählen als Elterntest); `render` schreibt Tabelle je Bereich mit Summenzeile, Zählung nach Thema, Liste der fehlenden Tests je Bereich und den Abschnitt „Ohne Ziel“.

## Flow
1. `task gen:portcheck` → `go run ./tools/portcheck` → `run`.
2. `git ls-tree` des Tags → `collectGo` (je Datei `git show`) → je Bereich `rustFns` → `compare` → `render` → `docs/messung/port-testabgleich.md`.

## Integration
- Aufrufer: Task `gen:portcheck` in `tools/Taskfile.yml` (Namensraum `gen`).
- Externe Programme: `git`; Eingaben: das Tag `go-referenz-final` und `rust/crates/**`.
- Test: `names_test.go` (`snake`).
