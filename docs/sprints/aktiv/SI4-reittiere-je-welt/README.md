# SI4 · REG, PAP, SIM, CLI · Reittiere je Welt: Regel, Assets, Engine, Anzeige

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** REG, PAP, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-970, B-1000
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), von 🧑 ausgewählt. Heute reitet jeder Monarch das Standard-Pferd (`data/monarch.json` › `mount`, `docs/rules/monarch.md` § 7); `MountOf` in `engine/sim/monarch_mount.go` liefert es ohne Feld je Spieler. In `public/grafik/reittiere/` liegen weitere Tiere, Regeln dafür fehlen.

## Ziel

Sengoku und Nil haben ein eigenes Reittier mit einer Stärke; es steht an einem Gatter auf der Insel, jeder Monarch kauft sein eigenes und behält es bis zum Ende der Welt. Regel, Assets, Engine und Anzeige liegen auf `main`.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions, Fachprüfung durch `k3c-reviewer` und `k3c-grafik`. Freigabe der Spec und Abnahme am TV: 🧑.

## Anforderungen

Siehe `B-970 › Anforderungen` (Regel und Engine, mit den Entscheidungen von 🧑 vom 2026-11-15) und `B-1000 › Anforderungen` (Assets und Anzeige). Die Auswahl im Welten-Editor ist B-1003 (Sprint SI7).

## Nicht-Ziele

Nachteile je Reittier, Reittier als reine Grafik, als Fracht oder als Belohnung nach dem Endgegner (B-970). Rentier und Polar-Welt. Sprint-Ausdauer. Welten-Editor (SI7). Simulation und Balancing (gesperrt).

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/monarch.md`, `docs/rules/welten.md`, `engine/sim/**`, `engine/room/**`, `data/monarch.json`, `data/worlds/sengoku.json`, `data/worlds/nil.json`, `public/grafik/reittiere/**`, `public/grafik/objekte/**`, `data/paper.json`, `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** spielstand
- Kette REG → PAP → SIM → CLI → Review; die Sessions laufen nacheinander auf `sprint/si4`.
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Das Reittier steht je Spieler im Spielstand; keine Rücksicht auf alte Spielstände. Kein Eingriff in `engine/level/**`.
- Das Format der Welt-Datei bekommt das optionale Feld `mount`. Werte vorläufig in `data/monarch.json`.

## Beispiele

Nil, Insel 1: Monarch 1 findet das Kamel am Gatter, wirft Münzen ein und steigt um. Monarch 2 bleibt auf dem Standard-Pferd. Auf Insel 2 reitet Monarch 1 weiter das Kamel.

## Ausnahme- und Fehlerfälle

Siehe `B-970 › Ausnahme- und Fehlerfälle` (zu wenig Münzen, schon gekauft, später Beitritt, unbekannte ID) und `B-1000 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Regeltext in `docs/rules/monarch.md` § 7 steht (B-970/AC-01).
- **AC-02** Die Assets Kamel und Gatter sind angelegt und geprüft, das Sengoku-Pferd ist zugeordnet (B-1000/AC-01).
- **AC-03** Katalog, Feld `mount` der Welt-Datei, Gatter, Kauf, Mitfahren und Spielstand sind in der Engine umgesetzt und getestet (B-970/AC-02 bis AC-09).
- **AC-04** Das Spiel zeichnet Gatter und je Monarch sein Reittier (B-1000/AC-02, AC-03, AC-05).
- **AC-05** Abnahme am TV durch 🧑 (B-1000/AC-04).
- **AC-06** Review: `check_run task:check` und `task:check:go` grün, schwere Befunde behoben, Abnahme-Ticket für Sim, Balance, Gen und Race angelegt (B-742).

## Offene Fragen

Keine blockierenden. Defaults in `B-970 › Offene Fragen` und `B-1000 › Offene Fragen` (vorläufig, 🧑 bestätigt), vor allem: Das Kamel ist im Dauerlauf schneller, weil der Sprint heute keine Ausdauer hat; Rentier erst mit der Polar-Welt; das Sengoku-Pferd nutzt das Bild `rappe`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI4.1 | `SI4.1-regeltext-reittiere.md` | Umsetzung | autonom | fertig |
| SI4.2 | `SI4.2-assets-reittiere.md` | Umsetzung | autonom | fertig |
| SI4.3 | `SI4.3-engine-reittiere.md` | Umsetzung | autonom | verworfen |
| SI4.4 | `SI4.4-anzeige-reittiere.md` | Umsetzung | autonom | verworfen |
| SI4.5 | `SI4.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
