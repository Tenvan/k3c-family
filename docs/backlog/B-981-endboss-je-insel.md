# B-981 · Jede Insel wählt ihren Endboss, der Bau liegt in ihrer tiefsten Stufe statt nur in der Kristallhöhle

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`engine/sim/island_data.go` › `checkIsland` verlangt in der tiefsten Stufe einer Insel einen Endboss (`endBoss(deepest)`), `data/bosses.json` › `endBosses` kennt nur `crystalHeartWarden` (Tiefe 4), und nur `data/biomes/crystal.json` hat einen Bau (`lair`). Folge (Praxistest WE3): Jede Insel muss auf der Kristallhöhle enden, egal welches Thema sie hat (`docs/rules/bosse.md` § 1, `welten.md` § 3).

## Ziel

Eine Insel endet in dem Biom, das zu ihrem Thema passt: Sie nennt ihren Endboss, und der Bau liegt in ihrer tiefsten Stufe, welches Biom das auch ist.

## Beteiligte und Zielgruppen

Spieler (2 Spieler, Split-Screen) kämpfen gegen den Endboss der Insel. Welten-Autoren (🧑, Claude über `world_*`) bauen thematische Inseln. 🧑 gibt die Spec frei (Spielregel).

## Anforderungen

- Insel-Feld `endBoss` (optional, ID aus `data/bosses.json` › `endBosses`). Ohne Feld gilt der Endboss, dessen `depth` die tiefste Stufe der Insel ist (heutiges Verhalten).
- `data/bosses.json` › `endBosses` bekommt je einen Endboss für die Tiefen 0 bis 3 (Basisart = Standardgegner der Stufe, Werte nach der Faustregel aus `bosse.md` § 1, nur vorhandene Fähigkeiten; Namen und Zahlen vorläufig, Balancing später).
- Der Bau liegt immer in der tiefsten Stufe der Insel. Die Bau-Werte (`triggerUnits`, `noticeUnits`, `escortUnits`) gelten für jedes Biom; der Level-Generator bekommt von der Insel gesagt, ob die Stufe den Bau trägt.
- Camp-Abstand zum Bau (`bosse.md` § 1, `engine/level/level.go` › `safeCampSlots`) gilt in jedem Biom als tiefster Stufe.
- Die Standard-Welt bleibt unverändert: Level und `testdata/golden/` sind bytegleich.
- Deterministisch (nur `engine/rng`), Verhalten mit 2 Spielern wie beim bisherigen Endboss.
- `docs/rules/welten.md` § 2, § 3 und `bosse.md` § 1 nennen Feld, Prüfung und Meldungen.

## Nicht-Ziele

Eigene Grafik der neuen Endbosse (das Spiel nimmt bis dahin das Asset der Basisart, B-993), neue Boss-Fähigkeiten, mehrere Endbosse je Insel, ein Biom mehrfach je Insel, Editor und MCP (B-987).

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Level in `rust/crates/k3c-level` (Entscheidung 013); die Go-Engine bekommt keine Änderung mehr. Keine Simulation, kein Balancing. Werte nur in `data/`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`, `data/bosses.json`, `data/biomes/**`, `docs/rules/welten.md`, `docs/rules/bosse.md`, `docs/rules/stufen.md`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Insel mit Tiefen 0, 1 ohne `endBoss` → Endboss der Tiefe 1, Bau in der Höhle. Insel mit Tiefen 0, 3 und `endBoss: crystalHeartWarden` → der Kristallherz-Wächter wartet im Bau des Eisenstollens. Standard-Welt (Tiefen 0–4) → unverändert.

## Ausnahme- und Fehlerfälle

`endBoss` nennt eine unbekannte ID → Welt lädt nicht, Meldung `<Insel>: endBoss "x" unbekannt (erlaubt: …)`. `endBoss` nennt einen Miniboss → abgelehnt. Biom zu kurz für Bau und Camp-Abstand → Prüfung lehnt die Insel mit Tiefe und Grund ab, nichts wird repariert. Spielstand mit besiegtem Endboss bleibt gültig (Sieg hängt an der Boss-ID).

## Akzeptanzkriterien

- **AC-01** Test in `engine/sim`: Insel mit tiefster Stufe 0, 1, 2 oder 3 lädt und hat ihren Endboss; `endBoss`-Feld wählt eine andere ID.
- **AC-02** Test in `engine/level`: Der Bau entsteht in der tiefsten Stufe jedes Bioms, nie in einer anderen; Camp-Abstand über 500 Seeds je Biom eingehalten.
- **AC-03** Tests der Fehlerfälle (unbekannte ID, Miniboss, zu kurzes Biom) mit Meldung.
- **AC-04** `testdata/golden/` unverändert (`go test ./engine/...` grün ohne `task go:golden`).
- **AC-05** `docs/rules/welten.md` und `bosse.md` nachgezogen.

## Offene Fragen

- Wahl des Endbosses → Default: Insel-Feld `endBoss`, sonst Endboss der tiefsten Stufe (vorläufig, 🧑 bestätigt).
- Neue Endbosse der Tiefen 0–3 → Default: Arbeitsnamen Wald-Häuptling, Höhlen-Koloss, Gruben-Fürst, Schmelz-Titan; 30× HP der Basisart, Phasen mit den vorhandenen Fähigkeiten (vorläufig, Balancing später, 🧑 bestätigt).
- Bau-Werte → Default: ein gemeinsamer Satz für alle Biome mit den Zahlen aus `crystal.json` (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Endboss je Insel-Thema).
