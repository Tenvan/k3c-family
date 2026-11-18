# RU19 · SIM · Sim: Events, Geschichte, Dev-Eingriffe

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Events (344), Geschichte (378) und Dev-Eingriffe samt Cheats (392 Zeilen).

## Ziel

Events, Geschichten je Welt und Dev-Eingriffe laufen in Rust. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 12 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und das Zustandsmodell `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{events,story,dev}/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru19`. Gegen das Repo geprüft (2026-11-17): `events_moon.go` hat 143 statt 167 Zeilen; `events_values.go` (53) fehlte in der Tabelle, davon bleibt nur `World.night` zu portieren (der Rest steht in `k3c-data`); Tests ohne eigene Quelle (`dev_cheat`, `dev_stress`, `events_budget`, `events_island`, `events_location`, `events_moon_stages`) gehören zur passenden Session.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: dev (206, T), dev_combat (111, T), dev_spawn (75, T) | 392 |
| 2 | `engine/sim/`: events (138, T), events_merchant (39, T), events_moon (143, T), events_values (nur `night`, 3) | 323 |
| 3 | `engine/sim/`: story (74, T), story_check (198), story_points (106) | 378 |
| 4 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1093 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Keine neue Abhängigkeit (kein `regex`), kein `HashMap` (Iterationsreihenfolge, Determinismus).

**Port-Orakel (Entscheidung 012):** Idiomatisches, performantes Rust ist erwünscht; Hinweise „wie Go“ und „Go-Reihenfolge“ in Sprint und Sessions (zum Beispiel Schleifenreihenfolge in `dev_fill_stock`) gelten nicht für Rechenreihenfolge, `k3c-det`-Mathematik, lineare Suche und Go-Datenstrukturen. Verbindlich bleiben Reihenfolge der Ereignisse und der rng-Aufrufe, Ganzzahl-Werte und Determinismus innerhalb von Rust; Fließkomma gilt mit Toleranz 1e-4 relativ (`docs/standards/rust-port.md` § 1).

- **Bereich:** `rust/crates/k3c-sim/src/events/**`, `rust/crates/k3c-sim/src/story/**`, `rust/crates/k3c-sim/src/dev/**`, `rust/crates/k3c-sim/src/lib.rs` (nur `pub mod`-Zeilen), `rust/crates/k3c-sim/src/state/pending.rs`, `rust/crates/k3c-sim/src/world/pending.rs` (nur Platzhalter löschen und Aufrufstellen auf die echte Funktion umstellen)
- **Abhängig von:** RU14, RU15
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Go-Tests zu Events und Geschichte sind portiert und grün.
- **AC-02** Dev-Eingriffe ändern den Zustand wie in Go.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Globale Ereignis-Helfer:** `fullMoon()`, `bloodMoon()`, `merchantRaid()` lesen in Go die Basis, der Spielablauf nutzt nur `w.night(id)`; die Helfer entfallen in Rust.
- **Unicode in der Geschichten-Prüfung:** `char::is_alphabetic`/`is_uppercase` ersetzen `unicode.IsLetter`/`IsUpper`; ein Test mit den Zeichen der Fachtexte sichert das; weicht ein Fall aus `data/worlds` ab, entsteht ein Ticket (RU19.3).
- **Insel-Rückverweis:** `pure`, `event_stage`, `stage_played` lesen in Go `w.island`; in Rust bekommen sie die Insel oder das Flag als Parameter (RU19.1, RU19.2).
- **Methoden an `WorldDef`/`Island`:** werden freie Funktionen (Rust kennt keine Erweiterung fremder Typen); Namen in `snake_case` mit Typ-Präfix (`story_start`).
- **Abweichung vom Schnitt:** `events_values` aufgenommen (nur `night`).

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeiten gehören nicht in den bitgleichen Port. Sie entstehen nach dem Golden-Abgleich (RU22) als neuer Rust-Code, angefangener Go-Code wird neu geschrieben statt konvertiert. Spec steht im Ticket:

- SI3.2 Wetter-Katalog, Zeitplan, Taifun und Flut (B-969)
- SI6.3 Friedliche Insel und Fest-Zustand (B-973)
- GS4.2 Flaschenpost, Ziel, Markierung, Spielstand (B-971)
- WE8.2 Event raid (Überfall) (B-991)
- WE8.3 Events sandstorm und fog (B-992)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU19.1 | `RU19.1-dev-eingriffe.md` | Umsetzung | autonom | fertig |
| RU19.2 | `RU19.2-events-moon-merchant.md` | Umsetzung | autonom | fertig |
| RU19.3 | `RU19.3-story.md` | Umsetzung | autonom | offen |
| RU19.4 | `RU19.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
