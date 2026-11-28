# tools/rustpaket/

## Responsibility

CLI für den Paketbau des Rust-Spiels (`task rust:game:build`, RU57): `lizenzen` erzeugt `LIZENZEN.txt` (B-1121), `pruefen` prüft den Paketordner `dist/k3c-game` gegen die feste Liste (B-1137).

## Design

TypeScript unter bun, nur Node-APIs (RU71); Aufruf `bun tools/rustpaket/main.ts lizenzen|pruefen …`.

- `main.ts` (Einstieg), `app.ts`: Unterbefehle `lizenzen` (`-exe`, `-rust`, `-target`, Vorgabe `x86_64-pc-windows-msvc`) und `pruefen` (`-max-exe-mib`, `-lokal`), Flags über `../checkdiff/flags`; ohne gültigen Unterbefehl Exit 2, sonst Exit 1 mit Meldung bei Fehler.
- `lizenzen.ts`: `cargo metadata --locked --filter-platform` (Windows-Ziel), Programme ohne Shell (`../checkdiff/exe`); der Kopf kommt aus `k3c-game licenses`, die Pakete von `k3c-game` und `k3c-devwin` (Cheat-Fenster, RU56) vereinigt `mergePackages`; `linkedPackages` nimmt ab einem Wurzel-Paket alle fremden Pakete über normale Abhängigkeiten (keine Workspace-Mitglieder, keine Build-/Dev-Abhängigkeiten); `render` schreibt je Crate Name, Version, SPDX und die LICENSE-/COPYING-/NOTICE-Dateien (als Bytes, nur ohne `\r`); ohne Lizenzdatei stehen SPDX, Quelle und Adresse des Standardtexts da, ohne SPDX bricht es ab.
- `pruefen.ts`: `pruefe` über eine Dateiliste (testbar ohne Platte), `listEntries` liest sie: Pflichtdateien (`k3c-game.exe`, `k3c-devwin.exe`, `README.txt`, `LICENSE`, `LICENSE-ASSETS`, `LIZENZEN.txt`, `public/schrift/LICENSE`) und -ordner (`data`, `data/worlds`, `public/grafik`, `public/audio`, `public/rig`), verbotene Dateien (Go- und Entwickler-Reste, Endungen `.go`, `.rs`, `.pdb`, `.tmp`) und Ordner, EXE-Größe (`k3c-game.exe` G1, 40 MiB; `k3c-devwin.exe` höchstens 5 MiB), Crate-Abschnitt in `LIZENZEN.txt`; `-lokal` (im Bau `game:dist`) duldet `saves/`, `reports/`, `trace/`, die beim Spielen aus dem Paketordner entstehen, der strenge Lauf `game:dist:check` und die CI nicht.
- `testdata/`: Mini-Crates für die Tests.

## Flow

`rust:game:dist` (rust/Taskfile.yml): `tools/sync` der Dateien, Cheat-Fenster daneben → `rustpaket lizenzen` → `rustpaket pruefen -lokal`; `rust:game:dist:check` ruft `pruefen` streng.

## Integration

- Aufrufer: `rust/Taskfile.yml` (`game:dist`, `game:dist:check`); Eingabe `cargo metadata` im Ordner `rust/`.
- Tests: `lizenzen.test.ts`, `pruefen.test.ts` mit `testdata/` (Vitest der Wurzel).
- Die Go-Dateien im Ordner nutzt niemand mehr; Löschung in RU71.4.
