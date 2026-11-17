# RU7 · CLI · Grafik laden und Ansicht: Papierschnitt-Index, Sheets, Kamera

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** Ralf, 2026-11-17

## Ausgangslage

`native/k3c-game/art` (1174 Zeilen ohne Tests) lädt Gruppen-Indizes, Manifeste und Sheets je Level mit Texturbudget, hält Schrift und Rig-Bibliothek; `view` (1293 Zeilen) ist der Vertrag zwischen Spielzustand und Darstellung (Kamera, Zellen, Ebenen, Frame, Session).

RU2 hat in `rust/crates/k3c-game/src/art/` einen vereinfachten Lader gebaut (`mod.rs` mit `Library::open/assets/load`, `manifest.rs`, `paper.rs` sammelt alle Assets aus `data/paper.json`, Tests gegen `tests/fixtures/art/`); die Demo (`src/demo/`) nutzt ihn. Er ist kein Port der Go-API (keine Level, kein Budget, kein Freigeben, keine Schrift, keine Rigs). RU4 liefert die Plattform: `Canvas::sprite_tf`, `free_texture`, `clip`, `text_font`, `InputState`, sowie `view/{mod,input,profile,settings,devkeys}.rs`.

Sim-Typen: `view/frame.go`, `layer.go` (Drawer bekommt den Frame), `roles.go`, `screen.go`, `session.go`, `story.go`, `voyage.go` und `Command.Sim()` brauchen `sim.World`, `sim.Player`, `sim.Event`, `sim.PlayerCommand`, `sim.WorldDef`, `sim.Island` (RU8). `art/rigs.go` braucht `k3c-rig` (RU5). Alles andere in `art` und `view` ist ohne Sim und Rig portierbar.

## Ziel

Das Rust-Spiel lädt die Papierschnitt-Grafik über die portierte Go-API von `art` und hat den gemeinsamen Vertrag `view`. Am Ende sichtbar: die Demo aus RU2 (`k3c-game demo --driver rust/demo/reiten.json`) läuft über `art::Library` mit Leveln und Budget-Log; nach RU5 und RU8 sind Rigs und Frame-Vertrag ergänzt.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 4 von 26 (B-1015), überarbeitet nach RU2 (Revision 2). Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell: `sonnet`.

### Sessions

Ziel: `native/k3c-game/art/` → `rust/crates/k3c-game/src/art/`, `native/k3c-game/view/` → `rust/crates/k3c-game/src/view/`; Dateinamen nach Leitfaden § 2. Zeilen ohne Tests, „T“ = Test wird mitportiert.

| Session | Go-Dateien | Zeilen | Abhängig von |
|---|---|---:|---|

RU7.1–RU7.4 laufen ohne RU5 und RU8. RU7.5 und RU7.6 warten nur auf ihre Sprint-Abhängigkeit; bis dahin bleibt der Sprint aktiv (Regel „Sprint erst erledigt, wenn alles fertig“), RU9 und RU13 brauchen ohnehin RU8.

### Vorentscheidungen

- Die Go-API ist das Ziel; der RU2-Lader wird ersetzt, nicht erweitert. `art/paper.rs` (alle Assets aus `data/paper.json`) hat kein Go-Vorbild und wandert als `demo/assets.rs` in die Demo.
- Hochladen braucht die Zeichenfläche: `Library::set_levels(want, canvas)` und `LevelLoad::step(n, canvas)` bekommen `&mut dyn Canvas`; Pläne und PNG-Dekodieren laufen ohne Canvas wie `planLevels`. Freigeben über `Canvas::free_texture`.
- `ebiten.Image` eines Frames → `SheetFrame { tex: TexId, src: Rect }`; `Sheet::draw(canvas, i, Affine)` → `canvas.sprite_tf`.
- `art/outline.go` rendert Text mit Kontur in Offscreen-Bilder; Rust hat kein Offscreen-Ziel (RU4): Kontur wird direkt mit `text_font` an den Versätzen `Outline4`/`Outline8` gezeichnet, die Cache-Grenzen entfallen; die betroffenen Tests im Ergebnis nennen.
- Daten-Dateien (`data/paper.json`, `data/native.json`) liest `art` selbst über `art/datafile.rs` (`K3C_DATA_DIR`, sonst `data/`), nicht über `k3c-data`; so läuft RU7 unabhängig von RU3.
- Was erst RU5 oder RU8 liefern, kommt nach Leitfaden § 7 in `pending.rs` des Moduls bzw. als `#[ignore = "wartet auf RU8"]`/`"wartet auf RU5"`.
- Bildhälften: Zellen liegen übereinander (`view/cell.go`); gezeichnet wird je Zelle mit `Canvas::clip` in Fensterkoordinaten.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/art/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/demo/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/tests/fixtures/**`
- **Abhängig von:** RU4
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Alle Gruppen aus `public/grafik/<gruppe>/index.paper.json` mit Manifesten und Sheets sowie `data/paper.json` und `data/native.json` laden über die portierte API von `native/k3c-game/art`, ohne Änderung an den Dateien; die Tests des Pakets sind portiert; die Demo läuft über diese API.
- **AC-02** Das Texturbudget (Breite × Höhe × 4 je Sheet) wird je Level wie in Go berechnet und geloggt; schrittweises Laden (`LevelLoad::step`) und Freigeben nicht mehr gebrauchter Sheets beim Levelwechsel wie in Go.
- **AC-03** Kamera, Zellen (`UNIT_PX` = 32, zwei Bildhälften), Ebenen und Frame-Vertrag verhalten sich wie in `native/k3c-game/view`; die Tests des Pakets sind portiert (Sim-Teile mit RU7.5).
- **AC-04** Rig-Figuren gehen nur über `art` (Brücke `art/rigs` auf `k3c-rig` aus RU5); die Rig-Teile laden aus `native/k3c-rig/assets`; die Tests aus `rigs_test.go` sind portiert.

## Offene Fragen

- Soll `art` nach RU3 die Daten über `k3c-data` lesen statt über `art/datafile.rs`? Default: nein bis zu einem eigenen Ticket nach RU3.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU7.1 | `RU7.1-art-manifest-plan-sheet.md` | Umsetzung | autonom | fertig |
| RU7.2 | `RU7.2-art-library-font-outline.md` | Umsetzung | autonom | fertig |
| RU7.3 | `RU7.3-art-scales-seamap-voyage-worldimages.md` | Umsetzung | autonom | fertig |
| RU7.4 | `RU7.4-view-teil-1-ohne-sim.md` | Umsetzung | autonom | in Arbeit |
| RU7.5 | `RU7.5-view-teil-2-frame-vertrag.md` | Umsetzung | autonom | offen |
| RU7.6 | `RU7.6-art-rigs.md` | Umsetzung | autonom | offen |
| RU7.7 | `RU7.7-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
