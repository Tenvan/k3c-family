# Abnahme-Läufe RU26: Rust-Spielschleife (B-1096, RU51.6)

Stand: 2026-11-19, Second Worker, Branch `sprint/ru51` auf `origin/main` (25f46a4a).

| AC | Lauf | Ergebnis |
|---|---|---|
| AC-01 | Sim | verschoben (Sperre 2026-11-11; `sim_test` lehnt ab, siehe `abnahme-ru18.md`) |
| AC-02 | Balance | verschoben (Sperre 2026-11-11) |
| AC-03 | `check_run task:test` samt `gen:paper`, Golden | 2005 grün, 2 rot, nicht Spielschleife (`abnahme-ru21.md`); `testdata/golden` unverändert |
| AC-04 | Race | `task go:race` grün, kein DATA RACE (`abnahme-ru14.md`) |
| zusätzlich | `cargo test --release -p k3c-game --no-fail-fast` | 931 grün, 10 ignoriert, 4 rot (alle Bildvergleich gegen Go, siehe unten); Spielschleife (`run_flow`, `run_smoke`, `run_smoke_grafik`, `replay_headless`, `replay_trace`, `run_args`) grün |

## Rote Tests (alle Bildvergleich Rust gegen Go)

- `still::bildvergleich_an_fuenf_staenden`: „morgen: mittlere Abweichung 29.454 über 0.5“.
- `still::lichtkarte_fehlt_in_rust`: „abend: Abweichung 0.11 gering, hat Rust jetzt eine Lichtkarte? (B-1056)“ (der Test erwartet eine große Abweichung, solange Rust keine Lichtkarte hat).
- `still_hud::plaetze_stimmen_mit_go_ueberein`: `tag-allein` Zelle 0 zeigt Element 13 zusätzlich (`["0","13","2","3","5","6","7"]`).
- `still_hud::bilder_stimmen_mit_go_ueberein`: über Schwelle in `tag-allein` (mittel 0,504), `nacht-banner` (1,128), `pad-tastatur` (1,984), `englisch-hub` (1,102); `englisch-hub` Element 3: Boxfarbe Rust [60, 90, 130] gegen Go [27, 41, 59], Textumriss um 11 px versetzt.

Vermutung (ungeprüft): Die Looks änderten sich nach dem Baseline-Stand dieser Tests (Look/Farben RU54, RU55 ist noch offen); die Erwartung wurde nicht angepasst.

## Prüfliste für Ralf (B-1096 AC-05, im Spiel mit zwei Pads)

1. Takt und Zwischenbilder: Bewegung und Kamera bei 60 Hz ruhig, kein Zittern, kein Gleiten beim Stufenwechsel (`JUMP_UNITS`); Vergleich mit dem Go-Spiel (`--pacing sync`) auf demselben Rechner.
2. Zwei Spieler: Zelle oben und unten, jede Kamera folgt dem eigenen Monarchen, jedes Gerät steuert nur seinen Spieler, Beitritt mit Bestätigen, verlorenes Pad wird ersetzt.
3. Menü: B geht in jedem Menü zurück, im Spiel ist B unbelegt, Pause öffnet und schließt.
4. Vollbild-Taste F schaltet Vollbild und Fenster (Größe und Lage setzt Rust nicht, B-1094).
5. Leistungsanzeige (F2/F3) zeigt FPS und Zeiten.
6. Cheat-Menü (F9, Pad View+Y): ab RU52.7 (Branch `sprint/ru52`) löst ein Cheat aus.
