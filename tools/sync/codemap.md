# native/k3c-game/cmd/sync/

## Responsibility

Kommando `sync`: kopiert Dateien und Ordner ins Spiel-Paket `dist/k3c-game/`, nur wenn sie im Ziel fehlen oder Größe bzw. Änderungszeit abweichen.

## Design

- `main.go`: je Argument `quelle=ziel` eine Datei oder ein Ordner (rekursiv, Ordner zusammengeführt); `syncPath` läuft durch die Quelle, `syncFile` vergleicht Größe und Änderungszeit, `copyFile` schreibt; `fail` beendet mit Fehlercode.
- Eigenes Werkzeug statt `cp`, weil Task unter Windows ein eingebautes `cp` ohne `-u` und ohne Zusammenführen nutzt.
- Nur Stdlib.

## Flow

1. Task `rust:game:build` schreibt mit `k3c-game --assets` die Liste `assets.txt` und baut daraus die Paare (EXE, `tools/measure/measure.ps1`, `tools/measure/messen.cmd`, `public/audio`, `public/schrift`, `native/k3c-rig/assets`, je Asset `public/grafik/<gruppe>/<id>`).
2. `go run ./cmd/sync <paare>` → `main` → `syncPath` je Paar → `syncFile`/`copyFile`; Ausgabe der Anzahl kopierter Dateien.

## Integration

- Konsument: `rust/Taskfile.yml` (`game:dist`, über `game:build` und `game:paket`).
