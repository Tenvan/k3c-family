# RU28 · SIM · Rust-Balance-Crate Teil 2: Messungen und Auswertung

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

Nach RU25 fehlen Messungen, Kennzahlen, Gruppen und Auswertung.

## Ziel

Messungen und Auswertung laufen in Rust. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 23 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell: `sonnet`. Kette auf `sprint/ru28`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/balance/` → `rust/crates/k3c-balance/src/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Tests ohne eigene Quelle | Zeilen |
|---|---|---|---:|
| 1 | `engine/balance/`: metrics (315), metrics_island (109), economy_measures (100); Vergleich mit Go (`rustref_test.go`) | – | 524 |
| 2 | `engine/balance/`: evaluate (161), figures (136, T), figures_island (158), groups (51, T) | – | 506 |
| 3 | `engine/balance/`: summary (276, T), targets (296, T) | economy_test | 572 |
| 4 | `engine/balance/`: delta (102, T), ship_need (188, T), baselines (141, T) | – | 431 |
| 5 | `engine/balance/`: curves (141), sensitivity (173, T), variant (113, T), vary (158) | – | 585 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – | – |

Summe: 2618 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Vorentscheidungen

- Vergleich mit Go über den neuen Go-Test `engine/balance/rustref_test.go` (`-write`): Bericht einer kleinen Matrix (1 Seed, 1 Tag) und Zusammenfassung aus `testdata/rust/balance/insights-run7.json` als Dateien unter `testdata/rust/balance/`; Rust vergleicht mit `k3c_det::golden::diff`. Kurze Testläufe mit der Ausnahme aus B-1014, kein Balancing.
- Geänderte Daten im Speicher (`fstest.MapFS`) werden `k3c_data::Source::Mem` (RU28.5).
- Zeitstempel in UTC (std hat keine Zeitzone).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: die neue Testdatei `engine/balance/rustref_test.go`. Einzige Änderung außerhalb von `k3c-balance` in Rust: `Source::Mem` in `k3c-data` (RU28.5).

- **Bereich:** `rust/crates/k3c-balance/**`, `rust/crates/k3c-data/src/source.rs`, `engine/balance/rustref_test.go`, `testdata/rust/balance/**`
- **Abhängig von:** RU25
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Kennzahlen aus einem festen Lauf stimmen mit Go überein.
- **AC-02** Der Bericht hat dasselbe JSON-Format wie der Go-Bericht.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Vergleich mit Go** über `engine/balance/rustref_test.go` und Dateien unter `testdata/rust/balance/` (Bereich erweitert).
- **Daten im Speicher:** Variante `Source::Mem` in `k3c-data` (Bereich erweitert); fehlt in `k3c-sim` der Port von `sim.UseData`, hält RU28.5 an und legt ein Ticket an.
- **Zeit:** `Saved` der Baselines in UTC statt Ortszeit (keine Zeit-Crate).
- **Abweichung vom Schnitt:** `economy_test.go` (ohne eigene Quelle) in Session 3.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU28.1 | `RU28.1-balance-metrics-economy-measures.md` | Umsetzung | autonom | offen |
| RU28.2 | `RU28.2-balance-evaluate-figures-groups.md` | Umsetzung | autonom | offen |
| RU28.3 | `RU28.3-balance-summary-targets.md` | Umsetzung | autonom | offen |
| RU28.4 | `RU28.4-balance-delta-ship-need-baselines.md` | Umsetzung | autonom | offen |
| RU28.5 | `RU28.5-balance-curves-sensitivity-variant-vary.md` | Umsetzung | autonom | offen |
| RU28.6 | `RU28.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
