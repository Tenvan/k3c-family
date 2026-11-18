# RU26 · CLI · Spielschleife Teil 1: Zustände, Split-Screen, Zeichnen

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`game` (3891 Zeilen) verbindet Menü, Spiel, HUD und Zeichnen und kennt alle Fachpakete.

## Ziel

Das Rust-Spiel hat seine Hauptschleife mit allen Zuständen und zwei Bildhälften. Am Ende sichtbar: vom Hauptmenü ins Spiel und zurück, noch mit eingebauter Test-Welt.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 11 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/game/` → `rust/crates/k3c-game/src/game/`; Dateinamen nach Leitfaden § 2. Die Tabelle weicht von der Entwurfs-Tabelle ab (Gründe unter Offene Fragen): `game.go` ist die Nabe und kommt nach ihren Voraussetzungen; `pacing.go` und `play.go` sind ergänzt; `deps.go` entfällt.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/game/`: modes (77, T), stack (62, T), scene (226, T), pacing (51, T; ersetzt durch `k3c_platform::Clock`) | 416 |
| 2 | `native/k3c-game/game/`: layout (99, T), parts (141, T), blend (188, T), Test `imports_test.go` | 428 |
| 3 | `native/k3c-game/game/`: devices (118, T), keytable (97, T), overlay (197, T), notice (136, T), clock_other (10), clock_windows (29) | 587 |
| 4 | `native/k3c-game/game/`: game (384), `deps.go` (17) entfällt | 384 |
| 5 | `native/k3c-game/game/`: window (76), menu (61), play (147, T) | 284 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2099 Zeilen Go (Entwurf: 1888; `game.go` hat 384 statt 354 Zeilen, dazu `pacing.go` 51 und `play.go` 147, ohne `deps.go`). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle. Tests anderer Sprints in `native/k3c-game/game/` (`levels`, `loading`, `voyage`, `script`, `shots`, `devkeys`, `devtable`, `devvalues`, `fault`, `measure*`, `stress`, `story`) bleiben bei ihren Dateien in RU30.

### Vorhandenes in Rust (Stand RU4, RU38)

- Die Plattform-Schleife mit und ohne Fenster steht (`k3c-platform`: `trait App { step, draw(alpha), done, fullscreen }`, `Runner`, `Clock`, `run_window`, `run_headless`, `NullCanvas`, `InputState`, `InputScript`). RU26 baut **keine** eigene Schleife, `Game` ist eine `App`; die Zeitsammlung (30 Hz, höchstens 4 Takte je Bild, alpha) ist `Clock` und ersetzt `pacing.go`.
- RU38.2 baut in `rust/crates/k3c-game/src/replay/` die Wiedergabe als `App` samt `replay/blend.rs` (Port von `blend.go`). RU26.2 nutzt diese Datei weiter (`pub use`), statt sie ein zweites Mal zu schreiben; weicht sie von `blend.go` ab, steht ein Ticket im Ergebnis.
- Der Messbericht der Zwischenbilder und des Takts (`docs/messung/rust-client-1.md`, RU2.4; Vergleich mit Go in RU0.2) ist die Grundlage für den ruckelfreien Weg: Takt 30 Hz mit alpha, VSync, `alpha` nie aus der Wanduhr.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-game/src/lib.rs` (nur `pub mod game;`)
- **Abhängig von:** RU12, RU17, RU23
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Alle Zustände der Go-Schleife gibt es mit denselben Übergängen.
- **AC-02** Beide Hälften zeichnen in derselben Reihenfolge wie im Go-Spiel; jeder Spieler hat eigene Eingabe.
- **AC-03** Die Schleife läuft auch headless (RU4/AC-02).

## Offene Fragen

Keine offenen. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Reihenfolge der Sessions: `game.go` (Nabe, kennt alle Teile) kommt als Session 4 nach ihren Voraussetzungen; Schnitt und Grund oben (Anforderungen).
- `play.go` (147, T) kommt aus RU30.1 nach RU26.5, weil das Ziel „vom Hauptmenü ins Spiel und zurück“ es braucht; RU30.1 schrumpft entsprechend.
- `pacing.go` wird nicht portiert: `k3c_platform::Clock` ist dieselbe Zeitsammlung, `TestPacer` läuft als `pacing_tests.rs` gegen `Clock`; `--pacing` wird akzeptiert und ignoriert. `alpha` der Zwischenbilder kommt immer aus `App::draw`, nie aus der Wanduhr (Gos Standard-Lauf nutzte die Wanduhr; Rust folgt dem in RU0.2 ruckelfreien `sync`-Weg).
- `Layout(ow, oh)` entfällt: Die Zeichenfläche ist immer 1920 × 1080, das Fenster skaliert.
- Fenster-Größe und -Lage (`window_rect`) setzt Rust nicht (nur Vollbild über `App::fullscreen`); Ticket für eine Fenster-API der Plattform in RU26.5.
- `deps.go` und `clock_windows.go` entfallen ohne Ersatz (Go-spezifisch).
- Kein `clap`: Die Schalter parst `game/flags.rs` ohne neue Abhängigkeit.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU26.1 | `RU26.1-args-modi-stapel-pacing.md` | Umsetzung | autonom | fertig |
| RU26.2 | `RU26.2-layout-parts-blend.md` | Umsetzung | autonom | fertig |
| RU26.3 | `RU26.3-devices-keytable-overlay-notice.md` | Umsetzung | autonom | fertig |
| RU26.4 | `RU26.4-game-schleife.md` | Umsetzung | autonom | fertig |
| RU26.5 | `RU26.5-window-menu-play.md` | Umsetzung | autonom | fertig |
| RU26.6 | `RU26.6-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18, Review RU26.6: AC-01 bis AC-03 geprüft (Ergebnis RU26.6: `Stack::apply` und `draw.rs` gegen `game.go`, `loop_tests` headless), `task check` und `task check:go` grün.
Keine schweren Befunde, keine Fixes nötig; Leitplanken (Dateigröße, Funktionslänge, `unsafe`, Platzhalter RU29/RU30, Bereichsgrenzen) eingehalten.
Neue Tickets aus den Sessions: B-1090 bis B-1094 (Blend, Importregel, Fenster-API u. a.).
Abnahme-Ticket für Sim, Balance, Gen und Race: B-1096 (Lane zweit, mit Prüfliste für Ralf zu Takt, Eingabe, Kamera).
