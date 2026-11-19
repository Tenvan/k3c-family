# Ende-zu-Ende-Bildvergleich Go gegen Rust (RU32)

Stand: Aufbau und Werkzeug fertig, **Messwerte stehen noch aus** (Abnahme-Ticket B-1120, Lane zweit; der Hauptrechner nimmt
keine Bilder auf, B-742). Dieser Bericht ersetzt nicht die Sichtprüfung durch 🧑 (RU36).

## Aufbau

Beide Spiele bekommen dieselbe Eingabe-Folge und dieselben Takte aus `testdata/rust/e2e/flow.json` (von Go geschrieben,
`go test ./e2e -run TestRustRef -write` in `native/k3c-game`), 14 Stationen, Größe 1280x720, Aufnahme-Modus
(`--flow --input --shots --at --size`), je zweimal (a, b; die zwei Läufe einer EXE müssen pixelgleich sein).

| Schritt | Befehl |
|---|---|
| Grafik, EXE | `task gen:paper`, `task native:game:exe`, `cargo build --release -p k3c-game` |
| Go-Bilder | in `native/k3c-game`: `K3C_GAME_EXE=<repo>/native/k3c-game/bin/k3c-game.exe K3C_E2E_OUT=<repo>/.work/e2e/go go test ./e2e -run TestFlowImages` |
| Rust-Bilder | `K3C_RUST_EXE=<repo>/rust/target/release/k3c-game.exe K3C_E2E_OUT=<repo>/.work/e2e cargo test -p k3c-game --test e2e_images` |
| Vergleich | `K3C_E2E_OUT=<repo>/.work/e2e cargo test -p k3c-game --test e2e_compare -- --nocapture` |

Ausgabe unter `.work/e2e/` (nicht eingecheckt): `go/{a,b}`, `rust/{a,b}` mit `shot-NN.png` und `stationen.txt`, `vergleich.json`,
Bildpaare `paare/NN-<station>.png` (links Go, Mitte Rust, rechts Differenz: Pixel über der Toleranz rot auf abgedunkeltem Go-Bild).

## Kennzahlen und Schwellen

Formel und Schwellen wie der Standbild-Vergleich `rust/crates/k3c-game/tests/still.rs` (RU13.6), damit beide gleich bewertet werden:

- **Mittel:** mittlere absolute Abweichung je Kanal (RGB, 0 bis 255) über das ganze Bild, Schwelle 0,5.
- **Anteil:** Anteil der Pixel, bei denen ein Kanal um mehr als 24 abweicht, Schwelle 0,005.
- Die Schwellen werden nie angehoben, um eine Station grün zu machen. Der Test schlägt bei einer Station über der Schwelle
  nicht fehl, sondern schreibt `vergleich.json`; je Station darüber entsteht ein Ticket im zuständigen RU-Sprint.
  (Die Spec nannte die Pixeltoleranz 32; maßgeblich ist die Vorgabe „dieselbe Formel wie RU13.6“, also 24.)

## Stationen

Takte und Namen stammen aus der Go-Referenz. Spalten Mittel, Anteil, Ergebnis und Ticket füllt die Abnahme (B-1120).

| Nr. | Station | Takt | Mittel | Anteil | Ergebnis | Ticket |
|---|---|---|---|---|---|---|
| 01 | hauptmenue | 1 | – | – | offen | – |
| 02 | lobby | 20 | – | – | offen | – |
| 03 | spielstart | 21 | – | – | offen | – |
| 04 | muenzen | 299 | – | – | offen | – |
| 05 | skillpunkt | 535 | – | – | offen | – |
| 06 | skillmenue | 559 | – | – | offen | – |
| 07 | gebaut | 823 | – | – | offen | – |
| 08 | nacht | 15539 | – | – | offen | – |
| 09 | pause | 15901 | – | – | offen | – |
| 10 | verlassen | 15913 | – | – | offen | – |
| 11 | weiter | 15917 | – | – | offen | – |
| 12 | zu-zweit | 15921 | – | – | offen | – |
| 13 | niederlage | 41309 | – | – | offen | – |
| 14 | ende | 41401 | – | – | offen | – |

## Vorab bekannt (ungeprüft für die Bilder)

- Welt-Hash gegen Go (`tests/e2e_flow.rs`, gemessen auf dem Hauptrechner): gleich an `hauptmenue`, `lobby`, `spielstart`,
  `muenzen`, `skillpunkt`, `skillmenue`, `gebaut`, `nacht`, `pause`, `weiter`, `zu-zweit`, `niederlage`, `verlassen`, `ende`
  (alle 14 Stationen, seit B-1106 ohne Ausnahmeliste). Takte und Ereignisse aller 14 Stationen sind gleich Go.
- Das Licht (Lichtkarte) zeichnet Rust nicht (B-1056); Nachtstationen weichen darum voraussichtlich über der Schwelle ab.
