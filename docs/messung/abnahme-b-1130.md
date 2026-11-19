# Abnahme B-1130 Teil 1: Szenen, Pixelmessung, Profil go (RU64.5)

2026-11-19, STIRA-PC, Second Worker, Commit eb9784dc. Teil 2 (Frame-Zeit, `alloc_je_bild`, Workspace-Tests): RU64.6. Szenen und Pixelwerte: `docs/messung/look-k2c.md`.

## Befehle und Ergebnis

| Lauf | Befehl | Ergebnis |
|---|---|---|
| Aufnahmen k2c | `K3C_LOOK=k2c k3c-game.exe --seed 1 --players N --input … --tick T --shots … --size 1920x1080` (Tag, Abend, Nacht, Morgengrauen, 1 und 2 Ansichten) | 8 Aufnahmen, Messung in `look-k2c.md` |
| Bildaufnahme Profil go | `K3C_LOOK=go K3C_RUST_EXE=… cargo test --release -p k3c-game --test e2e_images` | grün, 14 s, Lauf a und b pixelgleich |
| Bildvergleich Profil go | `K3C_LOOK=go … --test e2e_compare -- --nocapture` | Test grün (meldet nur), **14 von 14 Stationen über der Schwelle**, Werte wie `docs/messung/rust-bildvergleich.md` (Mittel 1,9 bis 11,7; Anteil 0,013 bis 0,146) |
| Zum Vergleich Profil k2c | `K3C_LOOK=k2c … e2e_images` und `e2e_compare` | 14 von 14 über der Schwelle, deutlich schlechter in den Weltstationen: 04 muenzen 37,3, 05 35,8, 06 32,4, 07 36,0, 08 nacht 15,1, 11 weiter 15,5, 12 15,4, 13 13,9 (Mittel) |
| testdata | `git status` | unverändert |

**Wichtig für B-1120:** Das Spiel baut `k3c-data` mit dem Feature `go-default` (`rust/crates/k3c-game/Cargo.toml`); ohne `K3C_LOOK` gilt also das Profil `go`. Die Zahlen in `rust-bildvergleich.md` (RU64.1) sind damit Profil `go`, wie hier bestätigt (gleiche Werte).

## Für den Hauptrechner

- AC-02 (Profil `go` grün) ist **nicht erfüllt**: alle 14 Stationen über der Schwelle, auch im Referenzprofil; k2c weicht in den Weltstationen noch stärker ab (erwartet, anderes Layout).
- Look k2c (Hub Tag, Abend, Nacht): Gehlinie 57,7 bis 57,8 %, Ufer 7,0 bis 7,6 %, Fluss 34,7 bis 35,1 % im Ziel. Spiegel-Helligkeit Tag 74 % (Ziel 60 bis 70 %); M (Reiterhöhe) nur gesichtet, ca. 15 % (Ziel 12 %), ungeprüft.
- Nicht aufnehmbar ohne Dev-Menü oder neuen Schalter: Blutmond, dichter Wald, gerodeter Hub, Lava-Stufe (Ticket-Vorschlag: `--dev-event`, `--phase` o. ä. für Aufnahmen, Entscheidung beim Hauptrechner).
- Morgengrauen: Kantenmessung unsicher (schwacher Kontrast); Messmethode für dunkle Szenen offen.

## Prüfliste für Ralf (AC-04, am TV)

Nacht, Blutmond, Wald und Hub wirken wie das K2C-Vorbild; Weite 4, 4,5 oder 5 M wählen (Default 5 M, B-1045).
