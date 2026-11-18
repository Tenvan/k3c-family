# RU9.4 · menu Teil 4: Hauptmenü und Lobby

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru9
- **Abhängig von:** RU9.3
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Hauptmenü und Lobby mit zwei Geräten (Tastatur, Pads) laufen in Rust wie in Go: Auswahl, Beitritt, Austritt, Start; B und Esc gehen zurück.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst lesen, dann die Vorentscheidungen in `RU9.1` bis `RU9.3`. Quellen in `native/k3c-game/menu/`: `main_screen.go` (200, Test `main_test.go`) und `lobby.go` (270, Test `lobby_test.go`). Ziel in `rust/crates/k3c-game/src/menu/`: `main_screen.rs`, `main_screen_tests.rs`, `lobby.rs`, `lobby_tests.rs`. Nach diesen Tests gehört `TestMainShowsOutcomeOfFrame` (steht in `pause_test.go`) in `main_screen_tests.rs`; RU12.1 portiert ihn dann nicht noch einmal. Die zwei Hauptmenü-Tests aus `table_test.go` (`main_shows_version`, `main_first_draw_after_leave`, in RU9.3 mit `ignore` angelegt) werden hier aktiviert.

**Vorentscheidungen:**
- Das Hauptmenü öffnet Screens, die erst RU9.5 und RU12 liefern (`new_world_pick`, `new_saves`, `new_options`, `new_testing`): Platzhalter in `pending.rs` (RU9.1); Tests, die einen davon durchlaufen, stehen mit `#[ignore = "wartet auf RU9.5"]` bzw. `"wartet auf RU12"`: `main_confirm_transitions`, `main_rereads_list_after_load_menu`, `main_continue_*` laufen ohne (Fortsetzen gibt `StepKind::Play`). RU9.5 und RU12.1 aktivieren die Tests, indem sie den Platzhalter löschen und das `ignore` entfernen.
- `dev_build()` ist `version::version().is_empty()` (RU9.1); `main_shows_version` setzt `set_version` und stellt den alten Wert am Ende zurück.
- Lobby: `sim.StandardWorldID` und `sim.DefaultOptionsFor(grade).Defeat` gibt es in Rust erst mit dem Sim-Teil (RU14). `default_defeat(grade)` liest `k3c_data::difficulty` (`grades[grade].default_defeat`, unbekannter Grad wie in Go `normal`) und sucht den Index in `DEFEATS = ["resources", "stage"]` (`max(0, …)`); die Standard-Welt-ID wird als `const STANDARD_WORLD_ID` mit dem Wert aus `engine/sim` (`grep -rn StandardWorldID engine/sim/*.go`) und Kommentar „RU14 ersetzt durch `k3c_sim`“ in `lobby.rs` geführt (kein `pending.rs`, weil es ein Wert ist).
- Zwei Geräte: `devs: [DeviceID; MAX_PLAYERS]`, `slot(d)` per `position`; nicht beigetretene Geräte bedienen nichts (nur `OK` tritt bei); `Back` eines Spielers mit Platz > 0 tritt aus, Spieler 0 und `Home` gehen zurück (`Pop`); Beitritt in Druck-Reihenfolge; `new_game()` schreibt `view::session::NewGame` wie Go. Die Reihenfolge der Geräte im Start-Aufruf ist Beitritts-Reihenfolge (Test `lobby_starts_with_two_players_in_join_order`).
- Ganzzahl-Typen: Zeilen und Indizes `usize`, Seed `i64` (0 = Name des Spielstands, höchstens 999, `lobby_seed_wraps_with_confirm`).

**B und Esc:** Das Hauptmenü ist die Wurzel, dort ist `Back` ohne Wirkung (Go lässt `keyBack` aus); ein Test `main_ignores_back` hält das fest. In der Lobby deckt `lobby_rows_and_cancel` das Zurück ab; zusätzlich ein Test, dass B am Pad und Esc an der Tastatur denselben Schritt liefern (Spieler 0 → `Pop`, Spieler 1 → tritt aus, bleibt in der Lobby).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`

## Nicht-Ziele

Änderungen an Go-Code. Spielstand-Liste, Weltwahl, Pause (RU12). Optionen und Steuerung (RU9.5). Neue Crates. Der Sim-Teil (`sim.DefaultOptionsFor`, RU14).

## Schritte

1. Leitfaden, `main_screen.go`, `lobby.go`, `main_test.go`, `lobby_test.go` und `TestMainShowsOutcomeOfFrame` ganz lesen.
2. `main_screen.rs`, dann `lobby.rs`; `mod`-Zeilen ergänzen, Platzhalter `new_main` und `new_lobby` aus `pending.rs` entfernen; nach jeder Datei `cargo build -p k3c-game`.
3. Tests portieren, die zwei `ignore`-Tests aus RU9.3 aktivieren, neue Tests für B/Esc ergänzen.
4. `cargo test -p k3c-game menu`, `task rust:lint`.
5. Ergebnis mit `plan_section` eintragen (die `ignore`-Tests mit Sprint nennen), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-03 (Teil main_screen, lobby): alle Tests aus `main_test.go` und `lobby_test.go` sowie `TestMainShowsOutcomeOfFrame` portiert; grün sind alle, die keinen Screen aus RU9.5 oder RU12 durchlaufen; die übrigen tragen `ignore` mit Sprint (`cargo test -p k3c-game menu`).
- [ ] AC-01 (Teil Hauptmenü, Lobby): Auswahl mit Hoch/Runter, Beitritt eines zweiten Geräts, Austritt mit B, Start mit zwei Spielern in Beitritts-Reihenfolge; B und Esc in der Lobby gehen zurück, im Hauptmenü ohne Wirkung.
- [ ] AC-02 (Teil Hauptmenü, Lobby): Zeilen, Farben (`MAIN_MATS`) und Texte wie Go (Bildvergleich folgt in RU9.6).
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

AC-01 geprüft: Auswahl mit Hoch/Runter (überspringt ausgegrautes Weiter), Home, Weiter spielt mit dem bestätigenden Gerät; Lobby: Beitritt eines zweiten Geräts mit A, Austritt mit B, Start mit zwei Spielern in Beitritts-Reihenfolge (`lobby_starts_with_two_players_in_join_order`). B und Esc: im Hauptmenü ohne Wirkung (`main_ignores_back`), in der Lobby liefern B am Pad und Esc an der Tastatur denselben Schritt (`lobby_back_and_esc_do_the_same`: Spieler 1 → `Pop`, Spieler 2 → tritt aus, bleibt).
AC-02 geprüft: Zeilen, Farben (`MAIN_MATS`) und Texte wie Go (Review k3c-reviewer: Handler-Ablauf, Standard-Niederlage, `fill`-Texte gleich); Bildvergleich folgt in RU9.6.
AC-03 geprüft: alle Tests aus `main_test.go`, `lobby_test.go` und `TestMainShowsOutcomeOfFrame` portiert; `cargo test -p k3c-game menu`: 39 grün, 4 ignoriert. Aktiviert: `main_shows_version`, `run_remembers_operating_device` (RU9.2), `menu_rows_carry_art` (RU9.2). Fmt und Clippy `-D warnings` grün.
Ignorierte Tests (alle auf RU12, weil sie Spielstand-Liste, Weltauswahl, Pause oder Testing öffnen): `main_confirm_transitions`, `main_rereads_list_after_load_menu`, `main_first_draw_after_leave`; dazu aus RU9.3 `licenses_page` (RU12). Ersatz ohne diese Screens: `main_confirm_play_quit_licenses`, `main_first_draw_shows_loading`.
Dateien: `main_screen.rs`, `lobby.rs`, `main_screen_tests.rs`, `lobby_tests.rs`; `version.rs` mit `version_text` (Go: `licenses.go`, hier schon gebraucht; RU12.3 baut darauf auf); `pending.rs` ohne `new_main`, `new_lobby`, `MAIN_COUNT`, `version_text`, dafür `new_world_pick`, `new_testing` (RU12; `new_saves` ebenfalls RU12, nicht RU9.4). Abweichungen: `default_defeat` liest `data/difficulty.json` (loggt bei Lesefehler), Standard-Welt als Wert `STANDARD_WORLD_ID` (RU14 ersetzt); `MenuSet::main_screen()` liefert den konkreten Typ (für Tests und die Meldung des nächsten Hauptmenüs); Test-Sperre `version_guard` für Tests, die die globale Version lesen oder ändern.
`task check:diff`: Rust-Test-Stufe rot nur durch `tests/still.rs` (auch auf `main` rot, siehe RU9.1).

### Für den Hauptrechner
- Im Rust-Spiel führen „Neues Spiel“, „Laden“, „Optionen“, „Testing“ bis RU9.5/RU12 in `unimplemented!`; der Rust-Client darf das Menü erst einbinden (RU26), wenn die Platzhalter weg sind.
- `version_guard` schützen nur zwei Tests; ein künftiger Test mit Index auf Zeilen ab Testing sollte ihn ebenfalls nehmen.
