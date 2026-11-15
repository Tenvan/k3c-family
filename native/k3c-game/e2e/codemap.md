# native/k3c-game/e2e/

## Responsibility

Ende-zu-Ende-Test des nativen Spiels (NG9.1, B-802): treibt die echte Schleife ohne Fenster mit fester Eingabe vom Hauptmenü bis zur Niederlage und vergleicht zwei Läufe.

## Design

- `doc.go`: einzige Nicht-Test-Datei, nur Paketkommentar; die Logik steht in den `_test.go`-Dateien.
- Stationen: Lobby mit zwei Geräten, laufen, Münzen geben, bauen, Skill lernen, Tag und Nacht, Pause, Verlassen (speichert), Weiter, Niederlage; geprüft an Zustandswerten und Welt-Hash nach dem Laden.
- Mit gebauter EXE (`K3C_GAME_EXE`) zusätzlich Aufnahme im Aufnahme-Modus (`--flow --at`) und Bildvergleich zweier Läufe.

## Flow

1. `task native:game:e2e` baut die EXE (`game:exe`) und ruft `go test -run TestFlow ./e2e/` mit `K3C_GAME_EXE`, `K3C_ROOT`, `K3C_E2E_OUT`.
2. Test baut `game.Game` mit allen Fachpaketen und `local`, spielt die Eingabe-Folge Update für Update.
3. Optional startet er die EXE zweimal, Bilder nach `reports/native-game-e2e/`, und vergleicht sie.

## Integration

- Abhängigkeiten (Tests): `native/k3c-game/game`, `local`, `view`, `k3c/engine/{sim,room,level,store}`; fällt nicht unter die Importregel der Fachpakete.
