# tools/sync/

## Responsibility

Kommando `sync`: kopiert Dateien und Ordner ins Spiel-Paket `dist/k3c-game/`, nur wenn sie im Ziel fehlen oder Größe bzw. Änderungszeit abweichen.

## Design

TypeScript unter bun, nur Node-APIs (RU71); Aufruf `bun tools/sync/main.ts quelle=ziel …`.

- `main.ts` (Einstieg), `sync.ts`: je Argument `quelle=ziel` eine Datei oder ein Ordner (rekursiv, Ordner zusammengeführt); `syncPath` läuft durch die Quelle, `syncFile` vergleicht Größe und Änderungszeit, schreibt und übernimmt die Quellzeit. Node setzt Zeiten gröber als Go, darum gilt eine Abweichung unter 1 µs als gleich. Ein Schreibfehler weist darauf hin, dass das Spiel im Ziel noch läuft.
- Eigenes Werkzeug statt `cp`, weil Task unter Windows ein eingebautes `cp` ohne `-u` und ohne Zusammenführen nutzt.

## Flow

1. Task `rust:game:dist` (aus `rust:game:build`) baut die Paare aus Release-EXE, `data/`, `public/grafik`, `public/rig`, `public/audio`, `LICENSE`, `LICENSE-ASSETS`, `rust/paket/README.txt` und, falls vorhanden, `public/schrift`, jeweils nach `dist/k3c-game/`.
2. `bun tools/sync/main.ts <paare>` → `main` → `syncPath` je Paar → `syncFile`; Ausgabe der Anzahl kopierter Dateien (`✅ n Dateien aktualisiert`), Fehler mit `❌` und Exit 1.

## Integration

- Konsument: `rust/Taskfile.yml` (`game:dist`, über `game:build`).
- Test: `sync.test.ts` (Vitest der Wurzel).
- Die Go-Dateien im Ordner nutzt niemand mehr; Löschung in RU71.4.
