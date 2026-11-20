# Abnahme B-1120: Rust-Aufnahmen und Bildvergleich (RU64.1)

2026-11-19, STIRA-PC, Second Worker. Tabelle: `docs/messung/rust-bildvergleich.md`.

## Befehle und Dauer

| Befehl | Dauer | Ergebnis |
|---|---|---|
| `task gen:paper` | 1 s (aktuell, bereits gerendert) | ok |
| `cargo build --release -p k3c-game` | 2 s (bereits gebaut) | ok |
| `cargo test --release -p k3c-game --test e2e_images` (`K3C_RUST_EXE`, `K3C_E2E_OUT=.work/e2e`) | 14 s Test, 1 min 34 s mit Übersetzen | grün: Lauf a und b pixelgleich |
| `cargo test --release -p k3c-game --test e2e_compare -- --nocapture` | 2 s | Test grün (meldet nur), **14 von 14 Stationen über der Schwelle** |

`git status`: `testdata/` unverändert, nichts eingecheckt außer diesen zwei Dokumenten. `e2e_flow` nicht gefahren (optional).

## Für den Hauptrechner

- Alle 14 Stationen überschreiten Mittel 0,5 und Anteil 0,005 (Details in der Tabelle); größte Abweichung: 01 hauptmenue, 10 verlassen, 14 ende (Mittel über 11, Anteil über 0,13). Bildpaare lagen unter `.work/e2e/paare/NN-<station>.png` (gelöscht, mit den obigen Befehlen neu erzeugbar).
- Sichtung Paar 03 (spielstart): Rust zeigt das Tutorial-Zeichen „T“ deutlich größer als Go, die HUD-Symbole (Beutel, Münzen) sind in Rust dunkler/abgedunkelt; Text und Leisten stimmen sonst überein. Welt ist in beiden schwarz (Aufnahmeausschnitt). Ungeprüft, ob Absicht (Rust-Look nach RU61) oder Fehler.
- Vorschlag: pro Station-Gruppe (Menüs 01/10/14, HUD 02–07, Overlays 08/09/11–13) ein Ticket mit Entscheidung, ob die Schwelle gegen die Go-Referenz noch gilt oder ob die Referenz neu gesichert wird. Schwellen nicht angehoben, Golden nicht geschrieben.
