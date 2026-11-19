# RU61 · CLI, SIM, PAP · Rust-Spiel: Nachträge und Wünsche aus den Go-Sprints (CH1, BI1, BI2, NA1, WG2, FB4, SR1)

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI, SIM, PAP
- **Reife:** bereit
- **Tickets:** B-885, B-889, B-890, B-891, B-910, B-911, B-938, B-944, B-947, B-948, B-949, B-1006
- **Start-Commit:** – (wird von der ersten Session gesetzt: `git rev-parse --short origin/main`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Die Sprints CH1, BI1, BI2, NA1, WG2, FB4 und NG3 sind geschlossen, ihre Tickets (Anzeige, Eingabe, Look, Rig) blieben offen und zeigten auf das Go-Spiel, das mit RU37 entfallen ist. B-1143 hat sie hier zusammengeführt und auf `rust/crates/k3c-game` umgehängt.

**Vorbefund (Code gelesen, 2026-11-19):** Der Port aus Go hat fast alle Wünsche schon mitgenommen; dieser Sprint belegt sie mit benannten Tests und schließt nur echte Lücken.

| Ticket | Stand im Rust-Spiel | Beleg (Auswahl) |
|---|---|---|
| B-947 Mauern ohne Lücken | umgesetzt | `world/layers.rs` › `TILE_OVERLAP`; Test `tiles_have_no_gap` |
| B-948 Bäume größer | umgesetzt (1,5-fach in den Daten) | `data/paper.json` › `skalierung`; Test `data_scales_aus_daten`; Größe bleibt unter dem Richtwert, folgt aus BB1.3 |
| B-949 Ladebild | umgesetzt | `game/loadscreen.rs`; Test `loading_in_steps` |
| B-890 HUD Icon plus Zahl | umgesetzt | `hud/icons.rs`, `drawstock.rs`; Test `stock_parts_zeigt_icon_und_zahl` |
| B-911 Tasten-Icons | umgesetzt, Icons und Kappen vorhanden | `hud/glyphs.rs`; Test `jeder_glyph_schluessel_hat_ein_icon` |
| B-944 Pad-Test, Zuordnung | Testseite und Tabellen-Tests vorhanden; Gerätetest offen | `menu/padtest.rs`, `input/bindings_tests.rs` |
| B-1006 gehaltenes A | umgesetzt; Allokations-Test fehlt | `hud/seekarte_hold_tests.rs` › `held_a_does_not_pay` |
| B-938 Weltauswahl | umgesetzt | `menu/worldpick.rs`; Test `world_pick_is_first_step_of_new_game` |
| B-889 F10-Werte, Chronik, Bericht | umgesetzt | `devtools/values.rs`, `chronicle.rs`, `report.rs` |
| B-891 Biom-Eigenschaften | umgesetzt in Level, Sim, Daten, Editor; Golden erst nach RU41 | `k3c-level/src/nature_tests.rs`, `k3c-sim/src/island/nature_tests.rs` |
| B-910 Abnahme BI2 | offen (braucht gerenderte Grafik) | Session RU61.5 |
| B-885 Rig-Export Bosse | offen (Export steht auf `version` 1, Specs auf 3) | Session RU61.6 |

## Ziel

Die Nachträge und Wünsche aus den Go-Sprints sind im Rust-Spiel belegt oder, wo eine Lücke bleibt, umgesetzt.

**Am Ende sichtbar:** HUD-Icons mit Zahl statt Namen, Tasten-Icons, Weltauswahl im großen Bild, Debug-Werte (F10), Xbox-Testseite, Lader im Spielstil; Mauern ohne Lücken, Bäume in der Größe aus den Daten; Biom-Eigenschaften je Welt. Ralf sichtet das am PC mit den Aufnahmen aus RU61.5.

## Beteiligte und Zielgruppen

Second Worker setzt um (Lane zweit); 🧑 Ralf nimmt am PC ab; der Hauptrechner schreibt die Sessions vor dem Aktivieren (Reife Entwurf).

## Anforderungen

Sessions (Lane in Klammern; alle offline, Modell sonnet):

- RU61.1 Look belegen: Mauern (B-947), Bäume (B-948), Ladebild (B-949) (AC-01; hauptrechner)
- RU61.2 HUD und Eingabe belegen: Icons (B-890), Tasten-Icons (B-911), Pad-Test (B-944), gehaltenes A samt Allokations-Test (B-1006) (AC-02; hauptrechner)
- RU61.3 Weltauswahl (B-938), F10-Werte und Chronik (B-889) belegen, Abgleich mit dem Cheat-Fenster (AC-03; hauptrechner)
- RU61.4 Biom-Eigenschaften belegen und Vollständigkeits-Test ergänzen (B-891) (AC-04; hauptrechner, Domäne SIM)
- RU61.5 Abnahme-Läufe auf dem zweiten Rechner: `gen:paper`, Grafik-Golden, Bildtests mit Pflicht-Grafik, `check:race`, Aufnahmen (B-910) (AC-01 bis AC-04; zweit)
- RU61.6 Rig-Export der Bosse (B-885) (AC-05; hauptrechner, Domäne PAP)
- RU61.7 Review (AC-06; hauptrechner)

## Nicht-Ziele

Neue Mechaniken (Entscheidung 013: nur in Rust; B-891 ist schon umgesetzt). Sim- und Balance-Läufe. Das Go-Spiel. Neue Assets und Asset-Größen (BB1). Golden-Datei für B-891 (erst nach RU41).

## Regeln und Einschränkungen

Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013). B-910 ist das Abnahme-Ticket dieses Sprints (B-742); seine Läufe laufen als Session RU61.5 auf dem zweiten Rechner, die Review legt kein zweites Abnahme-Ticket an.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-game/src/**`, `rust/crates/k3c-game/tests/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/src/biome.rs`, `data/biomes/**`, `public/grafik/objekte/golden.json`, `public/rig/bosse/**`, `public/rig/soll/bosse/**`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/frontend/src/world/**`, `docs/rules/welten.md`
- **Abhängig von:** RU37, RU55, RU56
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Ein Ticket mit Beleg: Test benannt (`tiles_have_no_gap`), grün, im Ergebnis der Session eine Zeile je Kriterium. Ein Ticket mit Lücke: Test zuerst, dann die kleinste Änderung.

## Ausnahme- und Fehlerfälle

Ein Test, der ohne gerenderte Grafik überspringt, beweist nichts: Er gilt erst in RU61.5 (`K3C_STILL_REQUIRE=1`). Ein Befund ohne Platz in diesem Sprint wird im Ergebnis als Ticket-Vorschlag für den Hauptrechner genannt (Second Worker legt keine Tickets an).

## Akzeptanzkriterien

- **AC-01** B-947, B-948, B-949 sind im Rust-Spiel belegt oder umgesetzt.
- **AC-02** B-890, B-911, B-944, B-1006 sind im Rust-Spiel belegt oder umgesetzt.
- **AC-03** B-938 und B-889 sind im Rust-Spiel belegt oder umgesetzt.
- **AC-04** B-891 und B-910 sind umgesetzt und belegt.
- **AC-05** B-885 ist umgesetzt.
- **AC-06** Review: `task check`, `task check:go`, `task rust:test`, `task rust:lint` grün; Abnahme-Ticket B-910 benannt.

## Offene Fragen

Entschieden am 2026-11-19 mit Default (autonom, Claude; Ralf ändert, was ihm nicht gefällt):

- Abhängigkeit RU60 entfällt: RU61 hängt nur an RU37, RU55, RU56 (alle fertig); die Überlappung mit RU60 im Bereich `rust/crates/k3c-game/src/**` regelt `plan_waves` über den Bereich (nie in derselben Welle).
- Zuschnitt: Die Wünsche sind im Rust-Spiel schon umgesetzt; daher Beleg-Sessions statt Neubau. B-910 läuft als Session auf Lane `zweit` (Läufe brauchen gerenderte Grafik und `check:race`).
- B-891 bleibt in RU61 (Domäne SIM für die Session RU61.4), Golden-Datei erst nach RU41.
- Baumgröße (B-948): Der Faktor 1,5 bleibt; die Richtwerte liefern die neuen Baumvarianten aus BB1.3.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU61.1 | `RU61.1-look-beleg-mauern-baeume-lader.md` | Umsetzung | autonom | offen |
| RU61.2 | `RU61.2-hud-eingabe-beleg.md` | Umsetzung | autonom | offen |
| RU61.3 | `RU61.3-menue-debug-beleg.md` | Umsetzung | autonom | offen |
| RU61.4 | `RU61.4-biom-eigenschaften-beleg.md` | Umsetzung | autonom | offen |
| RU61.5 | `RU61.5-abnahme-laeufe-bi2-bild.md` | Umsetzung | autonom | offen |
| RU61.6 | `RU61.6-rig-export-bosse.md` | Umsetzung | autonom | offen |
| RU61.7 | `RU61.7-review.md` | Review | autonom | offen |

Sim und Balance laufen nicht in diesem Sprint. Gen, Grafik-Golden und `check:race` laufen als Session RU61.5 (Lane `zweit`), sie ist zugleich die Abnahme B-910 (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
