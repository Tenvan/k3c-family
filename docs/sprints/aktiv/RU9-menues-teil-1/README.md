# RU9 · CLI · Menüs Teil 1: Hauptmenü, Lobby, Einstellungen

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** 7edb03edb
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`native/k3c-game/menu` hat 3429 Zeilen in 22 Dateien; das ist für einen Sprint zu viel.

## Ziel

Hauptmenü, Lobby mit zwei Geräten und Einstellungen laufen im Rust-Spiel. Am Ende sichtbar: Start bis Lobby mit Tastatur und Pad.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 9 von 26, 7 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/menu/` → `rust/crates/k3c-game/src/menu/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf `sprint/ru9`.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/menu/`: menu (102), layout (91), draw (255); Test-Hilfen `fake_test` (154), `paper_test` (Farben, Karten), `keys_test` (`TestGeomFitsWindow`); neu: `version.rs`, `pending.rs` | 448 |
| 2 | `native/k3c-game/menu/`: art (130, T), backdrop (132), hero (31), keys (183, T); `paper_test` (Hintergrund) | 476 |
| 3 | `native/k3c-game/menu/`: table (194, T), texts (298); `keys_test` (`TestTextsComplete`) | 492 |
| 4 | `native/k3c-game/menu/`: main_screen (200, T), lobby (270, T); `TestMainShowsOutcomeOfFrame` aus `pause_test` | 470 |
| 5 | `native/k3c-game/menu/`: options (229, T), controls (122, T) | 351 |
| 6 | Bildvergleich der Seiten main, lobby, options, keys, licenses gegen Go-Bilder (Aufnahme-Modus der Go-EXE) | – |
| 7 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2237 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle. Abweichung vom Schnitt: `main_screen` trägt ein `T` (`main_test.go` existiert), Session 6 ist neu (siehe Offene Fragen).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

Ausnahme für Session 6: Die Go-Bilder entstehen mit der vorhandenen Go-EXE im Aufnahme-Modus (kein Sim-Lauf, kein Balancing); `task gen:paper` nur dort und nur für die Illustrationen (B-742).

- **Bereich:** `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/src/lib.rs` (nur `pub mod menu;`), `rust/crates/k3c-game/tests/menu_bild.rs`, `testdata/rust/bild/menu/**`
- **Abhängig von:** RU7, RU39
- **Braucht 🧑:** – (Haltepunkt in RU9.6: fehlt der Rasterer `still/` aus RU13.6, hält die Session an und fragt 🧑)
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Navigation mit Tastatur und Pad; B und Esc gehen in jedem Menü zurück.
- **AC-02** Hauptmenü, Lobby mit zwei Geräten, Optionen und Steuerung sehen aus wie im Go-Spiel (Bildvergleich je Seite).
- **AC-03** Die Tests der portierten Dateien aus `native/k3c-game/menu` sind portiert und grün.

## Offene Fragen

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Session 6 (Bildvergleich) ist neu, weil AC-02 einen Bildvergleich je Seite verlangt und ihn sonst keine Session liefert; Quelle der Go-Bilder ist der Aufnahme-Modus der Go-EXE (`--menu <seite> --shots`), nicht neuer Go-Code.
- `Version` (Go: Variable in `licenses.go`, RU12) wird schon in Session 1 als `menu/version.rs` angelegt, weil das Hauptmenü sie liest; sie hat kein Go-Gegenstück als Datei.
- Gemeinsamer Menü-Zustand als `Rc<RefCell<Menus>>` (Screens zeichnen mit `&self`, die Schrift braucht `&mut`); der Leitfaden verbietet beides nur in den Sim-Crates.
- `sim.DefaultOptionsFor` und `sim.StandardWorldID` stehen in Rust erst mit dem Sim-Teil (RU14); die Lobby liest `k3c_data::difficulty` und führt die Welt-ID als Konstante.
- Die Lobby-Aufnahme mit zwei Geräten hängt davon ab, ob `--input` das Beitreten ausdrückt; sonst deckt ein Rust-Test das zweite Gerät.
- Sichtbar am Ende des Sprints (Ziel „Start bis Lobby“) heißt hier: Bilder der Seiten aus dem Bildvergleich; ein Fenster mit Schleife liefert erst RU26. Die Sichtprüfung mit Tastatur und Pad am PC steht im Abnahme-Ticket.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU9.1 | `RU9.1-menu-kern-layout-draw.md` | Umsetzung | autonom | fertig |
| RU9.2 | `RU9.2-menu-art-backdrop-hero-keys.md` | Umsetzung | autonom | fertig |
| RU9.3 | `RU9.3-menu-table-texts.md` | Umsetzung | autonom | fertig |
| RU9.4 | `RU9.4-menu-main-lobby.md` | Umsetzung | autonom | offen |
| RU9.5 | `RU9.5-menu-options-controls.md` | Umsetzung | autonom | offen |
| RU9.6 | `RU9.6-menu-bildvergleich.md` | Umsetzung | autonom | offen |
| RU9.7 | `RU9.7-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
