# RU9.1 · menu Teil 1: Kern, Layout, Zeichner

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU7, RU39
- **Tickets:** B-1014
- **Kriterien:** AC-02, AC-03

## Ziel

Der Kern des Pakets `menu` (gemeinsamer Zustand, Seiten-Maße, Zeichner für Karten und Rückfrage) rechnet und zeichnet in Rust wie in Go; die Hilfen für alle weiteren Menü-Tests stehen bereit.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst lesen. Quellen in `native/k3c-game/menu/`: `menu.go` (102), `layout.go` (91), `draw.go` (255); Hilfen der Tests: `fake_test.go` (154), dazu aus `paper_test.go` die Tests `TestColorsMatchStilPaper` und `TestCardsInWindowWithoutOverlap` (liest `public/grafik/stil.paper.json`, Pfad ab `env!("CARGO_MANIFEST_DIR")` nach `../../../public/grafik/stil.paper.json`) und aus `keys_test.go` `TestGeomFitsWindow`. Ziel: `rust/crates/k3c-game/src/menu/` mit `mod.rs` (aus `menu.go`), `layout.rs`, `draw.rs`, `paper_tests.rs` (die zwei Tests aus `paper_test.go`), `layout_tests.rs` (`geom_fits_window`), `testutil.rs` (aus `fake_test.go`: `FakeSession`, `press`, `step_through`, `has_cue`, `save`, Geräte `keys1`, `pad1` … als Funktionen/Konstanten, `fake_session_fulfils_contract` als Test). `pending.rs` und `version.rs` legt diese Session an, `menu/mod.rs` bindet alle `mod`-Zeilen der Sprints RU9 und RU12 ein, die Dateien dazu folgen in den späteren Sessions (nicht vorhandene `mod`-Zeilen erst mit der Session, die die Datei liefert). In `lib.rs` kommt nur `pub mod menu;` hinzu.

**Vorentscheidungen** (gelten für alle Sessions von RU9 und RU12):
- Farben: Go `color.RGBA` (vormultipliziert, `uint8`) wird `pub type Rgba = [u8; 4]`; an `platform::Canvas` geht `c.map(|v| f32::from(v) / 255.0)` wie in `art/font.rs` (`Font::draw` nimmt dasselbe `[u8; 4]`). `rgb(v: u32) -> Rgba` und `fade(c, a) -> Rgba` wie in Go (`uint8(f64 * a)` schneidet ab: `as u8`).
- Gemeinsamer Zustand `menus` wird `pub struct Menus` mit den Feldern von Go (`paint`, `keys: KeyTable`, `seats: [Seat; MAX_PLAYERS]`, `n_seats`, `notice`, `worlds: art::WorldImages`, `usable`, `has` als `Option<Box<dyn Fn(&str) -> bool>>`). Die Screens halten `Shared = Rc<RefCell<Menus>>`: `view::Screen::draw` nimmt `&self`, `art::Font::draw` aber `&mut self` und der Painter cachet Bilder; `Rc` und `RefCell` sind nur in den Sim-Crates verboten (Leitfaden § 4), im Client erlaubt; die Review-Session führt sie auf. Eine Borrow-Dauer reicht nie über einen Aufruf von `update` oder einem anderen Screen.
- `Library` besitzt die Schrift (`lib.font`) und die Sheets. `menu::new(lib: Option<Rc<RefCell<art::Library>>>, keys: KeyTable) -> Box<dyn view::Menus>`; ohne Library die Pixel-Schrift über `art::font::load_font(Path::new(""), false)` (fehlende Datei, nicht strikt = `fallback`). `impl view::Menus` und `impl view::MenuPages` (Rust `Menus::main(&mut self)`, `page(name, in_game, by)`) wie in `menu.go`; die Screens `new_main`, `new_pause`, `new_saves`, `new_options`, `new_lobby`, `new_keys`, `new_licenses` kommen aus den späteren Sessions: bis dahin Platzhalter in `menu/pending.rs` mit derselben Signatur (Leitfaden § 7, `unimplemented!("RU9: new_main")`; RU9.2 bis RU9.5 und RU12 löschen sie nach und nach).
- Zeichner: Go `painter` mit `ebiten.Image` (`pixel`, `round`, `scratch`) entfällt als Bild-Zwischenspeicher; `rect` → `canvas.rect(Rect, tint(c))`, `text` → `Font::draw` (die Pixel-Schrift zeichnet `Font::draw` selbst, `scratch` und `text_scale` bleiben nur als reine Rechnung `text_scale(size)` für `measure`), `disc` → `canvas.fill_circle` (siehe RU9.2). `Rect` ist `k3c_platform::Rect`, Fensterrechtecke sind `view::cell::Rectangle` (Ganzzahl, `dx()`, `dy()`).
- Zustand von `version`: Go `var Version string` (per `-ldflags -X` gesetzt, `licenses.go`) braucht schon `main_screen` (RU9.4). Diese Session legt dafür `menu/version.rs` an (`static VERSION: RwLock<String>`, `pub fn version() -> String`, `pub fn set_version(&str)`; der Build ruft später `set_version` mit `option_env!("K3C_VERSION")`); RU12.3 hängt `licenses.rs` daran. Das ist die einzige Datei ohne Go-Gegenstück.

Tests, die einen Screen aus einer späteren Session brauchen, stehen mit `#[ignore = "wartet auf RU9.4"]` (bzw. RU12) da und werden von dieser Session aktiviert (Leitfaden § 6): `TestFakeSessionFulfilsContract` braucht nichts und läuft sofort. `TestMenusPagesAndFallbackFont` (steht in `pause_test.go`) gehört RU12.3.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/src/lib.rs` (nur `pub mod menu;`)
- `rust/crates/k3c-game/Cargo.toml` (nur Abhängigkeiten auf vorhandene Workspace-Crates)

## Nicht-Ziele

Änderungen an Go-Code. Die Screens selbst (RU9.2 bis RU9.5, RU12). Texte (`texts.go`, RU9.3). Hintergrund und Illustrationen (RU9.2). Neue Crates. Verbesserungen über den Go-Stand hinaus.

## Schritte

1. Leitfaden, `menu.go`, `layout.go`, `draw.go`, `fake_test.go` und die drei genannten Tests ganz lesen; in `rust/crates/k3c-game/src/view/` (`screen.rs`, `session.rs`, `cell.rs`, `settings.rs`) und `art/font.rs` die Rust-Verträge ansehen.
2. `menu/mod.rs`, `pending.rs`, `version.rs` anlegen, `lib.rs` ergänzen, `cargo build -p k3c-game` in `rust/`.
3. `layout.rs` (reine Rechnung, `geom_of`, `Material`, Farb-Konstanten als `const fn`/`static`), dann `draw.rs` (`Page`, `Row`, `Hero`, `Painter` mit `page`, `card`, `style_of`, `paper`, `ask_box`) und `mod.rs` (`Menus`, `new`, `main`, `pause`, `page`, `sync`, `seat_of`) portieren; die Reihenfolge der Zeichenbefehle (Schatten, Rahmen, Papier, Kante, Schild, Text) bleibt die von Go.
4. `testutil.rs`, `paper_tests.rs`, `layout_tests.rs` portieren, Namen in `snake_case`.
5. `cargo test -p k3c-game menu`, `task rust:lint`.
6. Ergebnis mit `plan_section` eintragen (auch: welche Tests mit `ignore` stehen), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-03 (Teil menu, layout, draw): `fake_session_fulfils_contract`, `colors_match_stil_paper`, `cards_in_window_without_overlap`, `geom_fits_window` portiert und grün (`cargo test -p k3c-game menu`); Zeichen-Reihenfolge wie Go.
- [ ] AC-02 (Teil Zeichner): `style_of`, `paper`, `card` und `ask_box` zeichnen Karte, Auswahl-Rahmen und Rückfrage wie Go (Bildvergleich folgt in RU9.6).
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
