# Abnahme B-1126: Skills und Inselwahl (RU64.3)

2026-11-19, STIRA-PC (16 Kerne), Second Worker, Commit 02cf2679 (Code gleich `origin/main` 0d4096d6; seither nur Docs). rustc 1.99.1.

| Lauf | Befehl | Dauer | Ergebnis |
|---|---|---|---|
| e2e_flow Release | `cargo test --release -p k3c-game --test e2e_flow` | 6,4 s Test (9,5 s mit Aufruf) | grün, 1 Test (`ablauf_wie_go_zwei_laeufe_gleich`) |
| e2e_flow Debug | `cargo test -p k3c-game --test e2e_flow` | 39,9 s Test (1 min 7 s mit Übersetzen) | grün |
| Workspace | `cargo test --workspace --release --no-fail-fast` | 34 s (zweiter Lauf, Übersetzung warm) | **5 rot**, 2668 grün, 17 ignoriert (Summe aller `test result`-Zeilen) |
| Race | Zitat `docs/messung/abnahme-b-1123.md` (RU64.2, gleicher Code) | 12 min 28 s | keine Data Races; nur `engine/internal/detvec` rot (Daten-Test, nicht Race) |
| testdata | `git status` | – | `testdata/` unverändert |

`e2e_flow` hat keine `#[ignore]`-Marken (`grep -c "#\[ignore"` = 0) und keine Ausnahmeliste: grün ohne Skip für Skills oder Inselwahl.
Ohne `--no-fail-fast` bricht `cargo test --workspace` beim ersten roten Binary (`still`) ab.

## Rote Workspace-Tests

| Test | Meldung |
|---|---|
| `k3c-game/tests/still.rs` `bildvergleich_an_fuenf_staenden` | `mittag: 0.0056 der Pixel weichen ab, erlaubt 0.005` (RU51.8 meldete grün; knapp über der Schwelle, Rechner-/Schrift-/Grafikabhängigkeit möglich) |
| `k3c-game/tests/still_hud.rs` `plaetze_stimmen_mit_go_ueberein` | `tag-allein` Zelle 0: Elemente `[0,13,2,3,5,6,7]` statt `[0,3,5,6,7]` (bekannt, RU51.8: Radar Element 13) |
| `still_hud.rs` `bilder_stimmen_mit_go_ueberein` | über Schwelle: tag-allein 0,504, nacht-banner 1,128, pad-tastatur 1,984, englisch-hub 1,102; Boxfarbe Rust [60,90,130] gegen Go [27,41,59] (bekannt, RU51.8) |
| `k3c-game/tests/still_props.rs` `bilder_stimmen_mit_go_ueberein` | basis-tag 5,714; baustelle 6,074; mauer-ruine 17,822; wald-plantage 25,285; kampf 4,001 |
| `k3c-sim` Lib `island::worlds_tests::eingebettete_welten_gueltig` | eingebettet `["rom.json","standard.json"]`, erwartet nur `standard.json` (Welt `rom.json` neu, Test überholt) |

## Für den Hauptrechner

- AC-03 aus B-1126 (Skills und Inselwahl): Die Ablauf- und Welt-Tests (`e2e_flow`) sind grün; die fünf roten Tests betreffen Standbild-Vergleiche gegen Go (`still`, `still_hud`, `still_props`) und eine Welt-Liste (`rom.json`), nicht Skills oder Inselwahl.
- Tickets (Typ Problem): (1) `still_props` fünf Stände weit über der Schwelle (mauer-ruine, wald-plantage 17 bis 25), (2) `still_hud` (bereits aus RU51.8 offen), (3) `still` mittag 0,0056, (4) `worlds_tests` Erwartung `rom.json`.
- Prüfliste für 🧑 (Spieltest zu zweit am Pad, nicht von mir geprüft): Skill-Menü zu zweit (Lernen, Respec nur am Tag an der Burg), Seekarte und Zielinsel-Wahl nach Endboss-Sieg.
