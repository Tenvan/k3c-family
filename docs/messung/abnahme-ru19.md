# Abnahme-Läufe RU19: Events, Geschichte, Dev-Eingriffe (B-1104, RU51.4)

Stand: 2026-11-19, Second Worker, Branch `sprint/ru51` auf `origin/main` (25f46a4a).

| Lauf | Ergebnis |
|---|---|
| `cargo test --release -p k3c-sim -- --ignored` | 0 grün, 70 rot (Rümpfe/ungeklärte Ursachen auf main; `sprint/ru52`, ungemergt, baut sie auf 977 grün / 5 ignoriert ab) |
| `cargo test --release -p k3c-sim` | 911 grün, 70 ignoriert, 1 rot (`eingebettete_welten_gueltig`, rom.json) |
| `sim_test` / Balance | nicht gefahren: Workbench-Sperre (🧑 2026-11-11, `K3C_SIM_ENTSPERRT`), siehe `abnahme-ru18.md` |
| `task gen:paper` samt Golden | grün (up to date); `task:test` 2005 grün, 2 rot nicht RU19 (`abnahme-ru21.md`) |
| `check:race` (`task go:race`) | grün, kein DATA RACE (`abnahme-ru14.md`) |
| Messlauf `events/budget_tests.rs` | `ereignis_budget_je_tick` grün: Mittel der Ereignis-Bytes je Stufe und Tick ≤ 200 B (Budget `EVENT_BUDGET_BYTES = 200`); `gehaltene_skill_tasten_im_ereignis_budget` grün. Der Test gibt den Ist-Mittelwert nicht aus, nur die Grenze 200 B ist belegt |
