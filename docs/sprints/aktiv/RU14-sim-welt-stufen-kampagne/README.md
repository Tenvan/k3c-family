# RU14 · SIM · Sim: Welt-Schritt, Kampagne, Schiff und Vorschau

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

`world*.go` (1396 Zeilen) rechnet den Schritt einer Stufe; dazu Kampagne, Schiff, Vorschau und Orte (rund 330 Zeilen).

## Ziel

Der Schritt einer Stufe und der Ablauf einer Kampagne laufen in Rust. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 11 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und das Zustandsmodell `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`, Session 4 `opus` (Formtabelle statt Reflection).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{world,campaign}/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru14` (eine nach der anderen, ein Commit je Session). Gegen das Repo geprüft (2026-11-17): `world.go` hat 236 statt 231 Zeilen, `world_values.go` 294 statt 289, `world_values_locked.go` 52 statt 51; `world_hub_sites.go` (67) und `world_skins.go` (76) fehlten in der Tabelle und sind aufgenommen; `events_values.go` (53) gehört zu RU19, `world_hub_sites` teilt sich auf RU14 (`LevelSites`, `checkWorldPlans`) und RU18 (`checkWorldHubSites`, `checkSiteBounds`, weil sie `HubFinding` aus `hub_validate` brauchen).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: campaign (117, T), preview (112, T), ship (76, T), world (236) | 541 |
| 2 | `engine/sim/`: world_cycle (122, T), world_lava (46, T), world_lines (108, T), world_state_mirror (71, T), world_skins (76, T) | 423 |
| 3 | `engine/sim/`: world_travel (74), world_values (294, T), world_values_locked (52) | 420 |
| 4 | `engine/sim/`: world_values_merge (267), world_values_room (88, T), world_vein (53, T), world_hub_sites (nur `LevelSites`, `checkWorldPlans`, 17) | 425 |
| 5 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1809 Zeilen Go. Tests ohne eigene Quelldatei (`harbor_test`, `world_cycle_night_test`, `world_values_b692_test`) wandern zur passenden Session. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

Ziel-Funktionen, die RU8 als Platzhalter in `state/pending.rs` führt und die in diesem Sprint entstehen: `values`, `level_site`, `gather_of`; die Sessions löschen sie.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Keine neue Abhängigkeit (kein `regex`, kein `chrono`), kein `HashMap` (Iterationsreihenfolge, Determinismus), keine Reflection-Nachbildung per Makro.

**Port-Orakel (Entscheidung 012):** Idiomatisches, performantes Rust ist erwünscht; Hinweise „wie Go“ und „Go-Reihenfolge“ in Sprint und Sessions gelten nicht für Rechenreihenfolge, `k3c-det`-Mathematik, lineare Suche und Go-Datenstrukturen. Verbindlich bleiben Reihenfolge der Ereignisse und der rng-Aufrufe (damit die Reihenfolge der Teilschritte, AC-01), Ganzzahl-Werte und Determinismus innerhalb von Rust; Fließkomma gilt mit Toleranz 1e-4 relativ (`docs/standards/rust-port.md` § 1).

- **Bereich:** `rust/crates/k3c-sim/src/world/**`, `rust/crates/k3c-sim/src/campaign/**`, `rust/crates/k3c-sim/src/lib.rs` (nur `pub mod`-Zeilen), `rust/crates/k3c-sim/src/state/pending.rs` (nur Platzhalter löschen), `rust/crates/k3c-sim/src/state/site_view.rs` (nur der Aufruf von `level_site`)
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Reihenfolge der Teilschritte ist dieselbe wie in Go; Teilschritte anderer Gruppen sind über feste Aufrufpunkte eingehängt.
- **AC-02** Die Go-Tests zu Kampagne, Schiff und Vorschau sind portiert und grün.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Paket-Variablen (`biomes`, `globalDayNight`, `dataFS`, `defaultValues`):** Es gibt keinen globalen Zustand in Rust; die Lösung von RU10 gilt, sonst `Arc<Vec<Biome>>` in `Options` und `Campaign`, `&Data` und `&Source` als Parameter (RU14.1, RU14.4).
- **Overrides ohne Reflection:** handgeschriebene Formtabelle (`values_shape.rs`) je übersteuerbarer Datei, abgesichert durch einen Test gegen die echten Basis-Dateien (RU14.4, Modell `opus`).
- **Overlay-Dateisystem:** ein Lese-Abschluss `read(name)` statt `fs.FS`; `k3c-data` bleibt unverändert (RU14.3).
- **`wallLookup`-Zwischenspeicher** entfällt (Zustandsmodell, Entscheidung 4): `level_site` sucht je Aufruf.
- **`World::values()`** legt in Rust nicht lazy den Basis-Wertesatz an; Test-Gerüste bauen ihn mit `testutil::world_with_defaults()`.
- **`world_hub_sites`:** `check_world_hub_sites` und `check_site_bounds` gehören zu RU18.3 (brauchen `HubFinding`); bis dahin Platzhalter, Tests mit `ignore = "wartet auf RU18"`.
- **`world_skins` ohne Regex:** die Pfadform wird von Hand geprüft (keine neue Abhängigkeit).
- **`Campaign::travel`** gibt die Tiefe der Zielwelt zurück statt `&World` (Borrow-Regeln), Verhalten gleich.
- **Abweichung vom Schnitt:** Die Dateien `world_skins` und `world_hub_sites` fehlten in der Tabelle; Session 2 und 4 nehmen sie auf.

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeiten gehören nicht in den bitgleichen Port. Sie entstehen nach dem Golden-Abgleich (RU22) als neuer Rust-Code, angefangener Go-Code wird neu geschrieben statt konvertiert. Spec steht im Ticket:

- SI1.2 Laderaum, Ladeplätze und Fracht in der Engine (B-967)
- GS3.1 Chronik abgeschlossener Inseln im Spielstand (B-966)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU14.1 | `RU14.1-world-campaign-ship-preview.md` | Umsetzung | autonom | fertig |
| RU14.2 | `RU14.2-world-cycle-lava-lines-mirror-skins.md` | Umsetzung | autonom | offen |
| RU14.3 | `RU14.3-world-travel-values-locked.md` | Umsetzung | autonom | offen |
| RU14.4 | `RU14.4-world-values-merge-room-vein.md` | Umsetzung | autonom | offen |
| RU14.5 | `RU14.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
