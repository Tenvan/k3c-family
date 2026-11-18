# RU30 · CLI · Spielschleife Teil 2: echtes Spiel, Aufnahme-Modus, Start

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach RU27 kann die Schleife die echte Sim rechnen; es fehlen Aufnahme-Modus (`--flow`, `--at`) und Startparameter.

## Ziel

Das Rust-Spiel ist von Start bis Niederlage spielbar. Am Ende sichtbar: `k3c-game.exe` aus Rust startet, zwei Spieler spielen eine Nacht.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 16 von 26, 7 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/game/` → `rust/crates/k3c-game/src/game/`, `native/k3c-game/main.go` → `rust/crates/k3c-game/src/main.rs` und `game/run.rs`; Dateinamen nach Leitfaden § 2. Die Tabelle weicht vom Entwurf ab (Gründe unter Offene Fragen): `play.go` ist in RU26 gelandet; `measure_frames.go` und `measure_load.go` fehlten; der Entwurf nannte `cmd/k3c-game/`, die Quelle ist `native/k3c-game/main.go`.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/game/`: levels (94, T), loading (147, T), loadscreen (95) | 336 |
| 2 | `native/k3c-game/game/`: assets (44), voyage (154, T), script (143, T), shots (181, T) | 522 |
| 3 | `native/k3c-game/game/`: devshot (42), devkeys (124, T), devvalues (60, T), fault (130, T) | 356 |
| 4 | `native/k3c-game/game/`: measure (259), measure_frames (170, T), measure_load (122, T) | 551 |
| 5 | `native/k3c-game/game/`: stress (225, T), stressreport (170) | 395 |
| 6 | `native/k3c-game/`: main (168), `winres/` (Icon, Manifest) | 168 |
| 7 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2328 Zeilen Go (Entwurf: 2003). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- **Vorhandenes in Rust nicht doppelt bauen:** Die Plattform-Schleife mit Fenster und headless (`k3c-platform`: `run_window`, `run_headless`, `Runner`, `Clock`) liefert RU4; RU30 baut keine eigene. Die Wiedergabe (`replay`, RU38) hat `replay/blend.rs` und `replay/report.rs`; RU30.4 benutzt den Bericht, RU26.2 den `blend`.
- **Canvas nur im Zeichen-Takt:** Texturen lassen sich in Rust nur über die Canvas hochladen; das Laden der Level (RU30.1) läuft deshalb am Anfang von `draw`, die Entscheidung im Takt.
- **Aufnahme ohne Pixel-Auslesen:** `k3c-platform` kann kein Bild auslesen. Aufnahme-Modus (`--shots`) und F12 zeichnen gegen den Software-Rasterer aus RU13.6 (`still/`); Abweichungen davon zum Fensterbild sind erwartet und werden im Abnahme-Ticket gesichtet.
- **Messbericht:** Format und Feldnamen wie `Report` in `measure.go`, damit RU39 und spätere Messungen Go und Rust vergleichen; Felder ohne Rust-Gegenstück (`go_heap_mib`, `gc_*`) bleiben 0.
- **Ruckeln:** Zwischenbilder und Takt hängen an `Clock` und `alpha` der Plattform (RU4); die Messgrundlage ist `docs/messung/rust-client-1.md` (RU2.4) und der Vergleich in RU0.2. Messläufe des echten Spiels gehören in das Abnahme-Ticket und nach RU39, nicht in die Port-Sessions.
- **Zwei Spieler:** Jede Session mit Eingabe, Kamera oder Zeichnen testet beide Spieler; B ist in Menüs zurück, im Spiel unbelegt.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Seit 2026-11-18 liefert RU48 (Smoke-Client) den Start: `game/launch.rs`, den Verteiler in `main.rs` und `task rust:run`; RU30.6 ergänzt darauf nur Startparameter, Fenster-Fehlerbild und Icon, RU30.1 ersetzt das einmalige Laden der Grafik durch das Laden je Stufe.

- **Bereich:** `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-game/src/main.rs` (nur Verteiler und Spielstart), `rust/crates/k3c-game/tests/run_*.rs`; nur mit Freigabe von 🧑 (RU30.6): `rust/crates/k3c-game/build.rs`, `rust/crates/k3c-game/assets/**`, `rust/crates/k3c-game/Cargo.toml` (Ressourcen-Crate für das Icon)
- **Abhängig von:** RU26, RU27, RU48, RU50
- **Braucht 🧑:** Freigabe eines Ressourcen-Crates für das EXE-Icon (RU30.6, Haltepunkt)
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Ein Spiel mit zwei Spielern läuft vom Hauptmenü bis zur Niederlage.
- **AC-02** Der Aufnahme-Modus nimmt dieselben Stationen auf wie das Go-Spiel.
- **AC-03** Die EXE trägt das Spiel-Icon und kennt die Startparameter des Go-Spiels.
- **AC-04** Der Messmodus und der Stresstest schreiben Berichte mit denselben Feldern wie das Go-Spiel (`measure.go` › `Report`, `stressreport.go` › `StressReport`).

## Offene Fragen

Eine Entscheidung bleibt bei 🧑: Darf `k3c-game` ein Ressourcen-Crate (Vorschlag `winresource`, nur Windows, nur `build-dependency`, Version fest) für das EXE-Icon und das Manifest einbinden? RU30.6 baut alles andere und hält dann an.

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Schnitt geändert: `play.go` (147, T) steht in RU26.5 (Ziel von RU26 verlangt den Weg Hauptmenü, Spiel, zurück); `measure_frames.go` (170, T) und `measure_load.go` (122, T) sind ergänzt (`measure.go` hat 259 statt 247 Zeilen); `main.go` (168) bekommt eine eigene Session; das Entwurfs-Ziel „`cmd/k3c-game/`“ war falsch, die Quelle ist `native/k3c-game/main.go`.
- Der Aufnahme-Modus und F12 nutzen den Software-Rasterer aus RU13.6 (`still/`), weil `k3c-platform` keine Pixel ausliest; Bild kann vom Fensterbild abweichen.
- `--pacing` und `--novsync` werden angenommen und wirken in Rust nicht (die Zeitsammlung ist `Clock`, VSync ist Fifo); im Bericht bleibt `vsync` wahr. `--size` bestimmt nur die Aufnahme-Größe; Fenstergröße und -Icon der Titelleiste bleiben bei der Plattform (Ticket).
- Das Laden der Grafik der Level läuft am Anfang von `draw` (nur dort liegt eine Canvas), die Entscheidung im Takt wie in Go.
- Der Messbericht benutzt `replay/report.rs` (RU38.3), wenn er alle Felder trägt; sonst entsteht er in `game/measure.rs` und ein Ticket stellt `replay` um.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU30.1 | `RU30.1-levels-loading-loadscreen.md` | Umsetzung | autonom | fertig |
| RU30.2 | `RU30.2-assets-voyage-script-shots.md` | Umsetzung | autonom | fertig |
| RU30.3 | `RU30.3-dev-tasten-fehlerfenster.md` | Umsetzung | autonom | fertig |
| RU30.4 | `RU30.4-measure-bericht.md` | Umsetzung | autonom | offen |
| RU30.5 | `RU30.5-stresstest.md` | Umsetzung | autonom | offen |
| RU30.6 | `RU30.6-start-main-icon.md` | Umsetzung | autonom | offen |
| RU30.7 | `RU30.7-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
