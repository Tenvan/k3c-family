# tools/rustpaket/

## Responsibility

CLI für den Paketbau des Rust-Spiels (`task rust:game:build`, RU57): `lizenzen` erzeugt `LIZENZEN.txt` (B-1121), `pruefen` prüft den Paketordner `dist/k3c-game` gegen die feste Liste (B-1137). Nur Go-Standardbibliothek, im Wurzel-Modul.

## Design

- `main.go`: Unterbefehle `lizenzen` und `pruefen`, Exit 1 mit Meldung bei Fehler.
- `lizenzen.go`: `cargo metadata --locked --filter-platform` (Windows-Ziel); `cmdLizenzen` (`-exe`, `-rust`, `-target`, Vorgabe `x86_64-pc-windows-msvc`) holt den Kopf aus `k3c-game licenses`, nimmt die Pakete von `k3c-game` und `k3c-devwin` (Cheat-Fenster, RU56) und vereinigt sie mit `mergePackages`; `linkedPackages` nimmt ab einem Wurzel-Paket alle fremden Pakete über normale Abhängigkeiten (keine Workspace-Mitglieder, keine Build-/Dev-Abhängigkeiten); `render` schreibt den Kopf (`k3c-game licenses`, Liste der Lizenz-Seite) und je Crate Name, Version, SPDX und die LICENSE-/COPYING-/NOTICE-Dateien; ohne Lizenzdatei stehen SPDX, Quelle und Adresse des Standardtexts da, ohne SPDX bricht es ab.
- `pruefen.go`: `pruefe` über ein `fs.FS` (testbar ohne Platte): Pflichtdateien (`k3c-game.exe`, `k3c-devwin.exe`, `README.txt`, `LICENSE`, `LICENSE-ASSETS`, `LIZENZEN.txt`, `public/schrift/LICENSE`) und -ordner (`data`, `public/grafik`, `public/audio`, `public/rig`), verbotene Dateien (Go- und Entwickler-Reste, Endungen `.go`, `.rs`, `.pdb`, `.tmp`) und Ordner, EXE-Größe (`k3c-game.exe` G1, 40 MiB; `k3c-devwin.exe` höchstens 5 MiB), Crate-Abschnitt in `LIZENZEN.txt`; `-lokal` (im Bau `game:dist`) duldet `saves/`, `reports/`, `trace/`, die beim Spielen aus dem Paketordner entstehen, der strenge Lauf `game:dist:check` und die CI nicht.
- `testdata/`: Mini-Crates für die Tests.

## Flow

`rust:game:dist` (rust/Taskfile.yml): `tools/sync` der Dateien, Cheat-Fenster daneben → `rustpaket lizenzen` → `rustpaket pruefen -lokal`; `rust:game:dist:check` ruft `pruefen` streng.

## Integration

- Aufrufer: `rust/Taskfile.yml` (`game:dist`, `game:dist:check`); Eingabe `cargo metadata` im Ordner `rust/`.
- Tests: `lizenzen_test.go`, `pruefen_test.go` mit `testdata/`.
