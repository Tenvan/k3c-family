# RU29 · CLI · Testing-, Debug- und Cheat-Menüs im Spiel

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

`devtools` (1271 Zeilen) liefert Testing-, Debug- und Cheat-Menüs.

## Ziel

Die Dev-Menüs laufen im Rust-Spiel. Am Ende sichtbar: Cheat-Menü öffnet mit Pad und Tastatur.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 12 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/devtools/` → `rust/crates/k3c-game/src/devtools/`; Dateinamen nach Leitfaden § 2. Die Tabelle weicht vom Entwurf ab (Grund unter Offene Fragen): `devtools.go` steht in Session 1 (der Typ `overlay` wird dort gebraucht), `drawcheats.go` in Session 3.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/devtools/`: devtools (78), cheatlist (104), cheatmenu (83), cheats (124, T) | 389 |
| 2 | `native/k3c-game/devtools/`: chronicle (138, T), chronicle_stage (149), report (80) | 367 |
| 3 | `native/k3c-game/devtools/`: debuggeo (99, T), drawcheats (69), drawdebug (69), help (90, T), values (188, T) | 515 |
| 4 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1271 Zeilen Go (unverändert gegenüber dem Entwurf). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Das Overlay (`view.Overlay` und `view.Drawer` in einem Typ) wird in Rust zu zwei Hüllen auf einem gemeinsamen Zustand (`Rc<RefCell<…>>`; im Client-Crate erlaubt, verboten nur in den Sim-Crates).
- Die Anmeldung im Spiel (`game::packages`, Eintrag `devtools`) ist bis RU29.3 ein Platzhalter aus RU26.2; RU29.3 tauscht ihn.
- Die Entwickler-Tasten (F-Tasten, `game/devkeys.go`) gehören zu RU30.3, nicht hierher.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-game/src/lib.rs` (nur `pub mod devtools;`), `rust/crates/k3c-game/src/game/parts.rs`, `rust/crates/k3c-game/src/game/pending.rs` und `rust/crates/k3c-game/src/game/mod.rs` (nur die Zeilen für den Eintrag `devtools`, RU29.3)
- **Abhängig von:** RU26
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Debug- und Cheat-Menü bieten dieselben Einträge wie im Go-Spiel.
- **AC-02** Die Paket-Tests sind portiert und grün.

## Offene Fragen

Keine offenen. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Schnitt geändert: `devtools.go` (Typ `overlay`) zu Session 1, `report.go` mit der Chronik in Session 2, `drawcheats.go` zu den übrigen Zeichen-Dateien in Session 3 (Abhängigkeitsreihenfolge); alle Sessions bleiben unter 650 Zeilen Go.
- Die Wanduhr der Chronik und des Berichts nutzt `std::time::SystemTime` in einem kleinen eigenen Zeit-Typ; kein `chrono` (keine neue Abhängigkeit).
- Die Anmeldung von `devtools` in `game::packages` tauscht RU29.3 (dazu steht `game/parts.rs`, `pending.rs`, `mod.rs` im Bereich), damit das Paket nach dem Sprint im Spiel angemeldet ist.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU29.1 | `RU29.1-devtools-cheats.md` | Umsetzung | autonom | fertig |
| RU29.2 | `RU29.2-devtools-chronicle.md` | Umsetzung | autonom | fertig |
| RU29.3 | `RU29.3-devtools-debug-help-values.md` | Umsetzung | autonom | fertig |
| RU29.4 | `RU29.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18: AC-01 und AC-02 erfüllt (Ergebnisse RU29.1 bis RU29.3; `cargo test -p k3c-game` 733 Lib-Tests plus `run_smoke` grün, `task check` und `task check:go` grün). Alle fünf Go-Tests portiert; Listen von Cheat-Menü und Hilfe gegen Go gelegt, gleich.
Behoben: Chronik allokierte je Bild den Phasen-Text (`changed_str`). Review (k3c-reviewer, k3c-hud): keine HOCH-Befunde.
Abweichung: `devtools/pending.rs` bleibt mit `dev_of` (None), weil `view::Session` keine Entwickler-Rolle bietet, Cheats wirken erst mit B-1111 (Sprint RU30); `game/packages.rs` meldet devtools an (statt parts/mod). Tickets: B-1110 (native.json, RU30), B-1111 (Dev-Rolle, RU30).
Abnahme-Ticket (Race, Gen, Prüfliste für Ralf, Lane zweit): B-1112.
