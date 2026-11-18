# RU12 · CLI · Menüs Teil 2: Spielstände, Weltwahl, Pause, Test-Menüs

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

Nach RU9 fehlen Spielstand-Liste, Weltwahl, Pause, Pad-Test, Testing-Menü und Lizenzen (1092 Zeilen).

## Ziel

Alle Menüs des Go-Spiels gibt es im Rust-Spiel. Am Ende sichtbar: Spielstand-Liste, Weltwahl, Pause, Pad-Test und Testing-Menü, zunächst mit Platzhalter-Daten.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 10 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/menu/` → `rust/crates/k3c-game/src/menu/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf `sprint/ru12`, die erste hängt an RU9.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/menu/`: saves (194, T), worldpick (88, T), pause (157, T); neu: `placeholder.rs` | 439 |
| 2 | `native/k3c-game/menu/`: padtest (218, T) | 218 |
| 3 | `native/k3c-game/menu/`: testing (269, T), testing_text (46), stress_presets (62), licenses (58); räumt `pending.rs` und alle `ignore` aus RU9 und RU12 | 435 |
| 4 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1092 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle. Der Schnitt bleibt wie vorgegeben.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/menu/**`
- **Abhängig von:** RU9
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Spielstand-Liste und Weltwahl zeigen Daten über den vorhandenen Vertrag `view::Session` (`list`, `world_list`), den RU27 mit dem lokalen Spiel füllt; bis dahin liefert `menu::placeholder::PlaceholderSession` feste Einträge.
- **AC-02** Pause, Pad-Test und Testing-Menü laufen je Spieler wie im Go-Spiel; B und Esc gehen in jedem Menü dieses Sprints zurück (in der Pause: weiterspielen).
- **AC-03** Kein Test aus `native/k3c-game/menu` fehlt ohne Begründung.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeit entsteht direkt im Rust-Client. Spec steht im Ticket:

- GS3.5 Seekarte als Vorschau in der Spielstand-Auswahl (B-978)

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Die in AC-01 genannte Schnittstelle `menu::Sources` entfällt: `view::Session` ist sie schon (Liste, Welten, Plätze); neu ist nur der Platzhalter `menu::placeholder::PlaceholderSession`. AC-01 ist entsprechend umformuliert.
- Die Ortszeit der Spielstand-Zeile (Go: `Local().Format`) kommt als Versatz in Minuten aus dem Client (`Menus::utc_offset_min`, Standard 0), weil `std` keine Ortszeit kennt und `chrono`/`time` neue Crates wären (Freigabe 🧑, Haltepunkt in RU12.1).
- Der Pad-Test liest Pads über `trait PadSource` (Standard: keine); `InputPads` baut sie aus `k3c_platform::InputState` ohne Gerätename, SDL-Kennung und rohe Tasten (die `k3c-platform` nicht liefert); die Verdrahtung in der Schleife macht RU26.
- Den Stresstest startet `Menus::starter` (`Option<Box<dyn StressStarter>>`), weil `dyn Session` keinen Typ-Test erlaubt; eine Methode `Session::stress_starter()` wäre sauberer und ändert `view/` (außerhalb des Bereichs), Vorschlag für RU27.
- Die Lizenz-Liste wird wörtlich portiert (Go-Bibliotheken); die Liste des Rust-Spiels und die Hilfs-EXE `menu/licensefile` gehören in den Paket-Sprint RU35.
- `TestWorldPickTwoPlayersAndNoAllocs` verliert die Allokationsprüfung (Go-Werkzeug `testing.AllocsPerRun`), der Zwei-Spieler-Teil bleibt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU12.1 | `RU12.1-menu-saves-worldpick-pause.md` | Umsetzung | autonom | fertig |
| RU12.2 | `RU12.2-menu-padtest.md` | Umsetzung | autonom | fertig |
| RU12.3 | `RU12.3-menu-testing-licenses.md` | Umsetzung | autonom | fertig |
| RU12.4 | `RU12.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
