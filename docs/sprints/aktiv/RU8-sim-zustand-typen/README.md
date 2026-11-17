# RU8 · SIM · Sim-Zustand in Rust: Zustandsmodell, Typen, Welt-Grundgerüst

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** aac1bb539
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Die Go-Sim hält die Welt als Geflecht aus Zeigern (`[]*Coin`, `*Player`), das jede Funktion frei ändert. Rust lässt das so nicht zu; der Zustand braucht Indizes statt Zeiger.

## Ziel

Ein Zustandsmodell für Rust (Indizes statt Zeiger) und alle Zustands-Typen, auf die jede Sim-Gruppe baut. Am Ende sichtbar: Ein Go-Spielstand lädt in Rust und wird bitgleich wieder geschrieben.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 4 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/state/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | Zustandsmodell nach dem Abschnitt „Vorentscheidungen“ in `docs/standards/rust-sim-zustand.md` festschreiben, Crate-Gerüst `lib.rs` und `state/mod.rs` | – |
| 2 | `engine/sim/`: types (394) | 394 |
| 3 | `engine/sim/`: common (230), data (307) | 537 |
| 4 | `engine/sim/`: action_view (39), node_mark (35, T), purse (31, T), site_view (28, T), torches (33, T) | 166 |
| 5 | `engine/sim/`: save (351), save_economy (145, T) | 496 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 1593 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Vorentscheidungen

Das Zustandsmodell steht fest; Session 1 schreibt es nach `docs/standards/rust-sim-zustand.md` und baut das Gerüst.

1. `World` besitzt alle Dinge direkt als `Vec<Player>`, `Vec<Coin>`, `Vec<Troop>` … in Entstehungs-Reihenfolge. Kein `Box`, `Rc`, `RefCell`, kein Lebenszeit-Parameter an Zustands-Typen.
2. Verweise zwischen Dingen sind IDs wie in Go (`WorkerID *int` → `worker_id: Option<i64>`). Nachschlagen über Funktionen wie `fn troop(&self, id: i64) -> Option<&Troop>` und `troop_mut`; lineare Suche wie in Go, kein Index-Speicher.
3. Go-Zeiger auf Einzelteile (`*Travel`, `*Merchant`, `*Crown`, `*Job`, `*float64`) werden `Option<T>`.
4. Rückverweise (`island *Island`, `hubW *World`) gibt es in Rust nicht. Funktionen, die die Insel brauchen, bekommen sie als Parameter.
5. Der Material-Vorrat `Stock` gehört in einer Insel allen Stufen (Go teilt den Zeiger). Rust: Die Insel besitzt ihn; vor dem Schritt einer Stufe tauscht sie ihn mit `std::mem::swap` in die Stufe und danach zurück. Solange eine Stufe rechnet, liest niemand den Vorrat der Insel. Braucht Go an einer Stelle beides zugleich: Ticket, nicht raten.
6. Unveränderliche Wertesätze (`vals *WorldValues`, `hubData *HubData`) werden `Arc<WorldValues>` und `Arc<HubData>`. Das ist die einzige erlaubte Teilung.
7. Zwei Dinge zugleich ändern: Schleifen über Indizes (`for i in 0..w.troops.len()`); bei verschachtelten Listen die innere Liste herausnehmen (`let mut coins = std::mem::take(&mut w.coins); … w.coins = coins;`). Die Reihenfolge der Änderungen bleibt die von Go.
8. Löschen wie in Go an derselben Stelle: `remove(i)` für `append(a[:i], a[i+1:]...)`, `retain` für das Filtern am Platz, `swap_remove` nur, wo Go tauscht.
9. `NextID` zählt wie in Go; IDs werden an denselben Stellen vergeben.
10. Felder mit `json:"-"` und kleingeschriebene Felder stehen im Zustand, aber nicht im JSON: `#[serde(skip)]` und `Default`.
11. `rng *rng.Rng` wird `rng: Rng` in `World`. Jeder Würfel-Aufruf bleibt an seiner Stelle der Reihenfolge.
12. `Events` wird wie in Go je Schritt geleert.

### Hinweise für die Umsetzung

- `World` hält alle Listen als `[]*T` in Entstehungs-Reihenfolge; Verweise zwischen Dingen laufen schon heute über IDs (`WorkerID`, `TowerID`, `BlockedPlayerID`). In Rust: `Vec<T>` in derselben Reihenfolge, Verweise als `Option<i64>`.
- `engine/internal/rules/map_range_test.go` verbietet Durchläufe über Maps in der Sim; in Rust gilt dasselbe (`BTreeMap` statt `HashMap`).
- Felder mit `json:"-"` und kleingeschriebene Felder gehören nicht ins JSON, aber in den Zustand (`#[serde(skip)]`).
- Der Zustand verweist auf Level-Typen (`level.Biome`, `level.Layout`). Diese Session legt sie als reine Daten in `rust/crates/k3c-level/src/types.rs` an (Quelle: die Typ-Definitionen in `engine/level`); den Generator liefert RU6.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-sim/src/state/**`, `rust/crates/k3c-sim/src/lib.rs`, `docs/standards/rust-sim-zustand.md`, `rust/crates/k3c-level/src/types.rs`
- **Abhängig von:** RU3
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `docs/standards/rust-sim-zustand.md` legt fest, wie Verweise, Reihenfolgen und Löschen im Zustand laufen; ein Beispiel je Fall.
- **AC-02** Alle Zustands-Typen der Go-Sim existieren in Rust mit denselben JSON-Namen.
- **AC-03** Listen behalten die Reihenfolge der Go-Sim; kein Durchlauf hängt von einer Hash-Reihenfolge ab.
- **AC-04** Ein Zustand aus einem Sim-Golden lädt und wird ergibt geschrieben denselben JSON-Baum.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU8.1 | `RU8.1-zustandsmodell-geruest.md` | Umsetzung | autonom | offen |
| RU8.2 | `RU8.2-types.md` | Umsetzung | autonom | offen |
| RU8.3 | `RU8.3-common-data.md` | Umsetzung | autonom | offen |
| RU8.4 | `RU8.4-views-purse-torches.md` | Umsetzung | autonom | offen |
| RU8.5 | `RU8.5-save.md` | Umsetzung | autonom | offen |
| RU8.6 | `RU8.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
