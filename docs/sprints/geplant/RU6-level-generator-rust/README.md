# RU6 · SIM · Level-Generator in Rust, Level-Golden bitgleich

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

`engine/level` (2206 Zeilen, 2019 Zeilen Tests) erzeugt aus einem Seed ein Level; sechs Level-Golden und `nature.json` prüfen jedes Feld.

## Ziel

Der Level-Generator läuft in Rust und liefert in allen Level-Golden das gleiche Ergebnis wie Go (Entscheidung 012): gleiche Chunks und Objekte (Art, Anzahl, Reihenfolge), Fließkomma mit Toleranz 1e-6 relativ (absolut 1e-9 nahe 0). Bitgleich bleibt nur der rng. Die Rechenreihenfolge und die Datenstrukturen von Go sind keine Pflicht, idiomatisches, performantes Rust ist erwünscht; verbindlich bleiben die Reihenfolge der rng-Aufrufe und der Determinismus innerhalb von Rust. Am Ende sichtbar: `task rust:test -- -p k3c-level` grün mit allen Level-Golden.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 9 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`. Die Sessions laufen als Kette auf dem Branch `sprint/ru6` (eine nach der anderen, ein Commit je Session).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/level/` → `rust/crates/k3c-level/src/`; Dateinamen nach Leitfaden § 2. Die Reihenfolge ist gegenüber der ersten Fassung nach Abhängigkeiten sortiert (siehe Offene Fragen); `lair.go` (41, T) fehlte in der ersten Fassung und kommt in Session 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/level/`: backdrop (34, T), lines (77), planned (276, T), biome (172, T; fast ganz schon in `k3c-data`); neu in Rust: `rules.rs`, `testutil.rs`, `pending.rs` | 559 |
| 2 | `engine/level/`: nature (218, T), nature_check (173; großer Teil schon in `k3c-data`), lair (41, T) | 432 |
| 3 | `engine/level/`: planned_gen (135, T), planned_totals (88), guards (96, T), ruins (158, T); `level.rs` als Rumpf (`Gen`) | 477 |
| 4 | `engine/level/`: sites (275, T), validate (191) | 466 |
| 5 | `engine/level/`: level (364 ohne die Typen aus RU8, T) samt allen Golden-Tests | 364 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2298 Zeilen Go (die erste Fassung zählte 2206; Dateien sind seither gewachsen: level 364, lines 77, planned 276, planned_gen 135, nature_check 173). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Der Generator würfelt nur über `k3c-rng`; jede Würfel-Stelle bleibt an derselben Stelle der Reihenfolge, auch wenn das Ergebnis verworfen wird.
- Die Level-Golden vergleicht `engine/level/*_test.go` über `engine/internal/golden`; denselben Vergleich liefert `k3c_det::golden` (RU3).
- Daten, Biom-Prüfungen, `GuardRules`, `RuinRules`, `HubData` liegen seit RU3 in `k3c-data`; Gos Paket-Variablen werden in Rust zu `Rules` (`rules.rs`, RU6.1), siehe RU6.1 › Kontext.
- Tests, die erst mit `generate` laufen können, tragen bis RU6.5 `#[ignore = "wartet auf RU6.5"]`; RU6.5 entfernt sie alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Bitgleichheit der Level-Fließkommawerte zu Go (entfällt, Entscheidung 012).

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-level/**`
- **Abhängig von:** RU3, RU39
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `level-cave`, `-crystal`, `-forest`, `-ironhold`, `-mine`, `-planned` und `nature.json` werden von Rust mit gleichem Ergebnis erzeugt: gleiche Chunks und Objekte (Art, Anzahl, Reihenfolge), Fließkomma innerhalb 1e-6 relativ (absolut 1e-9 nahe 0), ohne Golden-Neuschrieb.
- **AC-02** Die Tests aus `engine/level` (19 Dateien, davon `bench_test.go` nur mit Benchmarks, daher 18 portierte Dateien) sind sinngleich portiert und grün; kein `#[ignore]` mit Marke `RU6` bleibt.
- **AC-03** Das Review nennt Sessions und Zeilen je Session (Plan: 450 Zeilen Code je Session); bei mehr als dem 1,5-Fachen des Plans empfiehlt es, vor RU10 neu zu schätzen.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Session-Reihenfolge nach Abhängigkeit statt nach Go-Dateiname:** Go-Reihenfolge der ersten Fassung (Session 2: `level.go` vor `sites.go`, `ruins.go`, `planned_gen.go`) hätte jede frühe Session mit Platzhaltern für `Generate` belassen. Neu: Geometrie und Plan-Prüfung zuerst, `level.go` zuletzt, dort laufen alle Golden. Die Zahl der Sessions bleibt 6.
- **`lair.go` (41, T)** fehlte in der Tabelle und kommt in Session 2.
- **Paket-Variablen von Go** (`guardRules`, `ruinRules`, `hubSites`, `sitePlacement`, `payGapUnits`) werden ein Struct `Rules` (aus `k3c_data::data::Data`), das `generate` als Parameter bekommt (wie RU8 mit `WorldValues`); kein globaler Zustand.
- **`bench_test.go`** (nur Benchmarks) wird nicht portiert; die Messung übernimmt RU39. AC-02 zählt 19 Dateien, 18 davon mit Tests.
- **Abhängigkeit von RU39** (Feld `Abhängig von`) bleibt wie in der ersten Fassung stehen; sie wird vom Koordinator geprüft.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU6.1 | `RU6.1-plan-geometrie-backdrop-lines-biome.md` | Umsetzung | autonom | offen |
| RU6.2 | `RU6.2-natur-nature-check-lair.md` | Umsetzung | autonom | offen |
| RU6.3 | `RU6.3-planung-gen-totals-guards-ruins.md` | Umsetzung | autonom | offen |
| RU6.4 | `RU6.4-sites-validate.md` | Umsetzung | autonom | offen |
| RU6.5 | `RU6.5-erzeugung-level-golden.md` | Umsetzung | autonom | offen |
| RU6.6 | `RU6.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
