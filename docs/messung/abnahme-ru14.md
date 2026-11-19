# Abnahme-Läufe RU14 (B-1093, RU51.1)

Stand: 2026-11-19, Second Worker, Branch `sprint/ru51` auf `origin/main` (25f46a4a, ohne RU52).

| AC | Lauf | Ergebnis | Dauer |
|---|---|---|---|
| AC-01 | `cargo test --release -p k3c-sim` | 911 grün, 70 ignoriert, 1 rot: `eingebettete_welten_gueltig` (eingebettet `rom.json` + `standard.json`, erwartet nur `standard.json`; `data/worlds/rom.json` ist im Arbeitsbaum, nicht Sim-Code) | 57 s |
| AC-01 | `cargo test --release -p k3c-sim -- --ignored` | 0 grün, 70 rot (erwartet: Platzhalter-Rümpfe, in RU52.3 bis .6 auf der Sprint-Branch `sprint/ru52` abgebaut: dort 977 grün, 5 ignoriert) | 0,4 s |
| AC-02 | `go test -race` (`task go:race`, wie `check:race`) | grün, kein DATA RACE; `engine/simcli` 814 s | ca. 20 min |
| AC-03 | `task go:test` (21+ Pakete, Golden nur lesend) | grün; Arbeitsbaum danach unverändert | 129 s |
| AC-04 | Läufe auf dem Second Worker (Lane zweit) | ja | – |

Hinweis: `check_run task:check:race` bricht nach 10 min ab (Zeitlimit des Tools); der Lauf lief deshalb direkt als `task go:race`.
