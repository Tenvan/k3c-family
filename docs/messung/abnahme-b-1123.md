# Abnahme B-1123: Paket, CI und schwere Läufe (RU64.2)

2026-11-19, STIRA-PC (16 Kerne), Second Worker. Go 1.27.0, rustc 1.99.1, gcc 16.2.0 (WinLibs) vorhanden. Stand `origin/main` 0d4096d6.

| AC / Lauf | Befehl | Dauer | Ergebnis |
|---|---|---|---|
| Gen + Golden | `check_run task:test` (`gen:paper` aktuell, `tools/paper/golden/public.test.ts` Golden public/grafik 261 s) | 4 min 24 s | **rot: 6 von 2044 Tests**, Golden-Test grün |
| Race | `check_run task:check:race` | 0,6 s | **Setup-Fehler:** `"grep": executable file not found in $PATH` (PATH des Task-Runners der Workbench) |
| Race (Ersatz, Git Bash) | `CGO_ENABLED=1 go test -race -timeout 20m $(go list ./engine/... \| grep -v '/engine/balance$')` | 12 min 28 s | **keine Data Races**; grün: golden, rules, trace, level, rng, room, sim, simcli, store; **rot: `engine/internal/detvec`** |
| Paket | `task buildall` | 4 s (alles aktuell) | ok; `dist/k3c-game` vollständig (2131 Dateien), `rustpaket pruefen` grün, LIZENZEN.txt vorhanden |
| EXE | `dist/k3c-game/k3c-game.exe` | – | 19,3 MB (Grenze 40 MB); Start/Beenden mit `--seed k3c --tick 5 --shots … --size 640x360`: Bild geschrieben, Exit 0 |
| CI | `gh run list` | – | CI auf `main` (Push RU62, 20:55Z) **rot**: dieselben Fehler wie unten (5 Vitest + 2 detvec); PR-Läufe von RU61 und RU62 grün (13 s) |
| Sim, Balance | `sim_test`, Balancing | – | **nicht gefahren: Sperre** (`K3C_SIM_ENTSPERRT=1` nicht freigegeben) |

## Rote Tests im Detail

1. `tools/paper/golden/hintergruende.test.ts`: „75 Assets“ und „Specs heißen hg-…“: neue Specs `hg-wald-wand` und `leitmotiv-britannien|nil|sengoku|standard` im Ordner (80 statt 75), Test erwartet genau 75.
2. `tools/paper/golden/materialien.test.ts`: „Hintergründe nutzen das Material ihres Bioms“: 76 Dateien statt 75 (gleiche Ursache).
3. `tools/paper/golden/natur.test.ts`: `birke` (237 px) und `pappel` (273 px) unter der geforderten Höhe 340–380 px.
4. `tools/paper/cli/work.test.ts` „zweiter Lauf schreibt nichts mehr“: `EPERM` beim `rename` im Windows-Temp (`.mauer.tmp-…`), wie bekannt vermutlich Virenscanner (siehe `abnahme-ru21.md`); in der CI nicht aufgetreten.
5. `engine/internal/detvec`: `TestDataFilesComplete` (cargo, coop, festival, hermits, mounts, weather fehlen in `dataFiles`) und `TestDataTree` (`data.paper.json.figuren.gegner.greed.hoehe = 48.6, erwartet 81`).

## Für den Hauptrechner

- Punkte 1 bis 3 und 5 sind keine Umgebungsfehler (CI auf `main` rot, gleiche Meldungen): Tests und Erwartung (75 Hintergründe, Baumhöhen, `dataFiles`, Golden-Baum) wurden durch neue Specs und `data/`-Dateien (RU61/RU62) überholt. Je ein Ticket (Typ Problem): Hintergründe/Leitmotive-Tests, Baumhöhen birke/pappel, detvec-Daten. `task go:golden` und `-write` habe ich nicht gefahren.
- `check_run task:check:race` braucht `grep` im PATH der Workbench (Git `usr/bin`); sonst bleibt der Lauf ein Setup-Fehler. Ticket Typ Problem, Domäne DEV/INF.
- `EPERM` im Temp (Punkt 4) trat in einem Lauf auf (Ursache nicht geprüft, vermutet Virenscanner wie in `abnahme-ru21.md`).
- Sim und Balance bleiben offen, bis 🧑 die Sperre aufhebt.
