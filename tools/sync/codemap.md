# tools/sync/

## Responsibility

Kommando `sync`: kopiert Dateien und Ordner ins Spiel-Paket `dist/k3c-game/`, nur wenn sie im Ziel fehlen oder Größe bzw. Änderungszeit abweichen.

## Design

- `main.go`: je Argument `quelle=ziel` eine Datei oder ein Ordner (rekursiv, Ordner zusammengeführt); `syncPath` läuft durch die Quelle, `syncFile` vergleicht Größe und Änderungszeit, `copyFile` schreibt und `Chtimes` übernimmt die Quellzeit; ein Schreibfehler weist darauf hin, dass das Spiel im Ziel noch läuft; `fail` beendet mit Fehlercode.
- Eigenes Werkzeug statt `cp`, weil Task unter Windows ein eingebautes `cp` ohne `-u` und ohne Zusammenführen nutzt.
- Nur Stdlib.

## Flow

1. Task `rust:game:dist` (aus `rust:game:build`) baut die Paare aus Release-EXE, `data/`, `public/grafik`, `public/rig`, `public/audio`, `LICENSE`, `LICENSE-ASSETS`, `rust/paket/README.txt` und, falls vorhanden, `public/schrift`, jeweils nach `dist/k3c-game/`.
2. `go run ./tools/sync <paare>` → `main` → `syncPath` je Paar → `syncFile`/`copyFile`; Ausgabe der Anzahl kopierter Dateien (`✅ n Dateien aktualisiert`).

## Integration

- Konsument: `rust/Taskfile.yml` (`game:dist`, über `game:build`).
