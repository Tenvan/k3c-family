# RU60 · INF, CLI, SIM · Feature-Parität im Rust-Spiel: fertige Go-Feature-Sprints prüfen, Lücken schließen, Sprints abschließen

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** INF, CLI, SIM
- **Reife:** bereit
- **Tickets:** B-940, B-941, B-942, B-953, B-962, B-964, B-965, B-974, B-977, B-1011, B-1012, B-981, B-982, B-983, B-984, B-1008
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19 (Ralf: ausdrücklich)

## Ausgangslage

Das Go-Spiel (`native/k3c-game`) wird in RU37 gelöscht (Beschluss Ralf 2026-11-19), die Go-Engine bleibt bis RU41. Die Sprints KS2 bis KS7, GS1, GS2, WE4, WE5, WI1 und W15 haben alle Sessions fertig und warten nur auf Review oder Abnahme am Go-Spiel; ihre Sim-Anteile hat der Port RU10 bis RU22 schon nach Rust übernommen (Go-Engine zuletzt geändert 2026-11-17, Port 2026-11-18). Ob das Rust-Spiel auch die Anzeige-Anteile hat, ist nicht belegt. B-1143 (Inventur) liefert die Zuordnung.

## Ziel

Für die fertigen Go-Feature-Sprints ist belegt oder nachgezogen, dass das Rust-Spiel dasselbe zeigt und rechnet; die Sprints sind abgeschlossen und stehen nicht mehr als aktiv in der Planung.

**Am Ende sichtbar:** `docs/messung/port-testabgleich.md` (Go-Tests ohne Rust-Gegenstück je Bereich), `plan_list {kind: sprint, status: aktiv}` ohne KS2 bis KS7, GS1, GS2, WE4, WE5, WI1, W15; `docs/messung/rust-abnahme.md` enthält die offenen Geräte-Abnahmen.

## Beteiligte und Zielgruppen

Second Worker (Lane zweit) arbeitet; 🧑 Ralf prüft die Geräte-Abnahmen am PC (RU36-Liste); Hauptrechner legt Tickets zu den Befunden an.

## Anforderungen

Siehe die Session-Dateien RU60.1 bis RU60.7. Tickets: B-940, B-941, B-942, B-953 (RU60.2), B-962, B-964, B-965, B-974, B-977, B-1011, B-1012 (RU60.3), B-981 bis B-984, B-1008 (RU60.4).

## Nicht-Ziele

Neue Mechaniken (Entscheidung 013: nur in Rust, eigene Sprints). Sim- und Balance-Läufe. Änderungen an der Go-Engine. Neue Grafik-Assets. Die Abnahme am Gerät selbst (🧑).

## Regeln und Einschränkungen

Der Second Worker arbeitet RU58, RU37, RU59 mit Vorrang vor diesem Sprint. Go-Vorlage nur aus dem Tag `go-referenz-final` (RU58.1). Port-Regeln: `docs/standards/rust-port.md`, Entscheidungen 012 und 013. Lücken über etwa 150 Zeilen werden Befund (kein Umsetzen).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `tools/portcheck/**`, `tools/Taskfile.yml`, `docs/messung/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-game/src/local/**`, `rust/crates/k3c-game/src/art/**`, `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/tests/still_*.rs`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-room/**`, `data/native.json`, `data/paper.json`, `docs/protocol.md`, `docs/sprints/**`, `docs/backlog/**`
- **Abhängig von:** RU58, RU52, RU55
- **Braucht 🧑:** Abnahme (Geräte-Abnahmen in der RU36-Liste, keine Abhängigkeit)
- **Sperre:** –

## Beispiele

RU60.2 findet, dass das Rust-Spiel das Horn auf dem Pfahl nicht zeichnet: Go-Datei `props/…` aus dem Tag lesen, Zeichnen in `rust/crates/k3c-game/src/props/` ergänzen, Standbild-Test dazu, B-941 mit Nachweis.

## Ausnahme- und Fehlerfälle

Lücke über 150 Zeilen: Befund im Ergebnis (`### Für den Hauptrechner`) mit Titel, Bereich, Größe; der Hauptrechner legt ein Ticket an. Tag `go-referenz-final` fehlt: Session blockiert, bis RU58.1 gemergt ist.

## Akzeptanzkriterien

- **AC-01** `docs/messung/port-testabgleich.md` nennt je Bereich Go-Tests, Rust-Gegenstücke und fehlende Namen und ist per Aufgabe reproduzierbar.
- **AC-02** Anzeige-Lücken zu KS2 bis KS7, WI1, W15 sind geschlossen oder als Befund benannt; B-940, B-941, B-942, B-953 mit Nachweis.
- **AC-03** Anzeige-Lücken zu GS1, GS2, WE4 sind geschlossen oder als Befund benannt; B-962, B-964, B-965, B-974, B-977, B-1011, B-1012 mit Nachweis.
- **AC-04** Sim-, Level- und Raum-Lücken zu WE5 und B-1008 sind geschlossen oder als Befund benannt; B-981 bis B-984, B-1008 mit Nachweis.
- **AC-05** KS2 bis KS7, WE4, WI1 sind abgeschlossen (Abnahme-Abschnitt gefüllt, Status erledigt).
- **AC-06** GS1, GS2, WE5, W15 sind abgeschlossen; offene Geräte-Abnahmen stehen in `docs/messung/rust-abnahme.md`.

## Offene Fragen

keine (alle mit Default entschieden, autonom, Claude, 2026-11-19: Parität wird geprüft statt neu geplant; was größer als 150 Zeilen ist, wird Befund).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU60.1 | `RU60.1-test-abgleich-go-rust.md` | Umsetzung | autonom | fertig |
| RU60.2 | `RU60.2-luecken-kampf-anzeige-welt.md` | Umsetzung | autonom | offen |
| RU60.3 | `RU60.3-luecken-geschichte-seekarte-fahrt.md` | Umsetzung | autonom | offen |
| RU60.4 | `RU60.4-luecken-sim-level-room.md` | Umsetzung | autonom | offen |
| RU60.5 | `RU60.5-abschluss-ks-we4-wi1.md` | Umsetzung | autonom | offen |
| RU60.6 | `RU60.6-abschluss-gs-we5-w15.md` | Umsetzung | autonom | offen |
| RU60.7 | `RU60.7-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
