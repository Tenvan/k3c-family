# native/k3c-game/menu/licensefile/

## Responsibility

Kommando `licensefile`: schreibt die Lizenzdatei des nativen Spiels nach stdout, die der Paket-Build als `LIZENZEN.txt` neben die EXE legt (B-808).

## Design

- `main.go`: oben die Liste der Lizenz-Seite (`menu.LicenseText`), darunter die Volltexte aller in die EXE gelinkten Module (`LICENSE`-, `COPYING`- und `NOTICE`-Dateien aus dem Modul-Cache) samt Go-Laufzeit; `-version` setzt die Version der Datei.
- `linkedModules` fragt `go list -deps` ab und lässt die eigenen Module `k3c`, `k3c/...` aus; `licenseFiles` sammelt die Dateien je Modul-Ordner.

## Flow

1. Task `native:game:dist` (intern, aus `game:build`/`game:paket`) → `go run ./menu/licensefile -version <v> > dist/k3c-game/LIZENZEN.txt` im Modul-Ordner des Spiels.
2. `main` → `menu.LicenseText` → `go env GOROOT` (Laufzeit-Lizenz) → `linkedModules` → `licenseFiles` → Ausgabe.

## Integration

- Abhängigkeiten: `native/k3c-game/menu`, Go-Werkzeugkette (`go list`, `go env`).
- Konsument: Task `native:game:dist`.
