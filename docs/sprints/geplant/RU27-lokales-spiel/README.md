# RU27 · CLI · Lokales Spiel: Sitzung, Kampagne, Spielstände, Einstellungen

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`local` (1233 Zeilen) bindet Raum, Spielstände und Einstellungen an das Spiel.

## Ziel

Das Rust-Spiel rechnet die Rust-Sim im selben Prozess und speichert. Am Ende sichtbar: portierte Tests aus `local` grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 15 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/local/` → `rust/crates/k3c-game/src/local/`; Dateinamen nach Leitfaden § 2. Die Tabelle weicht vom Entwurf ab (Grund unter Offene Fragen): Alle Dateien sind `impl Session`-Blöcke und brauchen den Typ aus `session.go` zuerst.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/local/`: session (352), settings (132, T), worlds (23, T), route (57, T) | 564 |
| 2 | `native/k3c-game/local/`: step (248, T), campaign (98, T) | 346 |
| 3 | `native/k3c-game/local/`: saves (136, T), actions (87, T), dev (100, T) | 323 |
| 4 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1233 Zeilen Go (unverändert). Die Tests (`local_test`, `load_test`, `outcome_test`, `world_test`, `settings_*_test`) zählen nicht in die Zeilen; Session 2 trägt mit rund 530 Zeilen Test den größten Teil. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- `local` besitzt den Raum aus `k3c-room` (RU21, RU24) im selben Prozess, ohne Thread je Raum; `step` rechnet einen Takt je Aufruf der Schleife (`game`, RU26). `Frame.clock` kommt aus der Zahl der Takte, nie aus der Wanduhr.
- **Ruckel-Risiko Takt:** Ob der Raum die Welt je Takt teilt oder kopiert und ob das Speichern den Takt hält (Go: Raum schreibt im Hintergrund), entscheidet über das Frame-Budget (4 ms je Takt, `data/native.json`). Beides steht in den Ergebnissen von RU27.1 und RU27.2; gemessen wird im Abnahme-Ticket (`step_ms` im Messbericht, RU30.4), nicht hier.
- `local` importiert kein Fachmodul und kein `game` (Importregel, Test in RU26.2).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Keine Rücksicht auf ältere Spielstände (Beschluss bis auf Widerruf): Es gilt das Format von Go.

- **Bereich:** `rust/crates/k3c-game/src/local/**`, `rust/crates/k3c-game/src/lib.rs` (nur `pub mod local;`), `rust/crates/k3c-game/Cargo.toml` (nur Abhängigkeiten auf vorhandene Workspace-Crates)
- **Abhängig von:** RU24
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Eine Sitzung mit zwei Spielern rechnet den Raum Schritt für Schritt.
- **AC-02** Spielstände und Einstellungen liegen an denselben Orten wie beim Go-Spiel.

## Offene Fragen

Keine offenen. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Schnitt geändert: `session.go` mit `settings`, `worlds`, `route` zuerst, danach `step` und `campaign`, zuletzt `saves`, `actions`, `dev` (Abhängigkeitsreihenfolge, alle unter 650 Zeilen Go); Platzhalter in `local/pending.rs` tragen die Methoden späterer Sessions, RU27.3 löscht die Datei.
- Die Wanduhr (Zeitstempel der Spielstände) nutzt `std::time::SystemTime` in einem kleinen eigenen Zeit-Typ; kein `chrono`. `local` importiert dazu nichts aus `devtools`.
- Kein `regex`: Der Spielstand-Name `^[a-z0-9-]{1,32}$` wird von Hand geprüft.
- Fehlt in `k3c-room` ein synchroner Takt oder ist das Speichern nur über einen Thread möglich, entscheidet die Session nach dem Ergebnis von RU24; Änderungen an `k3c-room` sind nicht Teil dieses Sprints (Ticket, Status `blockiert`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU27.1 | `RU27.1-session-settings-worlds-route.md` | Umsetzung | autonom | offen |
| RU27.2 | `RU27.2-step-campaign.md` | Umsetzung | autonom | offen |
| RU27.3 | `RU27.3-saves-actions-dev.md` | Umsetzung | autonom | offen |
| RU27.4 | `RU27.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
