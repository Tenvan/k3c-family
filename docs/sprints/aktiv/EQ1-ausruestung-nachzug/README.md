# EQ1 · SIM, CLI · Ausrüstung nachziehen: Rüstung abholen, Werte in Daten, Icons, Bogner oben, gemeinsames Limit, sichtbare Besorgung

- **Status:** aktiv
- **Projekt:** WIR
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-1205, B-1206, B-1207, B-1208, B-1209
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf-Entscheidung 2026-11-21 (Dialog), Claude trägt ein

## Ausgangslage

FG1 (Bürger, Meister-Gebäude, Kombinationen, Grafik) und ECO1 sind gemergt. Review und Dialog mit Ralf am 2026-11-21 ergaben sechs Nacharbeiten (B-1203 Offene Fragen).

## Ziel

Die Rüstung wird abgeholt, die Ausrüstungswerte stehen in den Daten, die Gegenstands-Icons sind im HUD sichtbar, der Bogner steht nur oben, das Truppen-Limit gilt für die Insel und Besorger laufen sichtbar.

## Beteiligte und Zielgruppen

Ralf entscheidet und nimmt am PC mit Pad ab; First Worker: Sim (EQ1.1, EQ1.4, EQ1.5), Second Worker: HUD, Werte und Review (EQ1.2, EQ1.3, EQ1.6).

## Anforderungen

Siehe B-1205 bis B-1209.

## Nicht-Ziele

Balancing der Zahlen; neue Berufe.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (Entscheidung 013); Go-Golden nur mit `native_rules` aus; deterministisch, 2 Spieler; Sim, Balance, Gen und `check:race` im Abnahme-Ticket (B-742). Golden schreiben nur EQ1.3 und EQ1.4 (Sperre je Ticket B-1206, B-1208).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`, `data/buildings.json`, `data/hub.json`, `engine/sim/data.go`, `testdata/rust/**`
- **Abhängig von:** FG1
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Soldat holt die Rüstung am Rüstungsschmied und wird Elite-Soldat.

## Ausnahme- und Fehlerfälle

Gegner in Sicht: niemand holt Ausrüstung.

## Akzeptanzkriterien

- **AC-01** (B-1205) Der Kämpfer holt die Rüstung am Gebäude ab.
- **AC-02** (B-1207) Regal und Holen-Hinweis zeigen die Gegenstands-Icons.
- **AC-03** (B-1206) Ausrüstungswerte stehen in `data/buildings.json`.
- **AC-04** (B-1208) Bogner nur oben, Limit gemeinsam für die Insel.
- **AC-05** (B-1209) Besorger laufen sichtbar über die Treppe.
- **AC-06** Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| EQ1.1 | `EQ1.1-ruestung-abholen.md` | Umsetzung | autonom | fertig |
| EQ1.2 | `EQ1.2-gegenstands-icons.md` | Umsetzung | autonom | fertig |
| EQ1.3 | `EQ1.3-werte-nach-data.md` | Umsetzung | autonom | fertig |
| EQ1.4 | `EQ1.4-bogner-oben-limit.md` | Umsetzung | autonom | fertig |
| EQ1.5 | `EQ1.5-besorger-weg.md` | Umsetzung | autonom | fertig |
| EQ1.6 | `EQ1.6-review.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
