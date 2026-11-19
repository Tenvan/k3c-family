# RU62 · SIM, CLI · Welt-Features SI1 bis SI6 neu in Rust (Sim und Anzeige, Entwurf)

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-967, B-997, B-968, B-998, B-969, B-999, B-970, B-1000, B-972, B-1001, B-973, B-1002
- **Start-Commit:** db360a93c
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Die Sprints SI1 bis SI6 (Fracht, Doppel-Truhe, Wetter je Welt, Reittiere, Einsiedler, Fest) haben ihre Sim- und Anzeige-Sessions mit dem Go-Stopp verworfen; Regeltext und Assets (SI1.1, SI1.3 usw.) stehen teils. Die Tickets blieben eingeplant im geschlossenen Sprint. B-1143 hat sie hier zusammengeführt und auf Rust umgehängt.

## Ziel

Die sechs Welt-Features laufen im Rust-Spiel: Sim in `rust/crates/k3c-sim`, Anzeige in `rust/crates/k3c-game`.

**Am Ende sichtbar:** Fracht an Deck, Doppel-Truhe mit Koop-Aktion, Taifun und Flut mit Vorboten, Welt-Reittiere, Einsiedler mit Angebot, Fest nach dem Sieg mit Musik.

## Beteiligte und Zielgruppen

Second Worker setzt um (Lane zweit); 🧑 Ralf gibt die Regeltexte frei und nimmt am PC ab; der Hauptrechner schreibt die Sessions vor dem Aktivieren (Reife Entwurf).

## Anforderungen

14 Sessions (alle Modell sonnet, Sim auf dem Hauptrechner, Anzeige und Review Lane `zweit`), je Welt-Feature eine Sim- und eine Anzeige-Session; die Sim-Sessions laufen nacheinander (gemeinsame Dateien, Spielstand-Versionen steigen in fester Reihenfolge):

- RU62.1 Sim Fracht (B-967, AC-01), RU62.2 Anzeige Fracht (B-997, AC-01).
- RU62.3 Sim Doppel-Truhe (B-968, AC-02), RU62.4 Anzeige Doppel-Truhe (B-998, AC-02).
- RU62.5 Sim Wetter-Kern und Taifun, RU62.6 Sim Flut (beide B-969, AC-03), RU62.7 Anzeige Wetter (B-999, AC-03).
- RU62.8 Sim Reittiere (B-970, AC-04), RU62.9 Anzeige Reittiere (B-1000, AC-04).
- RU62.10 Sim Einsiedler (B-972, AC-05), RU62.11 Anzeige Einsiedler (B-1001, AC-05).
- RU62.12 Sim Friedliche Insel und Fest-Zustand (B-973, AC-06), RU62.13 Anzeige Fest und Festmusik (B-1002, AC-06).
- RU62.14 Review und Abnahme-Ticket (alle Kriterien).

Regeltexte und Assets-Vorarbeiten aus SI1 bis SI6 (`docs/sprints/erledigt/`) werden gelesen und weiterverwendet; die Regeltexte liegen in `docs/rules/` (stufen, wirtschaft, koop, wetter, monarch, buerger, materialien-gebaeude, kampf § 10, bosse, zielkorridore), die Tickets sind von Ralf am 2026-11-15 freigegeben.

## Nicht-Ziele

Neue Mechaniken außer den genannten Tickets. Sim- und Balance-Läufe (gesperrt). Änderungen an der Go-Engine.

## Regeln und Einschränkungen

Neue Mechanik: Logik und Test in `rust/crates/k3c-sim`, das Spiel zeichnet nur (Entscheidung 013). Kein Go-Orakel: die SI-Engine-Sessions wurden verworfen, die Tests entstehen neu in Rust (Entscheidung 012).

**Daten in neuen Dateien:** `data/cargo.json`, `coop.json`, `weather.json`, `mounts.json`, `hermits.json`, `festival.json`; `data/ship.json` und `data/monarch.json` bleiben unverändert, ebenso die Welt-Dateien (`engine/sim/island_worlds.go` und `rust/crates/k3c-data/src/world.rs` lehnen unbekannte Felder ab, Go liest bis RU41). Die Zuordnung Welt auf Feature steht deshalb in den neuen Dateien (Feld `worlds`), die Felder `weather`, `mount`, `hermit` der Welt-Datei folgen nach RU41. Der Laderaum ist ein optionaler Abschnitt in `cargo.json`.

**Spielstand und Golden:** Sperre nur `spielstand`; `ISLAND_SAVE_VERSION` steigt in RU62.1, RU62.8 und RU62.10 je um 1 (Stand 9, keine Rücksicht auf alte Spielstände). Neue Mechaniken sind auf `standard` inaktiv, `rust/crates/k3c-sim/tests/golden_*.rs` und `testdata/golden/` bleiben unverändert grün, kein Regenerieren (Rust schreibt eigene Golden erst nach RU41). `data/worlds` hat nur `standard.json`: Tests brauchen Test-Welten und bauen sie im Code.

**Anzeige:** Bereiche eng halten (`props/plan*`, `sites*`, `buildings*`, `assets*`, `actors`, `fx`, `audio`, `world/tint|water|light|layers|river`); nicht angefasst werden `hud/`, `menu/`, `input/`, `devtools/`, `props/nature*`, `world/backdrop*`, `world/forest*` (RU61). Festmusik ohne neue Audiodatei, Wetter-Ton über vorhandene Slots. Assets nur über `assets_*`; Grafik-Golden, `gen:paper` und `check:race` laufen im Abnahme-Ticket (B-742). Keine Sim-, Balance- und Gen-Läufe.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `docs/rules/**`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/**`, `public/grafik/**`, `data/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Bleibt eine Session an einer Annahme hängen (Golden-Test rot wegen Versionswert, Feld im Spielstand oder Eingriff in `k3c-level` nötig, Festmusik nur mit neuer Datei möglich), hält sie an und meldet es; kein Regenerieren von Golden, kein Umgehen von Sperren.

## Akzeptanzkriterien

- **AC-01** B-967 und B-997 sind umgesetzt und belegt.
- **AC-02** B-968 und B-998 sind umgesetzt und belegt.
- **AC-03** B-969 und B-999 sind umgesetzt und belegt.
- **AC-04** B-970 und B-1000 sind umgesetzt und belegt.
- **AC-05** B-972 und B-1001 sind umgesetzt und belegt.
- **AC-06** B-973 und B-1002 sind umgesetzt und belegt.

## Offene Fragen

Keine. Freigegebene Defaults (autonom, Claude, 2026-11-19): (1) keine REG-Sessions, die Regeltexte liegen in `docs/rules/`; (2) kein Go-Orakel; (3) neue Daten in neuen Dateien, `ship.json` und `monarch.json` unverändert; (4) Sperre nur `spielstand`, Golden unverändert, neue Mechaniken auf `standard` inaktiv; (5) Anzeige-Bereiche eng, nichts aus RU61; (6) keine Abhängigkeit von RU37, RU60, B-961; (7) Festmusik ohne neue Audiodatei, Wetter-Ton über vorhandene Slots; (8) Spielstand-Version +1 in RU62.1, RU62.8, RU62.10. Folgerung aus (3) und (4): Zuordnung Welt auf Feature und Schalter stehen in den neuen Dateien und entfallen nach RU41 (Folge-Ticket der Review-Session). Manuelle Abnahmen am PC bleiben bei Ralf.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU62.1 | `RU62.1-sim-fracht.md` | Umsetzung | autonom | fertig |
| RU62.2 | `RU62.2-anzeige-fracht.md` | Umsetzung | autonom | offen |
| RU62.3 | `RU62.3-sim-doppel-truhe.md` | Umsetzung | autonom | fertig |
| RU62.4 | `RU62.4-anzeige-doppel-truhe.md` | Umsetzung | autonom | offen |
| RU62.5 | `RU62.5-sim-wetter-kern-taifun.md` | Umsetzung | autonom | fertig |
| RU62.6 | `RU62.6-sim-flut.md` | Umsetzung | autonom | offen |
| RU62.7 | `RU62.7-anzeige-wetter.md` | Umsetzung | autonom | offen |
| RU62.8 | `RU62.8-sim-reittiere.md` | Umsetzung | autonom | offen |
| RU62.9 | `RU62.9-anzeige-reittiere.md` | Umsetzung | autonom | offen |
| RU62.10 | `RU62.10-sim-einsiedler.md` | Umsetzung | autonom | offen |
| RU62.11 | `RU62.11-anzeige-einsiedler.md` | Umsetzung | autonom | offen |
| RU62.12 | `RU62.12-sim-friede-fest.md` | Umsetzung | autonom | offen |
| RU62.13 | `RU62.13-anzeige-fest-musik.md` | Umsetzung | autonom | offen |
| RU62.14 | `RU62.14-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
