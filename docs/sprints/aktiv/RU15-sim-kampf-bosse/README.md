# RU15 · SIM · Sim: Kampf, Bosse, Krone, Portale

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** e14dd7f17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Kampf (607), Bosse (552), Krone (206) und Portale (172 Zeilen). Projekt KSY baut diese Regeln in Go gerade um.

## Ziel

Kampfregeln in Rust auf dem Stand, den KSY in Go hinterlässt. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 10 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`. Die Sessions laufen als Kette auf dem Branch `sprint/ru15` (eine nach der anderen, ein Commit je Session).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{combat,boss,crown,portal}/`; Dateinamen nach Leitfaden § 2. Dazu Test-Dateien ohne eigene Quelldatei, die in der ersten Fassung fehlten: `combat_gate`, `combat_review`, `combat_state` (Session 2), `crown_review` und die Hilfen `crown_helpers` (Session 4).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: boss (120, T), boss_abilities (97, T), boss_data (199; Lader schon in `k3c-data`, neu nur die Abfragen) | 416 |
| 2 | `engine/sim/`: boss_endboss (136, T), combat (302, T) | 438 |
| 3 | `engine/sim/`: combat_data (97), combat_squad (208) | 305 |
| 4 | `engine/sim/`: crown (206, T), portal (172, T) | 378 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1537 Zeilen Go (Stand der ersten Fassung; Session 1 prüft die aktuellen Zahlen und den KSY-Stand). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Projekt KSY ändert diese Dateien in Go. Vor dem Start `git log --oneline aac1bb539..HEAD -- engine/sim/combat*.go engine/sim/boss*.go engine/sim/crown.go engine/sim/portal.go` lesen (`aac1bb539` = Start-Commit von RU8; bis 2026-11-17 hat KSY die Dateien zuletzt mit KS7 ändern lassen) und den aktuellen Stand portieren; RU15.1 hält an und fragt 🧑, wenn KSY noch aktive Arbeit an diesen Dateien hat.
- Gos `w.island` heißt in Rust `IslandRef` (RU11.1); RU15.1 und RU15.2 ergänzen besiegte Bosse und Endboss-Kampf.
- Gos Paket-Variablen `bosses`/`endBosses` heißen in Rust `WorldValues.bosses` (RU15.1).
- Tests, die `CreateWorld`/`Step`/`CreateIsland` brauchen, tragen `#[ignore = "wartet auf RU14"]` (Insel: `RU10`); RU14 und RU22 lassen sie laufen, RU15.5 listet alle Marken.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

**Port-Orakel (Entscheidung 012):** Idiomatisches, performantes Rust ist erwünscht; Hinweise „wie Go“ und „Go-Reihenfolge“ in Sprint und Sessions gelten nicht für Rechenreihenfolge, `k3c-det`-Mathematik, lineare Suche und Go-Datenstrukturen. Verbindlich bleiben Reihenfolge der Ereignisse und der rng-Aufrufe, Ganzzahl-Werte und Determinismus innerhalb von Rust; Fließkomma gilt mit Toleranz 1e-4 relativ (`docs/standards/rust-port.md` § 1).

- **Bereich:** `rust/crates/k3c-sim/src/combat/**`, `rust/crates/k3c-sim/src/boss/**`, `rust/crates/k3c-sim/src/crown/**`, `rust/crates/k3c-sim/src/portal/**`, `rust/crates/k3c-sim/src/state/island_ref.rs` (neue Felder), `rust/crates/k3c-sim/src/state/shared.rs` (nur das Feld `bosses` in `WorldValues`), `rust/crates/k3c-sim/src/state/pending.rs` (Platzhalter löschen und anlegen), `rust/crates/k3c-sim/src/unit/escort.rs` (nur der Rumpf aus RU11.3), `rust/crates/k3c-sim/src/lib.rs` (nur `pub mod`-Zeilen)
- **Abhängig von:** RU11
- **Braucht 🧑:** KSY-Stand in Session 1 (Haltepunkt, nur wenn KSY noch aktive Sprints an `combat*.go`, `boss*.go`, `crown.go`, `portal.go` hat)
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Go-Tests zu `combat*.go` (samt `combat_gate`, `combat_review`, `combat_state`) sind portiert; die ohne `World`-Aufbau lauffähigen sind grün, jede Ignore-Marke nennt eine Sprint-Nummer.
- **AC-02** Die Go-Tests zu Bossen, Krone und Portalen (samt `crown_review`) sind portiert, gleiche Regel für Marken.

## Offene Fragen

KSY (Rang 2, sieben aktive Sprints) ändert genau diese Dateien. Entweder ist KSY vor diesem Sprint erledigt oder ruht bis nach der Umstellung (🧑, Ziel-Ticket › Offene Fragen). **Bleibt bei 🧑:** RU15.1 prüft den Stand und hält nur an, wenn KSY noch aktiv ist.

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Insel-Bezug:** wie RU11 (`IslandRef` als Parameter, kein Rückverweis); Endboss-Kampf hält `boss_id` und `home` (Index der Stufe) statt Zeiger.
- **Boss-Daten:** `bosses`/`endBosses` (Paket-Variablen in Go) werden das Feld `WorldValues.bosses`; die Lader liegen seit RU3 in `k3c-data`. Ein Eingriff in `state/shared.rs` (nur dieses Feld) ist nötig; RU14 füllt es beim Aufbau des Wertesatzes.
- **Test-Dateien ohne Quelldatei** (`combat_gate`, `combat_review`, `combat_state`, `crown_review`, Hilfen `crown_helpers`) kommen in die Session der getesteten Datei.
- **Tests ohne `World`-Aufbau** laufen mit `#[ignore = "wartet auf RU14"]` bzw. `RU10`; AC-01/AC-02 zählen sie als portiert.
- **Nicht in diesem Sprint** (Haltepunkt: Koordinator): `cause_test.go` (3 Tests, prüft Schadensursachen aus `common.go`, RU8) und `events_moon_stages_test.go` (5 Tests, `events_moon`, RU19) haben keine Zuordnung in der Sprint-Tabelle; Hinweis im Bericht an den Koordinator.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU15.1 | `RU15.1-boss-abilities-data.md` | Umsetzung | autonom | fertig |
| RU15.2 | `RU15.2-boss-endboss-combat.md` | Umsetzung | autonom | fertig |
| RU15.3 | `RU15.3-combat-data-squad.md` | Umsetzung | autonom | offen |
| RU15.4 | `RU15.4-crown-portal.md` | Umsetzung | autonom | offen |
| RU15.5 | `RU15.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
