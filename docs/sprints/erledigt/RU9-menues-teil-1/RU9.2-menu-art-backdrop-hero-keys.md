# RU9.2 · menu Teil 2: Illustrationen, Hintergrund, Held, Tasten

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU9.1
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Illustrationen der Karten, Papier-Hintergrund, Welt-Held und die gemeinsame Tasten-Logik der Menüs (`run`, Meldung, Auswahl-Hilfen) laufen in Rust wie in Go; B und Esc gehen zurück.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst lesen, dann die Vorentscheidungen in `RU9.1` (Farben, `Shared`, `Menus`, `pending.rs`). Quellen in `native/k3c-game/menu/`: `art.go` (130, Test `art_test.go`), `backdrop.go` (132), `hero.go` (31), `keys.go` (183, Test `keys_test.go`), dazu aus `paper_test.go` `TestBackdropInWindow` und `TestBackdropSwitchesFilterAtMostOnce`. Ziel in `rust/crates/k3c-game/src/menu/`: `art.rs`, `art_tests.rs`, `backdrop.rs`, `hero.rs`, `keys.rs`, `keys_tests.rs`; die Backdrop-Tests kommen nach `paper_tests.rs` (von RU9.1 angelegt). Aus `keys_test.go` gehören `TestTextsComplete` zu RU9.3 und `TestGeomFitsWindow` zu RU9.1; hier `move`, `window_and_cycle`, `key_of`, `run_remembers_operating_device`.

Rust-Abhängigkeiten: `art::WorldImages` (`art/worldimages.rs`, `from_data()` statt `art.DataWorldImages()`), `art::Library::sheet`/`usable` (`art/library.rs`), `art::Sheet::draw(dst, i, Affine)` und `frame(i)` (`art/sheet.rs`), `view::input::Action` und `DeviceInput`. `texts` (`TextKey`, `tr`) liefert erst RU9.3: `keys.rs` braucht nur `TextKey` und `tr`/`tr1`; bis RU9.3 stehen in `pending.rs` ein Platzhalter-`enum TextKey` mit den Varianten, die `keys.rs` nennt (`deny`, `inform`), und `fn tr(lang, k) -> String` (`unimplemented!("RU9.3: tr")`); RU9.3 löscht beides.

**Vorentscheidungen:**
- `handler`-Schnittstelle: Go bettet `base` in jeden Screen ein und ruft `b.run(in, s, h)` mit `h` = derselbe Screen. In Rust ein Trait `Handler { fn base(&mut self) -> &mut Base; fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>); }` und eine freie Funktion `run<H: Handler>(h: &mut H, input: &Input, s: &mut dyn Session) -> Step` mit derselben Schleife wie `base.run` (Tasten aller Geräte in Druck-Reihenfolge; ein zweites `Back` desselben Geräts im selben Update zählt nicht). `Key`-Aufzählung: `None, Up, Down, Left, Right, OK, Back, Home, Delete` wie Go. **`key_of`: `Action::Back` und `Action::Menu` ergeben `Key::Back`, `Action::Home` ergibt `Key::Home`** (B am Pad, Esc und Backspace an der Tastatur; so geht B in jedem Menü zurück).
- Backdrop: `shape.filter()` bleibt (`k3c_platform::Filter::{Linear, Nearest}`) und der Test `backdrop_switches_filter_at_most_once` prüft die Reihenfolge der Formen wie in Go. Gezeichnet wird jede Scheibe mit `canvas.fill_circle(x, y, r, tint(c))` (Go zeichnete ein 64-px-Scheibenbild mit Linear-Filter; `discPx`/`round` entfallen, der Unterschied am Rand fällt im Bildvergleich RU9.6 auf, nicht anheben, sondern als Befund melden).
- `art.rs`: `frame_of` liefert `Option<(TexId, Rect)>` des ersten Bilds (`sheet.frame(0)`), zuerst Animation `idle`, dann `stand`; `art_box` zeichnet über `canvas.sprite` mit denselben Maßen (eingepasst, unten mittig, Faktor `min(w/iw, h/ih)`), Filter linear ist der Standard der Library-Texturen. `Art(lib)` aus Go wird `pub fn art(usable: &dyn Fn(&str) -> bool) -> Vec<String>`; `menus.Assets` entfällt (Go-`game` ruft es, der Rust-Client entscheidet in RU26 selbst; dann Ticket, falls die Methode im Vertrag `view::Menus` fehlt).
- `hero.rs`: `Painter::hero` und `hero_rect` aus `hero.go` (31 Zeilen), gezeichnet mit `art_box` und dem Namen über `centered`.

B und Esc: Das Go-Spiel prüft das für jeden Screen in dessen Test (siehe RU9.3 bis RU9.5, RU12); hier genügt `key_of_maps_back_menu_home` (Ergänzung zu `key_of`).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`

## Nicht-Ziele

Änderungen an Go-Code. `texts.go` (RU9.3), Screens (RU9.3 bis RU9.5). Neue Crates. Optimierungen über den Go-Stand hinaus (zum Beispiel ein Cache für die Formen des Hintergrunds).

## Schritte

1. Leitfaden, die vier Go-Dateien, `art_test.go`, `keys_test.go` und die zwei Tests aus `paper_test.go` ganz lesen.
2. `keys.rs` zuerst (Trait `Handler`, `run`), dann `backdrop.rs`, `hero.rs`, `art.rs`; nach jeder Datei `cargo build -p k3c-game`; `mod`-Zeilen ergänzen, Platzhalter aus `pending.rs` entfernen, die jetzt geliefert werden.
3. Tests portieren (`art_tests.rs`, `keys_tests.rs`, Backdrop-Tests in `paper_tests.rs`), Namen in `snake_case`.
4. `cargo test -p k3c-game menu`, `task rust:lint`.
5. Ergebnis mit `plan_section` eintragen, Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-03 (Teil art, backdrop, hero, keys): `art_tests`, `keys_tests` und die zwei Backdrop-Tests portiert und grün (`cargo test -p k3c-game menu`).
- [ ] AC-01 (Teil Tasten): B (`Action::Back`), Menu und Esc ergeben `Key::Back`, Home ergibt `Key::Home`; ein Test nennt beide Gerätearten (Tastatur, Pad) und ein zweites Back im selben Update wird verworfen.
- [ ] AC-02 (Teil Hintergrund): Hintergrund-Formen in Reihenfolge wie Go (erst Rechtecke, dann Scheiben), Filter wechselt höchstens einmal.
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

AC-01 geprüft: `key_of` bildet B (Back), Menu und Esc auf `Key::Back`, Home auf `Key::Home` ab; `key_of_maps_back_menu_home` nennt Pad und Tastatur, ein zweites Back desselben Geräts im selben Update wird verworfen, je Gerät zählt eins.
AC-02 geprüft: Hintergrund-Formen erst Rechtecke, dann Scheiben, Filter wechselt höchstens einmal (`backdrop_switches_filter_at_most_once`, `backdrop_in_window`).
AC-03 geprüft: `art_tests` (2), `keys_tests` (5), Backdrop-Tests (2) grün, `cargo test -p k3c-game menu`; Fmt und Clippy `-D warnings` grün. Review k3c-reviewer: Befund `tint` (Farben aus RU9.1 doppelt vormultipliziert) behoben: `menu::draw::tint` rechnet wie `fx::paint::tint` auf gerade Farbe zurück.
Ignorierte Tests: `menu_rows_carry_art` (wartet auf RU9.4, braucht `new_main`), `run_remembers_operating_device` (wartet auf RU9.4).
Dateien: `art.rs`, `backdrop.rs`, `hero.rs`, `keys.rs`, `art_tests.rs`, `keys_tests.rs`, Backdrop-Tests in `paper_tests.rs`; `pending.rs` ohne Backdrop, Hero, Art, `COL_SHADOW`, `Note` (jetzt in `keys.rs`), dafür `TXT_PAD`, `TXT_KEYBOARD`, `tr`, `tr1` (RU9.3).
Abweichungen: `Art(lib)` ist `art(usable)`, `menus.Assets` ist `Menus::assets()`; Scheiben über `fill_circle` (Rand im Bildvergleich RU9.6 prüfen). `Menus.has` bleibt die eigene Closure aus RU9.1 (Umstellung auf `Painter::has_sheet` nicht nötig).
`task check:diff`: Rust-Test-Stufe rot nur durch `tests/still.rs`, auch auf `main` rot (siehe RU9.1).

### Für den Hauptrechner
- `keys.rs`: `run` gibt `cues` per `mem::take` ab (Kapazität geht verloren, Go nutzt `[:0]`); nur eine kleine Allokation bei Eingaben, bei Bedarf in RU9.7/RU22 anschauen.
- Wer `deny`, `device_name` oder `Note::text` vor RU9.3 ruft, löst einen Panic aus (`tr` ist Platzhalter); aktuell ruft sie niemand.
