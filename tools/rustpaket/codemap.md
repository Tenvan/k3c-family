# tools/rustpaket/

## Responsibility

CLI für den Paketbau des Rust-Spiels (`task rust:game:build`, RU57): `lizenzen` erzeugt `LIZENZEN.txt` (B-1121), `pruefen` prüft den Paketordner `dist/k3c-game` gegen die feste Liste (B-1137). Nur Go-Standardbibliothek, im Wurzel-Modul.

## Design

- `main.go`: Unterbefehle `lizenzen` und `pruefen`, Exit 1 mit Meldung bei Fehler.
- `lizenzen.go`: `cargo metadata --locked --filter-platform` (Windows-Ziel); `linkedPackages` nimmt ab `k3c-game` alle fremden Pakete über normale Abhängigkeiten (keine Workspace-Mitglieder, keine Build-/Dev-Abhängigkeiten); `render` schreibt den Kopf (`k3c-game licenses`, Liste der Lizenz-Seite) und je Crate Name, Version, SPDX und die LICENSE-/COPYING-/NOTICE-Dateien; ohne Lizenzdatei stehen SPDX, Quelle und Adresse des Standardtexts da, ohne SPDX bricht es ab.
- `pruefen.go`: `pruefe` über ein `fs.FS` (testbar ohne Platte): Pflichtdateien und -ordner, verbotene Dateien (Go- und Entwickler-Reste), EXE-Größe (G1, 40 MiB), Crate-Abschnitt in `LIZENZEN.txt`; `-lokal` (im Bau `game:dist`) duldet `saves/`, `reports/`, `trace/`, die beim Spielen aus dem Paketordner entstehen, der strenge Lauf `game:dist:check` und die CI nicht.
- `testdata/`: Mini-Crates für die Tests.

## Flow

`rust:game:dist` (rust/Taskfile.yml): sync der Dateien → `rustpaket lizenzen` → `rustpaket pruefen`.
