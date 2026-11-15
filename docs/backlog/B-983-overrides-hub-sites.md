# B-983 · Eine Welt übersteuert die Plätze hub › sites und islandSites, der Level-Generator liest den Wertesatz der Welt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE5
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`hub.sites`, `hub.islandSites` und `hub.sitePlacement` sind nicht übersteuerbar (`docs/rules/welten.md` § 1.3, `engine/sim/world_values_locked.go`): Der Level-Generator liest sie aus der eingebetteten Basis (`engine/level/lines.go`, `ruins.go`, `sites.go`). Eine thematische Welt kann deshalb keine eigenen Plätze im Hub oder auf der Insel haben (Praxistest WE3).

## Ziel

Eine Welt legt ihre Hub- und Insel-Plätze selbst fest; Level und Simulation rechnen mit demselben Wertesatz.

## Beteiligte und Zielgruppen

Welten-Autoren (🧑, Claude über `world_override_set`), Spieler sehen je Welt andere Bauplätze. 🧑 gibt frei.

## Anforderungen

- `hub.sites` und `hub.islandSites` sind übersteuerbar; der Level-Generator bekommt die Plätze aus dem Wertesatz der Welt statt aus der Basis.
- Level-Erzeugung, Planungsprüfung (`level.ValidatePlanFor`) und Simulation nutzen denselben Satz; ein Test beweist das für eine Welt mit geänderten Plätzen.
- Prüfung beim Laden (fail closed): Plätze innerhalb der Hub-Breite, keine Überlappung, Pflichtplätze vorhanden; Meldung nach `welten.md` § 1.2.
- `hub.sitePlacement`, `economy.payRangeUnits` und `buildings.<id>.plantation.stripUnits` bleiben gesperrt.
- Standard-Welt unverändert, `testdata/golden/` bytegleich; deterministisch.
- `welten.md` § 1.3 nachgezogen (Tabelle der gesperrten Felder).

## Nicht-Ziele

`sitePlacement` entsperren, neue Platz-Arten, Editor-Oberfläche (B-987), Balancing der Plätze.

## Regeln und Einschränkungen

Sim-Code im eigenen Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). `engine/level` importiert weiterhin nichts aus `engine/sim` (Werte werden übergeben).

- **Bereich:** `engine/level/**`, `engine/sim/**`, `docs/rules/welten.md`
- **Abhängig von:** B-982
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Welt übersteuert einen Hub-Platz (Lage in Units) → Level und Simulation setzen den Bauplatz an die neue Stelle, Ruinen-Lage folgt. Welt ohne Override → Plätze der Basis.

## Ausnahme- und Fehlerfälle

Platz außerhalb der Hub-Breite oder zwei Plätze überlappen → Welt lädt nicht, Meldung mit Pfad. Override auf `hub.sitePlacement` → weiterhin `nicht übersteuerbar`. Geplante Chunks einer Insel passen nach der Änderung nicht mehr → Prüfung der Insel meldet es.

## Akzeptanzkriterien

- **AC-01** Test: Welt mit geändertem `hub.sites` → Level und Simulation zeigen denselben neuen Platz.
- **AC-02** Tests der Fehlerfälle (außerhalb, Überlappung, `sitePlacement` gesperrt).
- **AC-03** `testdata/golden/` unverändert.
- **AC-04** `docs/rules/welten.md` § 1.3 nachgezogen.

## Offene Fragen

- Umfang → Default: nur `sites` und `islandSites`, `sitePlacement` bleibt gesperrt (vorläufig, 🧑 bestätigt).
- Golden ändert sich wider Erwarten → Default: Session stoppt und legt ein Ticket an, statt `task go:golden` zu fahren (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Overrides für `hub › sites`). Entscheidung B-692 (sperren statt durchreichen) wird hier für zwei Felder aufgehoben.
