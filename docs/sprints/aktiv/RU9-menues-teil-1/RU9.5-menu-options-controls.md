# RU9.5 · menu Teil 5: Optionen und Steuerung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU9.4
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Die Seiten Optionen und Steuerung laufen in Rust wie in Go und schreiben über die Session; B und Esc gehen zurück.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst lesen, dann die Vorentscheidungen in `RU9.1` bis `RU9.4`. Quellen in `native/k3c-game/menu/`: `options.go` (229, Test `options_test.go`) und `controls.go` (122, Test `controls_test.go`). Ziel in `rust/crates/k3c-game/src/menu/`: `options.rs`, `options_tests.rs`, `controls.rs`, `controls_tests.rs`. Rust-Verträge: `view::settings` (`Settings`, `default_settings()`, `Settings::normalized()`, `WINDOW_MODES`, `LANGUAGE_ENGLISH`, `Controls`), `view::session::Session::{settings, set_settings}`, `view::profile` (Steuerprofile; `controls.go` nennt sie in `profile_text`).

**Vorentscheidungen:**
- `apply_option(s: &Settings, opt: usize, k: Key) -> Settings` ist rein (Tabellen-Test `apply_option`); die Lautstärke steppt wie Go (`volume_step = 0.1`, `step_volume` rundet wie `math.Round(v*10)/10`; `round` ist in Rust und Go gleich, dafür den Test mit den Randwerten 0 und 1 behalten).
- Eine Einstellung wird nur geschrieben, wenn sie sich ändert (`options_unchanged_value_is_not_written`); ein Fehler von `set_settings` behält die alten Werte und zeigt die Meldung (`options_save_error_keeps_values`, `Fehler` aus `view::session`).
- `Optionen` aus der Pause (`dim = true`) öffnet `new_controls(m, dim)`; Zeile Steuerung: `A` öffnet, `Links`/`Rechts` tun nichts (Go `sc.sel == optControls`).
- `new_options` und `new_controls` sind die letzten Screens dieser Session; die Platzhalter aus `pending.rs` werden gelöscht, ebenso die `ignore`-Marken in `main_screen_tests.rs`, die nur auf `new_options` warteten (`main_confirm_transitions` bleibt ignoriert, bis RU12.1 auch `new_world_pick`, `new_saves` und `new_testing` liefert).
- Zeichnen: `fill(lang)` baut die Seite nur bei neuer Version oder Sprache (`Page::stale`), wie in RU9.1; `RefCell<Page>` im Screen, `draw(&self)`.

**B und Esc:** Beide Seiten schließen mit `Back` und `Home` (`Pop`, Cue `Back`); Optionen auch über die Zeile Zurück mit `A`. Tests: `options_navigation`, `controls_back_and_texts`; ergänzend je Seite ein Test, dass `Action::Back` (B am Pad) und `Action::Menu` (Esc) `Pop` liefern, auch aus dem Pause-Menü (`dim = true`).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`

## Nicht-Ziele

Änderungen an Go-Code. Das Tastenbelegungs-Profil der Eingabe ändern (`input/`, RU7). Neue Optionen oder Texte. Neue Crates.

## Schritte

1. Leitfaden, `options.go`, `controls.go`, `options_test.go` und `controls_test.go` ganz lesen; `view/settings.rs` und `view/profile.rs` ansehen.
2. `options.rs`, dann `controls.rs`; Platzhalter `new_options` entfernen, `ignore`-Marken aufräumen; nach jeder Datei `cargo build -p k3c-game`.
3. Tests portieren (zehn und drei Tests) und die B/Esc-Tests ergänzen.
4. `cargo test -p k3c-game menu`, `task rust:lint`.
5. Ergebnis mit `plan_section` eintragen, Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-03 (Teil options, controls): alle zehn Tests aus `options_test.go` und alle drei aus `controls_test.go` portiert und grün (`cargo test -p k3c-game menu`).
- [ ] AC-01 (Teil Optionen, Steuerung): Navigation mit Hoch/Runter/Links/Rechts/A, B, Esc und Home gehen zurück, auch aus dem Pause-Menü; die Zeile Steuerung öffnet die Tastenbelegung.
- [ ] AC-02 (Teil Optionen, Steuerung): Wertetexte (Lautstärke, Fenster, Sprache, Profile) wie Go (`option_value_texts`, `option_window_modes`); Bildvergleich folgt in RU9.6.
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
