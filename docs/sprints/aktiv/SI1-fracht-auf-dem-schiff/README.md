# SI1 · REG, SIM, PAP, CLI · Fracht auf dem Schiff: Regel, Engine, Assets, Anzeige

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** REG, SIM, PAP, CLI
- **Reife:** bereit
- **Tickets:** B-967, B-997
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), von 🧑 ausgewählt. Heute beginnt die neue Insel nach dem Inselwechsel mit dem Startvorrat (`docs/rules/stufen.md` § 1); was mitfährt, wählt niemand. Das Schiff hat drei Abschnitte (`data/ship.json`, `engine/sim/ship.go`), der Wechsel läuft in `engine/sim/island_switch.go` und `island_travel.go`; Etappen und Zielwahl (B-961) sind in main.

## Ziel

Vor der Abfahrt beladen die Spieler am Steg das Schiff mit Truppen, Münzen und Material; die Fracht kommt auf der neuen Insel zum Startvorrat hinzu. Regel, Engine, Assets und Anzeige liegen auf `main`.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions, Fachprüfung durch `k3c-welten`, `k3c-grafik`, `k3c-hud`. Freigabe der Spec und Abnahme am TV: 🧑.

## Anforderungen

Siehe `B-967 › Anforderungen` (Regel und Engine, mit den Entscheidungen von 🧑 vom 2026-11-15) und `B-997 › Anforderungen` (Assets und Anzeige).

## Nicht-Ziele

Lade-Menü oder Overlay, automatische Beladung nach Profil, Wegfall des Startvorrats (B-967). Simulation und Balancing der Werte (gesperrt; Abnahme-Ticket des Sprints).

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/stufen.md`, `docs/rules/wirtschaft.md`, `engine/sim/**`, `engine/room/**`, `data/ship.json`, `public/grafik/objekte/**`, `public/grafik/gebaeude-paper/**`, `data/paper.json`, `native/k3c-game/**`
- **Abhängig von:** B-961
- **Braucht 🧑:** Freigabe
- **Sperre:** spielstand
- Kette REG → SIM → PAP → CLI → Review; die Sessions laufen nacheinander auf `sprint/si1`.
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Keine Simulation, kein Balancing; Werte vorläufig in `data/ship.json`. Keine Rücksicht auf alte Spielstände.
- Assets nur über `assets_*`. Die Abnahme am TV (AC-05) sammelt die Ralf-Queue; sie hält den Sprint nicht auf.

## Beispiele

Grundschiff mit 6 Plätzen: 3 Schützen, 2 Beutel Münzen, 1 Kiste Holz → Insel 2 beginnt mit Startvorrat plus dieser Fracht; an Deck sind die sechs Plätze belegt zu sehen.

## Ausnahme- und Fehlerfälle

Siehe `B-967 › Ausnahme- und Fehlerfälle` (voller Laderaum, Verlust des Laderaums, Abfahrt mit offenem Träger-Auftrag, letzte Insel) und `B-997 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Regeltext zu Laderaum und Fracht steht in `stufen.md` und `wirtschaft.md` (B-967/AC-01).
- **AC-02** Laderaum, Ladeplätze, Ankunft der Fracht und Spielstand sind in der Engine umgesetzt und getestet (B-967/AC-02 bis AC-09).
- **AC-03** Assets für Ladeplätze, Beutel, Kiste und Laderaum-Stufe sind angelegt und geprüft (B-997/AC-01).
- **AC-04** Das Spiel zeichnet Ladeplätze, Fracht an Deck und die Ablehn-Rückmeldung (B-997/AC-02, AC-03, AC-05).
- **AC-05** Abnahme am TV durch 🧑 (B-997/AC-04).
- **AC-06** Review: `check_run task:check` und `task:check:go` grün, schwere Befunde behoben, Abnahme-Ticket für Sim, Balance, Gen und Race angelegt (B-742).

## Offene Fragen

Keine blockierenden. Die Defaults stehen in `B-967 › Offene Fragen` und `B-997 › Offene Fragen` (vorläufig, 🧑 bestätigt), vor allem: 6 Plätze plus 4 durch den Laderaum, Beutel 20 Münzen, Kiste 50 Einheiten, Material-Ladeplatz lädt das Material mit dem größten Vorrat, Truppen kommen per Münzeinwurf an Bord.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI1.1 | `SI1.1-regeltext-fracht.md` | Umsetzung | autonom | fertig |
| SI1.2 | `SI1.2-engine-fracht.md` | Umsetzung | autonom | offen |
| SI1.3 | `SI1.3-assets-fracht.md` | Umsetzung | autonom | offen |
| SI1.4 | `SI1.4-anzeige-fracht.md` | Umsetzung | autonom | offen |
| SI1.5 | `SI1.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
