# Abnahme RU29 und RU30: Race, Gen, Messläufe, Stresstest, Aufnahme (B-1112, B-1119, RU51.7)

Stand: 2026-11-19, Second Worker, AMD Ryzen 7 5800X, 32 GB, Windows 11, Branch `sprint/ru51` auf `origin/main` (25f46a4a).
Nur Läufe ohne Fenster, die ohne Eingriff laufen; alles mit Fenster oder Pad steht in der Prüfliste.

| Lauf | Ergebnis |
|---|---|
| Race (`task go:race`, wie `check:race`) | grün, kein DATA RACE (`abnahme-ru14.md`) |
| Gen (`gen:paper`, `task:test` mit Grafik-Golden) | `gen:paper` grün (up to date), `task:test` 2005 grün, 2 rot nicht Rust-Spiel (`abnahme-ru21.md`) |
| `cargo test --release -p k3c-game --no-fail-fast` | 931 grün, 10 ignoriert, 4 rot (Bildvergleich gegen Go, `abnahme-ru26.md`) |
| Aufnahme `--shots` (Rust, ohne Fenster) | `k3c-game.exe --shots <dir> --at 1,20,300 --seed smoke --players 2`: 3 Bilder + `strip.png`, zweimal gefahren (a, b): alle Dateien byteidentisch |
| Aufnahmevergleich Rust gegen Go | nicht neu gefahren; die Tests `still` und `still_hud` messen ihn und sind rot (`abnahme-ru26.md`: z. B. morgen 29,454 über 0,5). Sichtvergleich der Bilder (B-1114) bleibt offen |
| Stresstest | nur im Testing-Menü des Fensters startbar (kein Schalter); Ablauf und Bericht per `stress_tests`/`stressreport_tests` abgedeckt (17 grün), kein `reports/native-stress-*.json` erzeugt: Prüfliste |
| `--measure 60`, `--measure-load` | brauchen ein Fenster (`run_window`) und einen ruhigen Rechner ohne parallele Läufe: nicht gefahren, Prüfliste |
| Allokationen von Chronik und Dev-Anzeige (B-1112) | nicht gemessen (kein Messlauf ohne Fenster); gehört zu RU43 |

## Prüfliste für Ralf (am PC, Pad und Tastatur)

1. `k3c-game.exe --measure 60 --report <datei>` auf ruhigem Rechner, daneben das Go-Spiel am selben Szenario; `bilder_ohne_takt` (rund die Hälfte ist bei 60 Hz Bild und 30 Hz Takt normal) und ob Fifo-VSync die Bildrate deckelt (sonst VSync-Option für RU39); `--measure-load` Go gegen Rust, schlechtester Frame zählt.
2. Stresstest aus dem Testing-Menü starten: schreibt er `reports/native-stress-*.json` mit denselben Feldern wie der Go-Bericht?
3. Entwickler-Tasten F1, F5 bis F10, F12 im Spiel (Cheat-Menü F9 wirkt erst mit RU52.7 aus `sprint/ru52`).
4. Zwei Spieler eine Nacht mit Pad und Tastatur: eigene Bildhälfte, Beitritt/Verlassen, Pause (B im Menü zurück, im Spiel unbelegt), Niederlage ins Hauptmenü.
5. Ruckeln bei 60 Hz in beiden Hälften auch mit vielen Gegnern, beim Stufen- und Inselwechsel (Schiffsfahrt, Ladeanzeige), Leistungsanzeige F2.
6. `--grafik` auf einen leeren Ordner: Meldung mit Pfad, Enter beendet mit Exit 1.
7. Aufnahme-Bilder `--shots` gegen die Go-Bilder ansehen (B-1114).
