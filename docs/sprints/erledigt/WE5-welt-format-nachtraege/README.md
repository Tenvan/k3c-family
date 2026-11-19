# WE5 · SIM, DEV · Welt-Format-Nachträge: Endboss je Insel, Overrides für Events und Plätze, Skins, Welten-MCP

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** SIM, DEV
- **Reife:** bereit
- **Tickets:** B-979, B-981, B-982, B-983, B-984, B-985, B-986
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Praxistest Welten-MCP (WE3, B-959): Inseln müssen auf der Kristallhöhle enden, `events` und `hub › sites` sind nicht übersteuerbar, Gegner-Varianten je Welt gehen nicht, `world_set` braucht für eine neue Insel zwei Aufrufe, Ablehnungen nennen die erlaubten Dateien nicht. Sengoku nutzt die Etappen aus WE4 noch nicht.

## Ziel

Das Welt-Format deckt thematische Welten ab. Am Ende sichtbar: Tests in `engine/sim` und `engine/level` für Endboss je Insel, Event- und Platz-Overrides und `skins`; `world_get sengoku` zeigt die Etappen 1, 2, 2; ein `world_set` hängt eine Insel samt Hafen an.

## Beteiligte und Zielgruppen

Welten-Autoren (🧑, Claude über `world_*`), Spieler mittelbar. 🧑 gibt die Specs frei; B-985 und B-986 sind Klasse A.

## Anforderungen

Siehe `B-979 › Anforderungen`, `B-981 › Anforderungen`, `B-982 › Anforderungen`, `B-983 › Anforderungen`, `B-984 › Anforderungen`, `B-985 › Anforderungen`, `B-986 › Anforderungen`. Sprint-eigen: Die Standard-Welt und `testdata/golden/` bleiben unverändert.

## Nicht-Ziele

Anzeige im Spiel und Editor (WE6), Assets (WE7), neue Events und Zuordnung in den Welten (WE8), Balancing, Simulationen.

## Regeln und Einschränkungen

Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Sim, Level und Raum liegen seit RU10 bis RU22 in Rust; die Parität prüft RU60.4 (B-981 bis B-984), der Abschluss folgt in RU60.6. Die Pfade `engine/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `k3c-level`, `k3c-room` (Entscheidung 013).

Sim-Code lief im eigenen Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`); WE5.1 lief im Hauptcheckout über `world_*` (Weltdateien sind gitignored). Die SIM-Sessions WE5.2 bis WE5.5 liefen nacheinander (gleicher Bereich), WE5.1 (Lane `welt`) und WE5.6 (DEV, Lane `mcp`) parallel dazu. Code-Review vor dem Commit bei SIM (Agent `k3c-welten`, `k3c-reviewer`).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-room/**`, `data/bosses.json`, `data/biomes/**`, `data/worlds/sengoku.json`, `docs/rules/welten.md`, `docs/rules/bosse.md`, `docs/rules/stufen.md`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/mcpsrv/tools_world.go`, `tools/k3c-workbench/internal/mcpsrv/tools_world_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

Insel mit Tiefen 0, 1 lädt mit dem Endboss der Höhle. Welt mit `overrides › events › fullMoon › everyNights: 5` hat Vollmond jede 5. Nacht. Sengoku bietet nach Insel 1 zwei Ziele.

## Ausnahme- und Fehlerfälle

Eine SIM-Session müsste `testdata/golden/` neu schreiben → sie stoppt, legt ein Ticket an und meldet es (der Sprint hat keine Sperre `golden`). Details je Ticket.

## Akzeptanzkriterien

- **AC-01** Sengoku hat die Etappen 1, 2, 2 und einen Hafen auf Insel 3 (B-979/AC-01, AC-02).
- **AC-02** Endboss je Insel wählbar, Bau in der tiefsten Stufe jedes Bioms (B-981/AC-01 bis AC-05).
- **AC-03** `overrides › events` wirkt im Raum (B-982/AC-01 bis AC-04).
- **AC-04** `hub › sites` und `islandSites` übersteuerbar, Level und Simulation gleich (B-983/AC-01 bis AC-04).
- **AC-05** Welt-Feld `skins` und Name-Override (B-984/AC-01 bis AC-04).
- **AC-06** Ein `world_set` setzt Hafen und hängt eine Insel an (B-985/AC-01 bis AC-03).
- **AC-07** Ablehnung nennt erlaubte Dateien und gesperrte Felder (B-986/AC-01 bis AC-03).
- **AC-08** `testdata/golden/` unverändert, `task check:go` grün.

## Offene Fragen

Keine blockierende. Vorläufige Defaults stehen in den Tickets (Endboss-Feld und neue Endbosse B-981, Abstand 0 = aus B-982, Umfang der entsperrten Felder B-983, Ablage der Skins B-984); 🧑 bestätigt sie mit der Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WE5.1 | `WE5.1-sengoku-etappen.md` | Umsetzung | autonom | fertig |
| WE5.2 | `WE5.2-endboss-je-insel.md` | Umsetzung | autonom | fertig |
| WE5.3 | `WE5.3-overrides-events.md` | Umsetzung | autonom | fertig |
| WE5.4 | `WE5.4-overrides-hub-sites.md` | Umsetzung | autonom | fertig |
| WE5.5 | `WE5.5-welt-skins.md` | Umsetzung | autonom | fertig |
| WE5.6 | `WE5.6-welten-mcp-hafen-meldung.md` | Umsetzung | autonom | fertig |
| WE5.7 | `WE5.7-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Review WE5.7 (2026-11-15): AC-01 bis AC-08 erfüllt. RU60.6 (2026-11-19, Claude): Parität von Endboss je Insel, Event-Overrides, Hub-Sites und Skins im Rust-Spiel durch RU60.4 geprüft (Events-Override-Tests ergänzt). Offene Sichtprüfung und die Entscheidung zum Camp hinter dem Portal stehen als G-10 in `docs/messung/rust-abnahme.md`; Sprint abgeschlossen.
