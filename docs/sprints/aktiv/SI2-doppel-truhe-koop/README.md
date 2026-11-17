# SI2 · REG, PAP, SIM, CLI · Koop-Aktion Doppel-Truhe: Regel, Engine, Asset, Anzeige

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** REG, PAP, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-968, B-998
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), von 🧑 ausgewählt. Das Zusammenspiel der beiden Monarchen wird bisher nicht eigens belohnt. Truhen sind Pickups der Art `chest` (`engine/sim/economy.go`), ihr Gold steht in `data/economy.json` › `chestGold`, die Münz-Fontäne gibt es seit B-953 (`engine/sim/economy_fountain.go`, `native/k3c-game/props/coin_fountain.go`).

## Ziel

Die erste Koop-Aktion ist im Spiel: Die Doppel-Truhe öffnet sich nur, wenn beide Monarchen davorstehen, und wirft deutlich mehr Gold als Fontäne aus; Solo steht dort eine normale Truhe.

## Beteiligte und Zielgruppen

Spieler: zwei Monarchen auf der Couch (Solo verliert nichts). Umsetzung: Claude in autonomen Sessions, Fachprüfung durch `k3c-reviewer` und `k3c-grafik`. Freigabe der Spec und Abnahme am TV: 🧑.

## Anforderungen

Siehe `B-968 › Anforderungen` (Regel und Engine, mit den Entscheidungen von 🧑 vom 2026-11-15) und `B-998 › Anforderungen` (Asset und Anzeige).

## Nicht-Ziele

Gemeinsames Bauen, gemeinsamer Angriff, besondere Beute, Truppe als zweiter Schlüssel (B-968). Änderungen am Level-Generator. Simulation und Balancing (gesperrt).

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/koop.md`, `engine/sim/**`, `data/economy.json`, `public/grafik/objekte/**`, `data/paper.json`, `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** –
- Kette REG → PAP → SIM → CLI → Review; die Sessions laufen nacheinander auf `sprint/si2`. Das Asset steht vor der Engine, damit die Engine-Sessions der Sprints nicht alle gleichzeitig anstehen.
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Kein Eingriff in `engine/level/**`, kein neues Feld im Spielstand; sonst anhalten und melden.
- Koop-Werte nur über Multiplikator-Felder; Werte vorläufig in `data/economy.json`.

## Beispiele

Koop: Monarch 1 steht an der Doppel-Truhe, ein Schloss leuchtet; Monarch 2 kommt dazu → Truhe springt auf, Fontäne, beide sammeln. Solo: an derselben Stelle steht eine normale Truhe.

## Ausnahme- und Fehlerfälle

Siehe `B-968 › Ausnahme- und Fehlerfälle` (Spieler geht oder kommt, Monarch ohne Krone, verschiedene Stufen) und `B-998 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Regeltext `docs/rules/koop.md` steht (B-968/AC-01).
- **AC-02** Die Assets der Doppel-Truhe sind angelegt und geprüft (B-998/AC-01).
- **AC-03** Die Doppel-Truhe ist in der Engine umgesetzt und getestet, Solo unverändert (B-968/AC-02 bis AC-08).
- **AC-04** Das Spiel zeichnet Doppel-Truhe, leuchtendes Schloss und Fontäne (B-998/AC-02, AC-04).
- **AC-05** Abnahme am TV durch 🧑 (B-998/AC-03).
- **AC-06** Review: `check_run task:check` und `task:check:go` grün, schwere Befunde behoben, Abnahme-Ticket für Sim, Balance, Gen und Race angelegt (B-742).

## Offene Fragen

Keine blockierenden. Defaults in `B-968 › Offene Fragen` und `B-998 › Offene Fragen` (vorläufig, 🧑 bestätigt), vor allem: dreifaches Gold, im Mittel jede dritte Truhe, das Kennzeichen wirkt nur bei zwei gesteuerten Monarchen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI2.1 | `SI2.1-regeltext-koop.md` | Umsetzung | autonom | fertig |
| SI2.2 | `SI2.2-asset-doppel-truhe.md` | Umsetzung | autonom | fertig |
| SI2.3 | `SI2.3-engine-doppel-truhe.md` | Umsetzung | autonom | verworfen |
| SI2.4 | `SI2.4-anzeige-doppel-truhe.md` | Umsetzung | autonom | verworfen |
| SI2.5 | `SI2.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Review SI2.5 (2026-11-17): AC-01 und AC-02 erfüllt (Regeltext, Assets ohne schweren Befund). AC-03 und AC-04 offen: SI2.3/SI2.4 verworfen, Engine und Spiel unverändert. AC-05 (TV, 🧑) offen. AC-06: `task:check` und `task:check:go` grün nach Neuschreiben des Detvec-Baums (B-1030). Sim, Balance, Gen und Race im Abnahme-Ticket B-1032.
