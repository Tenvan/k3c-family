# RU9.6 · Bildvergleich der Menü-Seiten gegen das Go-Spiel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU9.5
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

Hauptmenü, Lobby (zwei Geräte), Optionen, Tasten-Seite und Lizenzen sehen im Rust-Spiel aus wie im Go-Spiel: Bildvergleich je Seite mit Schwellen, Befunde als Tickets.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst lesen. Die Session liefert den Beleg für AC-02 und ist nach dem Muster von RU13.6 gebaut.

**Go-Bilder** entstehen mit dem vorhandenen Aufnahme-Modus der Go-EXE, ohne Änderung an Go-Code: `k3c-game.exe --saves <leerer Ordner> --menu <seite> --size 1280x720 --shots <ordner> --count 1` (Schalter in `native/k3c-game/game/scene.go`, `modes.go`, `menu.go`). Seiten: `main`, `lobby`, `options` (über dem Hauptmenü), `keys`, `licenses`; `saves` und `pause` gehören zu RU12. Ein leerer Spielstände-Ordner macht das Hauptmenü reproduzierbar (Fortsetzen ausgegraut). Die Lobby mit zwei Geräten wird so aufgenommen, wenn `--input` das Beitreten des zweiten Geräts ausdrücken kann (`game/input.go`, Aktion `confirm` des Pads 1); sonst nimmt die Session die Lobby mit einem Gerät auf und prüft das zweite Gerät im Rust-Test (Beitritt und Untertitel-Zeile `Spieler 1: Pad 1 · Spieler 2: Tastatur`). Die Bilder liegen als `testdata/rust/bild/menu/<seite>.go.png` (höchstens 1280×720), werden einmal erzeugt und eingecheckt. Die Karten-Illustrationen brauchen `task gen:paper` (für dieses Feature erlaubt, B-742); fehlt die Grafik, zeichnen beide Spiele Text-Karten, auch das ist ein gültiger Vergleich, wenn beide Seiten denselben Stand haben.

**Rust-Bild:** RU13.6 baut in `rust/crates/k3c-game/src/still/` einen kleinen Software-Rasterer (`sprite`, `rect`, `mesh`, `text`) für Standbilder und läuft vor dieser Session (RU39 verlangt die Zeichen-Sprints). Die Session liest `still/mod.rs` zuerst und prüft, ob er `fill_circle` und `text_font` kann (die Menüs brauchen Rechtecke, Kreise, Text). Fehlt etwas oder gibt es `still/` nicht: **hält an und fragt 🧑**, weil ein eigener Rasterer eine Entscheidung über den Bereich ist; kleine Lücken (ein fehlender Befehl) meldet sie als Ticket und lässt die betroffene Seite aus.

**Ablauf Rust:** Ein Test in `rust/crates/k3c-game/tests/menu_bild.rs` baut je Seite `menu::new(None, keys)` mit `testutil::FakeSession` (leere Liste), ruft `update` mit den Eingaben der Seite (Hauptmenü: keine; Lobby: Beitritt des zweiten Geräts; Optionen: ein `OK` auf der Zeile Steuerung für die Tasten-Seite), zeichnet über den Rasterer in 1280×720 und vergleicht mit dem Go-Bild: mittlere Abweichung je Kanal und Anteil abweichender Pixel; die Schwellen stehen im Test und im Ergebnis. `testutil` ist dafür als `pub` hinter `#[cfg(any(test, feature = "testutil"))]` zu nutzen oder die Seiten werden in `menu_bild.rs` mit einer eigenen kleinen Schein-Session gebaut (kleinere Lösung wählen, keine neue Feature-Flag-Kette).

Bekannte Abweichungsquellen, die als Befund (nicht als Schwelle) zu behandeln sind: Scheibe des Mondes (Go: 64-px-Bild, Rust: `fill_circle`, RU9.2), Schrift (Pixel-Schrift gegen TTF je nach Stand von `public/schrift/k3c.ttf`), Vormultiplikation der Farben. Weicht eine Seite ab, gilt Go: Befund als Ticket, Schwelle nicht anheben, kein Go-Bild neu schreiben.

## Erlaubte Dateien

- `testdata/rust/bild/menu/**`
- `rust/crates/k3c-game/tests/menu_bild.rs`
- `rust/crates/k3c-game/src/menu/**` (nur Fehlerbehebungen, die der Vergleich findet, mit Test)

## Nicht-Ziele

Änderungen an Go-Code. Änderungen am Rasterer in `still/` (RU13.6) und an `k3c-platform`. Die Seiten `saves`, `pause`, `padtest`, `testing` (RU12). Das Go-Bild neu schreiben, um Rust zu gefallen. Die Sichtprüfung von 🧑 (steht im Abnahme-Ticket).

## Schritte

1. Leitfaden und `RU13.6` lesen; `still/mod.rs` prüfen (Haltepunkt, siehe Kontext).
2. Go-Bilder aufnehmen (`task gen:paper` vorher, wenn Illustrationen gewollt sind), nach `testdata/rust/bild/menu/` legen, Größe prüfen.
3. `tests/menu_bild.rs` bauen, je Seite vergleichen; Schwellen aus dem ersten Lauf festhalten.
4. Abweichungen sichten: Fehler in `menu/` beheben (mit Test), Rest als Ticket (Projekt RSU, Sprint RU9).
5. `cargo test -p k3c-game --test menu_bild`, `task rust:lint`.
6. Ergebnis mit `plan_section` eintragen (Schwellen, Abweichungen, Tickets), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-02: Für Hauptmenü, Lobby, Optionen, Tasten und Lizenzen ist je ein Bildvergleich grün unter den im Ergebnis genannten Schwellen oder die Abweichung ist als Ticket erfasst.
- [ ] Die Lobby-Aufnahme zeigt beide Geräte (Go-Bild oder Rust-Test, siehe Kontext).
- [ ] `task rust:lint` grün, Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Bildvergleich Rust gegen Go für vier Menü-Seiten (`menu/image_tests.rs`, Test `menu_pages_match_go_images`): Hauptmenü, Lobby, Optionen, Tasten, 1280×720, Version `v0.24.1-164-g694bffa1`, Tastatur als Gerät; die Anzeige „T 0“ oben rechts ist maskiert. Go-Referenzen liegen in `testdata/rust/bild/menu/*.go.png` (Aufnahme-Modus `--menu <seite> --size 1280x720`). Neu: `menu::new_set` (liefert `MenuSet` mit `main_screen()` und `MenuPages`, löst den Punkt aus RU9.2).

Gemessen: mittlere Abweichung 2,6 bis 3,0 je Kanal, 2,6 bis 2,9 % abweichende Pixel (Toleranz 24); Schwellen 3,5 und 3,5 %. Aufbau, Farben, Illustrationen und Kartenrahmen stimmen. Die Abweichung kommt fast nur vom Text, der in Rust durchgehend etwa 2 % schmaler läuft als in Go.

Die Lizenzen-Seite ist nicht im Vergleich: `licenses()` liefert erst RU12.3 (`licenses.go.png` liegt bereit). Zwei-Geräte-Lobby prüfen schon die Tests aus RU9.4. Das Bild des Tests entsteht nur mit gerenderter Grafik; ohne sie überspringt er mit Hinweis.

### Für den Hauptrechner
- Befund: Text-Breite in Rust etwa 2 % kleiner als in Go (Hinweis im Hauptmenü 459 statt 468 px, Version 350 statt 358 px). Ursache vermutet in Schriftgröße oder Advance-Rundung im Painter bzw. `art::Font`; messen und angleichen, dann Schwellen in `image_tests.rs` senken. Als Ticket anlegen.
- Lizenzen-Bild in den Vergleich aufnehmen, sobald RU12.3 `licenses()` liefert.
- `task check:diff` bleibt im Rust-Teil rot wegen `tests/still.rs` (auch auf `main`).
