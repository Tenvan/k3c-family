# RU11 · SIM · Sim: Einheiten und Gegner

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`unit*.go` (1062 Zeilen) und `enemies*.go` (805 Zeilen) bewegen Truppen und Gegner.

## Ziel

Truppen und Gegner bewegen sich in Rust wie in Go. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 9 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`. Die Sessions laufen als Kette auf dem Branch `sprint/ru11` (eine nach der anderen, ein Commit je Session).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{unit,enemies}/`; Dateinamen nach Leitfaden § 2. Dazu Test-Dateien ohne eigene Quelldatei, die in der ersten Fassung fehlten: `enemies_behaviour`, `enemies_targets`, `difficulty_protect` (Session 1), `enemies_pools` (Session 2), `unit_shelter`, `disarm` (Session 3), `unit_fetch` (Session 4).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: enemies (327), enemies_equipment_taken (35, T), enemies_guard (116, T); neu in Rust: `state/island_ref.rs` (`IslandRef`) | 478 |
| 2 | `engine/sim/`: enemies_traits (156, T; Lader schon in `k3c-data`), enemies_waves (175), unit_archer (103) | 434 |
| 3 | `engine/sim/`: unit_common (317), unit_escort (35, T), unit_flee (28), unit_hammer (77, T) | 457 |
| 4 | `engine/sim/`: unit_professions (198, T), unit_warrior (304, T) | 502 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1871 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Die meisten Go-Tests dieser Gruppe bauen ihre Welt mit `quietWorld`/`newWorld`/`CreateIsland` und rechnen über `Step`; beides liefert erst RU14 (`world.go`, `campaign.go`). Solche Tests werden mitportiert und tragen `#[ignore = "wartet auf RU14"]` (bei Insel-Tests `RU10`); RU11.5 fasst alle Marken in einer Liste zusammen, RU14 und RU22 lassen die Tests laufen.
- Fremde Funktionen, die diese Dateien rufen (RU15, RU18, RU19, RU14, RU10), stehen als Platzhalter in `rust/crates/k3c-sim/src/state/pending.rs` (Leitfaden § 7); die Liste je liefernder Datei steht in RU11.1 › Kontext.
- Gos `w.island` heißt in Rust `IslandRef` und wird als Parameter durchgereicht (RU11.1 Vorentscheidung 1).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

**Port-Orakel (Entscheidung 012):** Idiomatisches, performantes Rust ist erwünscht; Hinweise „wie Go“ und „Go-Reihenfolge“ in Sprint und Sessions (zum Beispiel Löschen in `stepProjectiles`, Gleitkomma-Reihenfolge in `workFactor`/`craftSeconds`) gelten nicht für Rechenreihenfolge, `k3c-det`-Mathematik, lineare Suche und Go-Datenstrukturen. Verbindlich bleiben Reihenfolge der Ereignisse und der rng-Aufrufe, Ganzzahl-Werte und Determinismus innerhalb von Rust; Fließkomma gilt mit Toleranz 1e-4 relativ (`docs/standards/rust-port.md` § 1).

- **Bereich:** `rust/crates/k3c-sim/src/unit/**`, `rust/crates/k3c-sim/src/enemies/**`, `rust/crates/k3c-sim/src/state/island_ref.rs`, `rust/crates/k3c-sim/src/state/mod.rs` (nur `pub mod island_ref;` und `pub use`), `rust/crates/k3c-sim/src/state/pending.rs` (Platzhalter löschen und anlegen), `rust/crates/k3c-sim/src/lib.rs` (nur `pub mod`-Zeilen)
- **Abhängig von:** RU8, RU39
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Go-Tests zu `unit*.go` (samt `disarm`, `unit_fetch`, `unit_shelter`) sind portiert; die ohne `World`-Aufbau lauffähigen sind grün, jede Ignore-Marke nennt RU14, RU10 oder RU18.
- **AC-02** Die Go-Tests zu `enemies*.go` (samt `enemies_behaviour`, `enemies_pools`, `enemies_targets`, `difficulty_protect`) sind portiert, gleiche Regel für Marken; gewürfelt wird nur über `k3c-rng`.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Insel-Bezug der Stufen:** Gos `w.island` (Skalierung, Wellenfaktoren, Grad) heißt in Rust `IslandRef` (`state/island_ref.rs`) und wird als Parameter `isl: Option<&IslandRef>` bzw. `Option<&mut IslandRef>` durchgereicht; `None` ist `w.island == nil`. RU15 ergänzt besiegte Bosse und Endboss-Kampf, RU10 baut ihn vor dem Schritt einer Stufe aus der Insel (Vorentscheidung 4 des Zustandsmodells: kein Rückverweis). Folgt der Schnittstelle von RU14 (`step`) — der Koordinator gleicht sie ab.
- **Test-Dateien ohne Quelldatei** (`enemies_behaviour`, `enemies_targets`, `enemies_pools`, `difficulty_protect`, `disarm`, `unit_fetch`, `unit_shelter`) kommen in die Session der getesteten Datei.
- **Tests ohne `World`-Aufbau:** Sie laufen mit `#[ignore = "wartet auf RU14"]`, bis `create_world`/`step` existieren; AC-01 und AC-02 zählen sie als portiert, grün werden sie mit RU14 und RU22.
- **Abhängigkeit von RU39** (Feld `Abhängig von`) bleibt wie in der ersten Fassung stehen; sie wird vom Koordinator geprüft.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU11.1 | `RU11.1-enemies-guard-equipment-inselbezug.md` | Umsetzung | autonom | offen |
| RU11.2 | `RU11.2-enemies-traits-waves-archer.md` | Umsetzung | autonom | offen |
| RU11.3 | `RU11.3-unit-common-escort-flee-hammer.md` | Umsetzung | autonom | offen |
| RU11.4 | `RU11.4-unit-professions-warrior.md` | Umsetzung | autonom | offen |
| RU11.5 | `RU11.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
