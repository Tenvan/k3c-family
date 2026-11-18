# RU9.3 · menu Teil 3: Tabellen-Seiten und Texte

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU9.2
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Texte in zwei Sprachen und die Tabellen-Seiten Tasten und Lizenzen laufen in Rust wie in Go; B, A, Esc und Home schließen die Tabelle.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst lesen, dann die Vorentscheidungen in `RU9.1` und `RU9.2`. Quellen in `native/k3c-game/menu/`: `table.go` (194, Test `table_test.go`) und `texts.go` (298). Ziel in `rust/crates/k3c-game/src/menu/`: `table.rs`, `table_tests.rs`, `texts.rs`; aus `keys_test.go` kommt `TestTextsComplete` nach `texts_tests.rs`. Aus `table_test.go` gehören `TestKeysPage`, `TestLicensesPage`, `TestTableGeometry` hierher, `TestMainShowsVersion` und `TestMainFirstDrawAfterLeave` prüfen das Hauptmenü und stehen bis RU9.4 mit `#[ignore = "wartet auf RU9.4"]` in `table_tests.rs` (RU9.4 hebt das auf).

**Vorentscheidungen:**
- `texts.rs`: Go `textKey int` mit `iota`-Konstanten und `texts [textCount][2]string` wird `#[repr(usize)] pub enum TextKey` (Varianten in derselben Reihenfolge, Namen in `CamelCase` ohne `txt`-Vorsatz, z. B. `txtContinueSub` → `ContinueSub`) und `static TEXTS: [[&str; 2]; TEXT_COUNT]` mit `pub const TEXT_COUNT: usize`; `tr(lang, k)` und `tr1(lang, k, arg)` (ersetzt das erste `{}`) wie Go; `lang_index` kennt nur `view::settings::LANGUAGE_ENGLISH`, alles andere ist Deutsch. Die Texte bleiben wörtlich (Umlaute, `·`, `×`). Platzhalter `TextKey` und `tr` aus `pending.rs` (RU9.2) werden gelöscht.
- Die Datei hat in Go 298 Zeilen; passt sie mit den Rust-Zeilenumbrüchen nicht in 400 Zeilen, an derselben Stelle teilen, an der Go einen Abschnittskommentar hat (`texts.rs` + `texts_more.rs`), nicht kürzen.
- `table.rs`: `table`, `tableKind`, `tableScreen` mit `Handler` (RU9.2): `Back`, `Home` und `OK` ergeben `StepKind::Pop` mit Cue `Back`, alles andere `Stay` (**B und Esc gehen zurück**). `fill_licenses` liest die Liste `licenses` und `version_text` aus `licenses.go` (RU12.3): bis dahin Platzhalter in `pending.rs` (`pub fn licenses() -> &'static [License]`, `pub fn version_text(lang: &str) -> String`, je `unimplemented!("RU12: …")`); `table_tests::licenses_page` steht bis RU12.3 mit `#[ignore = "wartet auf RU12"]`. Die Tasten-Seite bekommt ihre Zeilen aus `Menus::keys` (`KeyTable`, `Vec<KeyRow>`).
- `Painter::table` und `fit_table` zeichnen über `Font::measure` und `Font::draw` und `canvas.rect`, Reihenfolge wie Go; `key_action_text` ist eine `match`-Tabelle statt `map`.
- Der Hauptmenü-Teil von `table_test.go` ruft `Version` (Go-Variable): in Rust `menu::version::set_version` (RU9.1).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`

## Nicht-Ziele

Änderungen an Go-Code. `licenses.go` (RU12.3). Hauptmenü (RU9.4). Übersetzungen verbessern oder Texte ändern. Neue Crates.

## Schritte

1. Leitfaden, `table.go`, `texts.go`, `table_test.go` und `TestTextsComplete` ganz lesen.
2. `texts.rs`, danach `table.rs`; nach jeder Datei `cargo build -p k3c-game`; `pending.rs` aufräumen und die Platzhalter der Lizenzen anlegen.
3. Tests portieren (`texts_tests.rs`, `table_tests.rs`), Namen in `snake_case`.
4. `cargo test -p k3c-game menu`, `task rust:lint`.
5. Ergebnis mit `plan_section` eintragen (nennt die vier `ignore`-Tests mit Sprint), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-03 (Teil table, texts): `texts_complete`, `keys_page`, `table_geometry` portiert und grün; `licenses_page`, `main_shows_version`, `main_first_draw_after_leave` portiert und mit `ignore` auf RU12 bzw. RU9.4 markiert (`cargo test -p k3c-game menu`).
- [ ] AC-01 (Teil Tabellen): B, Esc, Home und A schließen Tasten- und Lizenz-Seite (Test je Taste, Pad und Tastatur).
- [ ] AC-02 (Teil Tabelle): Zeichen-Reihenfolge und Zeilenmaße wie Go (`table_geometry`).
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

AC-01 geprüft: B (Back), Esc (Back + Menu zugleich), Home und A (Confirm) schließen die Tasten-Seite mit `StepKind::Pop` und Cue `Back`, je Taste an Pad und Tastatur; Hoch/Runter bleibt (`keys_page`). Die Lizenz-Seite teilt denselben Handler, ihr eigener Test (`licenses_page`) wartet auf RU12.
AC-02 geprüft: Zeilenmaße und Zeichen-Reihenfolge wie Go (`table_geometry`; Review: Reihenfolge backdrop, Titel, Fit-Cache, Papier, Zeilen, Hinweis gleich).
AC-03 geprüft: `texts_complete`, `keys_page`, `table_geometry` grün (`cargo test -p k3c-game menu`: 17 grün, 5 ignoriert); Fmt und Clippy `-D warnings` grün. Review k3c-reviewer: keine schweren Befunde; alle 131 Texte per Skript gegen `texts.go` verglichen (Wortlaut, Reihenfolge).
Ignorierte Tests: `licenses_page` (wartet auf RU12), `main_shows_version` (RU9.4), `main_first_draw_after_leave` (RU9.4), dazu aus RU9.2 `menu_rows_carry_art` und `run_remembers_operating_device` (RU9.4).
Dateien: `table.rs`, `texts.rs` (Enum `TextKey`, `tr`, `tr1`), `texts_data.rs` (Texte, aus `texts.go` per Skript erzeugt), `table_tests.rs`, `texts_tests.rs`; `pending.rs` ohne `TextKey`, `tr`, `new_keys`, `new_licenses`, dafür `License`, `licenses()`, `version_text` (RU12). `keys.rs` nutzt `TextKey::Pad`/`Keyboard`.
Abweichungen: Der Test `keys_page` öffnet die Seite direkt statt über Pause › Tasten (Pause-Menü kommt in RU12; der Weg wird dort geprüft). `TableScreen` hält den Text-Zwischenspeicher in einer `RefCell` (`Screen::draw` nimmt `&self`). Texte liegen aus Zeilenlängengründen in `texts_data.rs` statt in `texts_more.rs`.
`task check:diff`: Rust-Test-Stufe rot nur durch `tests/still.rs` (auch auf `main` rot, siehe RU9.1).

### Für den Hauptrechner
- `table::new_licenses` ist über `MenuSet::page("licenses")` erreichbar, `licenses()` aber noch Platzhalter (`unimplemented!`): Panic beim ersten `draw`, falls RU9.4 (Hauptmenü mit Karte Lizenzen) vor RU12.3 fertig wird. Reihenfolge beachten.
