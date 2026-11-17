# RU4.9 · Log-Regel und Probe-Programm für beide Modi

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.3, RU4.7, RU4.8
- **Tickets:** B-1014
- **Kriterien:** AC-02, AC-05

## Ziel

Log-Meldungen der Rust-Crates folgen der Emoji-Regel, und das Probe-Programm zeigt alles aus RU4 in beiden Modi.

## Kontext

Emoji-Liste: `CLAUDE.md` › Regeln › Logging mit Emojis (nur ein Codepunkt, kein U+FE0F). Vorgaben: RU4-README › Vorentscheidungen › Log. Probe heute: `rust/crates/k3c-platform/examples/probe.rs` (Sprite-Last, Messung). Plattform-Teile aus RU4.1–RU4.8: `InputState`, `clip`, Formen, `text_font`, `AudioOut`. Die Auswertung zu Spielern (`input::Source`) liegt in `k3c-game` und gehört nicht ins Probe-Programm; die Probe bewegt je Gerät (Tastatur, Pad) ein Kästchen in einer eigenen Bildhälfte.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/crates/k3c-game/src/demo/**`

## Nicht-Ziele

Log-Themen je Namensraum (`engine/conlog` › Topics). Logs anderer Crates umschreiben, die es noch nicht gibt.

## Schritte

1. `src/log.rs`: `EMOJIS: &[char]`, `debug`, `info`, `warn`, `error` mit Schlüssel-Wert-Paaren nach stderr; `debug_assert!` auf das erste Zeichen.
2. `tests/log_emoji.rs`: liest alle `rust/crates/*/src/**/*.rs` und `examples/*.rs`, sucht Aufrufe `log::(debug|info|warn|error)("` und prüft das erste Zeichen gegen `EMOJIS`.
3. Bestehende Meldungen in `k3c-platform` und `k3c-game/src/demo` (`eprintln!` mit Lage-Meldungen) auf `log::*` umstellen; reine Hilfe-Texte der Befehlszeile bleiben `eprintln!`.
4. `examples/probe.rs`: Modus `--two` mit zwei Bildhälften (`clip`), je Gerät ein Kästchen, Text „Grüße, ÄÖÜ“ mit `k3c.ttf`, Taste T spielt einen Klang; `--headless --steps N --script <datei>` ohne Fenster und Ton, Code 0. Beispielskript `examples/probe-script.json`.

## Fertig, wenn

- [ ] AC-05: `cargo test -p k3c-platform --test log_emoji` grün; eine Meldung ohne Emoji lässt ihn fehlschlagen (kurz probiert, nicht eingecheckt).
- [ ] AC-02: `cargo run -p k3c-platform --example probe -- --two --headless --steps 1000 --script crates/k3c-platform/examples/probe-script.json` endet mit Code 0.
- [ ] AC-05: Fensterlauf `--two` zeigt zwei Hälften und Umlaute (Screenshot durch Claude); Ton und Pad im Ergebnis als beobachtet oder ungeprüft.
- [ ] `task rust:lint` grün.

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

- AC-05 geprüft: `cargo test -p k3c-platform --test log_emoji` grün; eine eingefügte Meldung ohne Emoji ließ ihn fehlschlagen (probiert, zurückgenommen). Lage-Meldungen in Plattform, Probe und `k3c-game/src/input` laufen über `log::*`.
- AC-02 geprüft: `cargo run -p k3c-platform --example probe -- --two --headless --steps 1000 --script crates/k3c-platform/examples/probe-script.json` → Code 0 (1000 Takte, 2 Klänge über NullAudio, beide Kästchen bewegt).
- AC-05 geprüft (Screenshot durch Claude): Fenster `--two` auf Vulkan, 59 Bilder/s, zwei Hälften mit Ausschnitt, Text „Grüße, ÄÖÜ äöü ß“ aus `k3c.ttf`. Ton und Pad: ungeprüft (kein Pad am Rechner, Ton nicht hörbar prüfbar).
- Clippy (`-D warnings`) grün.
- Probe liegt jetzt unter `examples/probe/` (`main.rs`, `two.rs`).
- **Ralf prüft:** `--two` mit Pad: rechtes Kästchen folgt dem Stick, A und T sind hörbar. Die Hintergründe wirken heller als die Zahlen; vermutet: sRGB-Fläche, wie bei den anderen Farben.
