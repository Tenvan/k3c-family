# Abnahme-Läufe RU21: Rust-Store und Raum (B-1098, RU51.2)

Stand: 2026-11-19, Second Worker, Branch `sprint/ru51` auf `origin/main` (25f46a4a).

| AC | Lauf | Ergebnis | Dauer |
|---|---|---|---|
| AC-01 | `sim_test` | nicht betroffen: RU21 ändert keinen Sim-Code; Sim-Tests der Rust-Sim siehe `abnahme-ru14.md` | – |
| AC-02 | Balance | nicht relevant: kein Wert in `data/` geändert | – |
| AC-03 | `check_run task:test` (Vitest, `gen:paper`, Golden) | 2005 grün, 2 rot (siehe unten) | 4 min 25 s |
| AC-04 | `cargo test --release -p k3c-room` | 235 grün, 0 ignoriert, 0 rot | 17 s |
| AC-04 | `go test -race` (`task go:race`) | grün, kein DATA RACE (Lauf siehe `abnahme-ru14.md`) | ca. 20 min |

Rote Läufe in `task:test`:

1. `tests/planning.test.ts` › Planungsangaben für Arbeitswellen: `B-1143-feature-sprints-mit-go-spiel-bereich-umplanen.md: Umgebung: ?` (Planungsdaten des Hauptrechners, nicht Rust-Store/Raum).
2. `tools/paper/render/render.test.ts` › Temp-Ordner laufender Prozesse: `EPERM` beim `rename` im Windows-Temp (vermutet: Virenscanner, ungeprüft).
