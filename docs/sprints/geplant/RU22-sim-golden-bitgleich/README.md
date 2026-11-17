# RU22 · SIM · Sim-Abgleich: alle Sim- und Kampagnen-Golden bitgleich (Gate G3)

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

Nach RU19 sind alle Sim-Gruppen portiert und einzeln getestet. 13 Sim-Golden und 2 Kampagnen-Golden prüfen den Ablauf als Ganzes.

## Ziel

Die Rust-Sim erzeugt jedes Sim- und Kampagnen-Golden bitgleich. Am Ende sichtbar: Bericht für Gate G3 mit Liste der Golden und gefundenen Abweichungen.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 13 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`, Werkzeug (Session 1) und Bericht (Session 5) `opus`.

### Sessions

1. Abgleich-Werkzeug: Golden-Lauf in Rust, erster abweichender Snapshot und Pfad, Go-Spur je Schritt (AC-01)
2. Golden Wald (7 Dateien: ohne-spieler, tag, nacht, aufbau, raub, sturm, monarch) (AC-02)
3. Golden Höhle (3), Mine, Kristall, Ironhold (6 Dateien) (AC-02)
4. Kampagnen-Golden (2 Dateien: abstieg, ausbau; Spielstand, Laden, Reise) (AC-02)
5. Durchsatz Rust gegen Go, Platzhalter-Test, G3-Bericht (AC-03)
6. Review mit Entscheidung G3 durch 🧑

Gezählt 2026-11-17: `testdata/golden/` hat 23 Dateien, davon 13 `sim-*.json.gz` (7 Wald, 3 Höhle, je 1 Mine, Kristall, Ironhold) und 2 `campaign-*.json.gz`; die übrigen 8 (`level-*`, `nature`, `rng`) prüfen frühere Sprints.

### Vorentscheidungen

- Der Abgleich läuft als Test je Golden: Eingaben und Zwischenstände aus dem Golden lesen, die Rust-Sim rechnen lassen, JSON-Bäume mit `k3c_det::golden::diff` vergleichen. Tests unter `rust/crates/k3c-sim/tests/` (`common/`, `golden_abgleich.rs`, `golden_forest.rs`, `golden_biome.rs`, `golden_campaign.rs`, `golden_durchsatz.rs`, `kein_platzhalter.rs`).
- Für die Fehlersuche schreibt die Go-Testdatei `engine/sim/golden_trace_test.go` (Schalter `-trace`, sonst übersprungen) je Schritt den Zustand als JSON-Zeile nach `.work/trace/sim-<name>.jsonl`; das Rust-Werkzeug nennt den ersten Schritt und den ersten Pfad, die abweichen. Die Spur-Dateien werden nicht eingecheckt. Die Testdatei fällt unter B-925 (eigener Worktree, Agent `k3c-sim-dev`).
- Reihenfolge der Golden: zuerst `sim-forest-ohne-spieler` (kein Spieler, wenig Regeln), dann `sim-forest-tag`, `-nacht`, `-aufbau`, danach die übrigen Wald-Golden, dann die Biome, zuletzt die Kampagnen.
- Wie Go stellt jeder Lauf den Beutel auf 100 Gold (`goldenPurse`), nur im Test.
- Am Ende darf kein `unimplemented!`, `todo!` oder `pending.rs` mehr in `k3c-sim` stehen (Test, der die Quelltexte liest).
- Ein Golden, der in seiner Session nicht gleich wird, bekommt `#[ignore = "B-NNNN: …"]` mit Ticket im Sprint; zur Review darf keins mehr übrig sein.

### Hinweise für die Umsetzung

- Bei einer Abweichung zuerst den ersten abweichenden Schritt suchen (Zustand je Schritt aus Go schreiben lassen), dann das erste abweichende Feld. Typische Ursachen: Reihenfolge eines Würfel-Aufrufs, `fmax`/`fmin` mit NaN, Reihenfolge beim Löschen aus Listen, Sortierung mit Gleichstand.
- Ein Golden wird nie geändert. Zeigt die Suche einen Fehler in Go, entsteht ein Ticket; Rust bildet den Fehler nach.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Produktionscode (einzige Go-Datei: die Testdatei `engine/sim/golden_trace_test.go`). Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014 (fehlt deren Freigabe, hält RU22.1 an). Behebungen von Abweichungen dürfen jede Datei in `k3c-sim` und `k3c-level` ändern, aber keine Regel und keinen Wert. G3 entscheidet 🧑 (RU22.6 hält an).

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/src/**`, `engine/sim/golden_trace_test.go`, `docs/messung/rust-sim-abgleich.md`
- **Abhängig von:** RU18, RU19
- **Braucht 🧑:** Entscheidung
- **Sperre:** golden

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Ein Test nennt bei einer Abweichung den ersten Schritt und das erste Feld.
- **AC-02** Alle 13 `sim-*.json.gz` und beide `campaign-*.json.gz` stimmen bitgleich (gleicher JSON-Baum, jede Zahl mit denselben Bits), ohne Golden-Neuschrieb.
- **AC-03** Der G3-Bericht nennt Schritte je Sekunde Rust und Go für denselben Lauf und alle behobenen Abweichungen mit Ursache.

## Offene Fragen

Die Golden-Vergleiche sind Sim-Läufe als Tests; sie brauchen die Ausnahme von der Sim-Sperre (🧑, B-1014 › Offene Fragen 4; RU22.1 hält ohne sie an). Entscheidung G3 durch 🧑 in RU22.6.

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Die Go-Spur je Schritt entsteht in einer neuen Testdatei `engine/sim/golden_trace_test.go` (nicht im Produktionscode, nur mit `-trace`), weil `addPoolPoints` und `goldenWorld` paketintern sind; sie läuft nach B-925 im eigenen Worktree. Ohne diese Datei bliebe die Suche auf die Snapshot-Kadenz (30–300 Takte) beschränkt.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Bereich erweitert auf `rust/crates/k3c-sim/**` und `rust/crates/k3c-level/src/**`, weil Abweichungen in jedem Sim-Modul liegen können, nicht nur in `step.rs`.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Durchsatz an `sim-forest-aufbau` (6000 Takte, 2 Spieler) mit Go-Benchmark und Rust-Test mit `#[ignore]` im Release-Bau, ohne neue Crate.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU22.1 | `RU22.1-abgleich-werkzeug.md` | Umsetzung | autonom | offen |
| RU22.2 | `RU22.2-golden-wald.md` | Umsetzung | autonom | offen |
| RU22.3 | `RU22.3-golden-biome.md` | Umsetzung | autonom | offen |
| RU22.4 | `RU22.4-golden-kampagnen.md` | Umsetzung | autonom | offen |
| RU22.5 | `RU22.5-durchsatz-und-g3-bericht.md` | Umsetzung | autonom | offen |
| RU22.6 | `RU22.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
