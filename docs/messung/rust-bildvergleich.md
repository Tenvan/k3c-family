# Ende-zu-Ende-Bildvergleich Go gegen Rust (RU32)

Stand 2026-11-19 (STIRA-PC, `origin/main` 0d4096d6, RU64.1): **Messwerte gemessen, alle 14 Stationen über der Schwelle** (Abnahme-Ticket B-1120; der Hauptrechner nimmt
keine Bilder auf, B-742). Go-Seite: gesicherte Referenz `testdata/rust/e2e/go/a/` (Go-EXE entfällt seit RU58). Details und Befehle: `docs/messung/abnahme-b-1120.md`. Dieser Bericht ersetzt nicht die Sichtprüfung durch 🧑 (RU36).

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
| 01 | hauptmenue | 1 | 11,143 | 0,1390 | über Schwelle | – |
| 02 | lobby | 20 | 3,358 | 0,0564 | über Schwelle | – |
| 03 | spielstart | 21 | 1,911 | 0,0128 | über Schwelle | – |
| 04 | muenzen | 299 | 9,080 | 0,1052 | über Schwelle | – |
| 05 | skillpunkt | 535 | 7,417 | 0,0835 | über Schwelle | – |
| 06 | skillmenue | 559 | 7,792 | 0,0879 | über Schwelle | – |
| 07 | gebaut | 823 | 7,393 | 0,0903 | über Schwelle | – |
| 08 | nacht | 15539 | 3,644 | 0,0259 | über Schwelle | – |
| 09 | pause | 15901 | 3,297 | 0,0488 | über Schwelle | – |
| 10 | verlassen | 15913 | 11,277 | 0,1396 | über Schwelle | – |
| 11 | weiter | 15917 | 1,987 | 0,0150 | über Schwelle | – |
| 12 | zu-zweit | 15921 | 2,677 | 0,0194 | über Schwelle | – |
| 13 | niederlage | 41309 | 5,129 | 0,0365 | über Schwelle | – |
| 14 | ende | 41401 | 11,671 | 0,1462 | über Schwelle | – |

## Vorab bekannt (ungeprüft für die Bilder)

- Welt-Hash gegen Go (`tests/e2e_flow.rs`, gemessen auf dem Hauptrechner): gleich an `hauptmenue`, `lobby`, `spielstart`,
  `muenzen`, `skillpunkt`, `skillmenue`, `gebaut`, `nacht`, `pause`, `weiter`, `zu-zweit`, `niederlage`, `verlassen`, `ende`
  (alle 14 Stationen, seit B-1106 ohne Ausnahmeliste). Takte und Ereignisse aller 14 Stationen sind gleich Go.
- Das Licht (Lichtkarte) zeichnet Rust nicht (B-1056); Nachtstationen weichen darum voraussichtlich über der Schwelle ab.

## Standbild-Vergleich `still.rs` (RU51.8, B-1108)

Stand 2026-11-19, mit gerenderter Grafik (`task gen:paper`). Ursache der roten Tests `bildvergleich_an_fuenf_staenden` und
`lichtkarte_fehlt_in_rust`: RU55 hat die Lichtkarte (Tönung über die Lichtkarte) in Rust eingeführt (B-1056); der Test
verglich weiter mit dem Go-Bild **ohne** Lichtkarte (`*.go-nolight.png`) und erwartete, dass Rust sie nicht zeichnet. Korrektur
im Test, nicht im Zeichner und **ohne die Schwellen zu ändern** (Mittel 0,5, Anteil 0,005, Pixeltoleranz 24): Der Vergleich nimmt
jetzt das vollständige Go-Bild `*.go.png`; `lichtkarte_fehlt_in_rust` entfällt (sein Text verlangte genau diese Umstellung).
Ergebnis: `cargo test --release -p k3c-game --test still` 4 grün; die fünf Stände liegen unter den Schwellen (Rust zeichnet
Lichtkarte wie Go).

Offen, nicht in den erlaubten Dateien dieser Session: `tests/still_hud.rs` (`plaetze_stimmen_mit_go_ueberein`: Element 13
(Radar) erscheint in `tag-allein` Zelle 0 zusätzlich; `bilder_stimmen_mit_go_ueberein`: vier Stände über der Schwelle,
`englisch-hub` Element 3 (Uhr) Boxfarbe Rust [60, 90, 130] gegen Go [27, 41, 59]). Bisektion: rot schon auf dem Commit, der
die Referenz schrieb (`bb03554c`, RU23), und ebenso auf RU32, RU53, RU54, RU55; keine dieser Änderungen ist die Ursache.
Vermutung (ungeprüft): die Referenz hängt von der lokal gerenderten Grafik oder der Schrift ab.
