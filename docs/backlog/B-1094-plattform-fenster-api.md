# B-1094 · Die Plattform bekommt eine Fenster-API für Größe, Lage und Rahmen

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU26
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c_platform::Config` kennt nur `fullscreen`; `App::fullscreen()` schaltet randloses Vollbild (`rust/crates/k3c-platform/src/backend/mod.rs`, `src/lib.rs`). Go setzte Größe, Lage und Rahmen des Fensters (`native/k3c-game/game/window.go`: `applyWindow`, `windowRect`). Rust (`rust/crates/k3c-game/src/game/window.rs`, RU26.5) merkt den Modus nur und liefert den Vollbild-Wunsch; `window_rect` ist portiert, wird aber nicht angewendet. Auch `--size` (Fenstergröße beim Start) hat keine Entsprechung (siehe RU30.6).

## Ziel

Spieler und Entwickler bekommen wie in Go den Modus Fenster mit Rahmen (Startgröße, höchstens 90 % des Monitors, mittig), rahmenloses Fenster in Monitorgröße und Vollbild; die Startgröße lässt sich über `--size` setzen.

## Beteiligte und Zielgruppen

Spieler am PC (Optionen › Anzeige, Taste F bzw. rechter Stick); Entwickler (`--size`). Entscheidet: Ralf.

## Anforderungen

- `k3c_platform` bietet eine Fenster-API: Startgröße in `Config`, zur Laufzeit Größe, Lage und Rahmen setzen (zum Beispiel über `App`: gewünschter Modus als Wert statt nur `fullscreen() -> Option<bool>`).
- Die Monitorgröße steht der App zur Verfügung (für `window_rect`).
- `game::Game::apply_window` ruft die API wie Go `applyWindow`; `window_rect` bleibt die reine Funktion dafür.
- Headless und Aufnahme-Modus bleiben ohne Fenster; die Schleife (30 Hz, `alpha`) bleibt unverändert.

## Nicht-Ziele

Mehrere Fenster, Mehr-Monitor-Wahl, Änderungen an Menü oder HUD.

## Regeln und Einschränkungen

Leitfaden `docs/standards/rust-port.md` § 8 (Zeichnen nur über `platform::Canvas`); Verhalten wie Go (`window.go`) ist das Orakel; keine neuen Abhängigkeiten ohne Entscheidung.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-platform/**`, `rust/crates/k3c-game/src/game/window.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme (Fensterverhalten am PC)
- **Größe:** S
- **Sperre:** –

## Beispiele

Optionen › Anzeige › Fenster: 1280 × 720 auf einem 1920 × 1080-Monitor erscheint mittig mit Rahmen; rahmenlos füllt den Monitor; F wechselt zwischen Vollbild und Fenster.

## Ausnahme- und Fehlerfälle

Startgröße größer als 90 % des Monitors: auf 90 % begrenzt (wie Go `windowRect`). Aufnahme-Modus (`--shots`): kein Fensterwechsel.

## Akzeptanzkriterien

- **AC-01** Ein Test der Plattform zeigt, dass Größe, Lage und Rahmen gesetzt werden (Fenster-Attrappe oder Backend-Test).
- **AC-02** `Game::apply_window` wendet `window_rect` an; die Tests `window_tests` bleiben grün.
- **AC-03** Abnahme durch Ralf am PC: Fenster, rahmenlos und Vollbild wechseln wie im Go-Spiel.

## Offene Fragen

keine

## Notizen

Entstanden in RU26.5 (Abweichung: Rust setzt nur Vollbild über `App::fullscreen`).
