# RU47 · SIM · Abnahme-Stand Rust-Sim: ignorierte Tests und Go-Seite (Second Worker)

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-1095
- **Start-Commit:** 048733cfa
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

Die Abnahme-Tickets B-1081 (RU11), B-1084 (RU15), B-1086 (RU10) und B-1093 (RU14) warten auf RU18, RU19 und RU22. Ein Teil lässt sich heute belegen: Grünstand von `cargo test`, Zahl und Art der ignorierten Tests, Ergebnis der `--ignored`-Läufe für schon freischaltbare Marken (RU10, RU15, RU17) und die Go-Seite. `gen:paper` und `check:race` liefen nach RU14 bereits in RU45.1.

## Ziel

Nach dem Sprint liegt ein belegter Stand je Marke und je Abnahme-Ticket vor; der Hauptrechner weiß, welche `#[ignore]`-Marken sofort weg können und welche echten Befunde offen sind.

Am Ende sichtbar: Ergebnis-Abschnitte RU47.1 bis RU47.3 mit Zahlen, Notizen „Stand 2026-11-18 (RU47)“ in B-1081, B-1084, B-1086, B-1093, B-1095 erledigt.

## Beteiligte und Zielgruppen

Der Second Worker (Lane `zweit`) fährt die Läufe; der Hauptrechner legt Befund-Tickets an und schaltet Tests frei (nach RU18).

## Anforderungen

B-1095 › Anforderungen und Akzeptanzkriterien (AC-01 bis AC-05). Keine sprint-eigenen Anforderungen.

## Nicht-Ziele

Marken entfernen oder Tests reparieren (RU18 ändert dieselben Dateien), Balance-Läufe gegen Go, `gen:paper`, `check:race`, B-1074 Gate-Messung (Ralf: zum Schluss), alles, was von RU18, RU19, RU21, RU22 oder RU26 abhängt.

## Regeln und Einschränkungen

Alle Sessions `Lane: zweit`, Modell sonnet, nacheinander (RU47.1 → .2 → .3 → .4): die Läufe belasten den Rechner (B-284). Nur lesende Läufe, kein Code. Der Second Worker legt keine Tickets an. Vor dem Push `git merge origin/main`.

- **Bereich:** `docs/backlog/B-1081-*.md`, `docs/backlog/B-1084-*.md`, `docs/backlog/B-1086-*.md`, `docs/backlog/B-1093-*.md`, `docs/backlog/B-1095-*.md`
- **Abhängig von:** RU10, RU11, RU14, RU15, RU45
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`cargo test --release --workspace` meldet je Crate `passed/ignored`; Tabelle: `RU18 114`, `RU19 136`, `RU10 52` ….

## Ausnahme- und Fehlerfälle

Rote Tests: Name und Ausschnitt im Ergebnis, nichts beheben. Hat sich main seit der Planung geändert (RU18 gemergt), weichen die Zahlen ab: aktuelle Zahlen zählen, Abweichung zu den Angaben des Tickets erklären.

## Akzeptanzkriterien

- **AC-01** `cargo test --release --workspace`: Zahlen je Crate, 0 rot oder jeder rote Test benannt (`B-1095/AC-01`).
- **AC-02** Tabelle der ignorierten Tests nach Marke mit Abgleich zu B-1081 und B-1093 (`B-1095/AC-02`).
- **AC-03** `--ignored`-Läufe für `k3c-sim` und `k3c-game`, Ergebnis je Test mit Marke RU10, RU15, RU17 gruppiert nach Ursache (`B-1095/AC-03`).
- **AC-04** `go:test` grün, `testdata/` unverändert (`B-1095/AC-04`).
- **AC-05** Notizen der vier Abnahme-Tickets und von B-1095 tragen den Stand; Tickets bleiben offen (`B-1095/AC-05`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU47.1 | `RU47.1-cargo-stand-und-marken.md` | Umsetzung | autonom | fertig |
| RU47.2 | `RU47.2-go-seite-sim-tests.md` | Umsetzung | autonom | fertig |
| RU47.3 | `RU47.3-notizen-in-abnahme-tickets.md` | Umsetzung | autonom | fertig |
| RU47.4 | `RU47.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18 (Second Worker, autonom): AC-01 bis AC-05 erfüllt, Nachweis in den Ergebnissen RU47.1 bis RU47.4. Rust-Workspace 1381 grün / 326 ignoriert / 4 rot (nur `still*`), `go:test` grün und `testdata/` unverändert, 39 grüne ignorierte Tests als Kandidaten zum Freischalten, Notizen in B-1081, B-1084, B-1086, B-1093 geschrieben (Status `offen`). `task check` grün (1953). Vorschläge und Befunde für den Hauptrechner im Ergebnis RU47.1 und RU47.4.
